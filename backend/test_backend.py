import toml
import subprocess
import psycopg2
import traceback
from pprint import pp as pprint
from glob import glob
from termcolor import colored

data = toml.load('config/pos_config.toml')
user = data['database']['user']
password = data['database']['pwd']
name = data['database']['db_name']

def start_docker():
    command = f"""docker run --rm -d \
        -p 5432:5432 \
        -e POSTGRES_USER={user} \
        -e POSTGRES_PASSWORD={password} \
        -e POSTGRES_DB={name} \
        postgres:14
        """

    print('Starting docker ... ', end='', flush=True)
    ps = subprocess.run(command, shell=True, capture_output=True)
    print(colored('done', 'green'))

    return ps

def stop_docker(ps):
    command = f"""docker stop {ps.stdout.decode('utf-8')}"""
    print('Stopping docker ... ', end='', flush=True)
    subprocess.run(command, shell=True, capture_output=True)
    print(colored('done', 'green'))

def wait_for_database():
    print('Waiting for docker to start ... ', end='', flush=True)
    err = ''
    while True:
        try:
            conn = psycopg2.connect(
                user=user,
                password=password,
                host='localhost',
                port='5432',
                database=name
            )
            print(colored('done', 'green'))
            return conn
        except:
            if err != traceback.format_exc():
                print(colored('failed', 'red'))
                print(traceback.format_exc().rstrip())
                err = traceback.format_exc()
                print('Retrying ... ', end='', flush=True)

def load_schema(conn, cur):
    schema = open(glob('migrations/*_create_items/up.sql')[0], 'r').read()
    cur.execute(schema)
    conn.commit()

    schema = open(glob('migrations/*_create_purchases/up.sql')[0], 'r').read()
    schema += 'ALTER SEQUENCE purchases_id_seq RESTART WITH 1000;'
    cur.execute(schema)
    conn.commit()

def drop_tables(conn, cur):
    schema = open(glob('migrations/*_create_items/down.sql')[0], 'r').read()
    cur.execute(schema)
    conn.commit()

    schema = open(glob('migrations/*_create_purchases/down.sql')[0], 'r').read()
    cur.execute(schema)
    conn.commit()

def generate_test_data(conn, cur):
    def add_item(name, price, tax):
        cur.execute(f"""INSERT INTO items (name, price, tax) VALUES ('{name}', {price}, {tax})""")
        conn.commit()

    def add_purchase(id, items, total):
        cur.execute(f"""INSERT INTO purchases (id, items, total) VALUES ({id}, '{items}', {total})""")
        conn.commit()

    add_item('single glazed donut', 120, 1.00)
    add_item('half dozen glazed donuts', 625, 1.00)
    add_item('dozen glazed donuts', 1099, 1.00)

    add_purchase(100, '[{"name": "single glazed donut","price": 120,"quantity": 1}]', 120)
    add_purchase(101, '[{"name": "half dozen glazed donuts","price": 625,"quantity": 2}]', 1250)
    add_purchase(102, '[{"name": "half dozen glazed donuts","price": 625,"quantity": 1},{"name": "dozen glazed donuts","price": 1099,"quantity": 2}]', 2823)

def print_database(cur):
    cur.execute('SELECT * FROM items')
    pprint(cur.fetchall())

    cur.execute('SELECT * FROM purchases')
    pprint(cur.fetchall())

def run_rust_tests(conn, cur):
    did_fail = False

    print('Compiling (this might take a while) ... ', end='', flush=True)
    result = subprocess.run('cargo test --no-run --color=always', shell=True, capture_output=True)
    if result.returncode != 0: 
        print(colored('failed', 'red'))
        output = result.stderr.decode('utf-8')
        print(output)
        return True
    print(colored('done', 'green'))

    print('Fetching tests ... ', end='', flush=True)
    tests = subprocess.run('cargo test -- --list --format=terse', shell=True, capture_output=True).stdout.decode('utf-8').replace(': test', '').split('\n')[:-1]
    print(colored('done', 'green'))

    print(f'Running {len(tests)} rust tests:')
    for cnt, test in enumerate(tests):
        print(f'{cnt+1}. Running test: {test} ... ', end='', flush=True)

        drop_tables(conn, cur)
        load_schema(conn, cur)
        generate_test_data(conn, cur)

        out = subprocess.run(f'cargo test {test} -- --color=always', shell=True, capture_output=True).stdout.decode('utf-8').split('\n')
        x = [i for i in out if 'test result' in i][0]

        if 'FAILED' in x: 
            print('\n'.join(out))
            did_fail = True
        else: print(x)

    if did_fail: print(colored('done', 'red'))
    else: print(colored('done', 'green'))

    return did_fail

if __name__ == '__main__':
    ps = start_docker()
    conn = wait_for_database()
    cur = conn.cursor()

    result = run_rust_tests(conn, cur)

    stop_docker(ps)

    if result: exit(1)