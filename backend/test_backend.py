import toml
import subprocess
import psycopg2
import traceback
import time
from pprint import pp as pprint
from glob import glob
from termcolor import colored

data = toml.load('config/pos_config.toml')
user = data['database']['user']
password = data['database']['pwd']
name = data['database']['db_name']

def try_command(command):
    result = subprocess.run(command, shell=True, capture_output=True)
    if result.returncode != 0:
        print(colored(f'failed: {result.stderr.decode("utf-8")}', 'red'))
        exit(1)
    print(colored('\ndone', 'green'))
    return result.stdout.decode('utf-8')

def start_docker():
    command = f"""docker run --rm -d \
        -p 5432:5432 \
        -e POSTGRES_USER={user} \
        -e POSTGRES_PASSWORD={password} \
        -e POSTGRES_DB={name} \
        postgres:14
        """

    print('Starting docker ... ', end='', flush=True)

    return try_command(command)

def stop_docker(ps):
    command = f'docker stop {ps}'
    print('Stopping docker ... ', end='', flush=True)

    try_command(command)

def wait_for_database():
    print('Waiting for docker to start ... ', end='', flush=True)
    err = ''
    t_end = time.time() + 30
    while time.time() < t_end:
        try:
            conn = psycopg2.connect(
                user=user,
                password=password,
                host='localhost',
                port='5432',
                database=name
            )
            print(colored('\ndone', 'green'))
            return conn
        except:
            if err != traceback.format_exc():
                print(colored(f'failed: {traceback.format_exc().rstrip()}', 'red'));
                err = traceback.format_exc()
                print(colored(f'Retrying {int(round(t_end - time.time()))} seconds left ... ', 'yellow'), end='\r', flush=True)
    print(colored('\nfailed', 'red'))
    exit(1)

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
    failed_tests = []

    print('Compiling (this might take a while) ... ', end='', flush=True)
    result = subprocess.run('cargo test --no-run', shell=True, capture_output=True)
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

        x = y = err = ''
        while True:
            try:
                out = subprocess.run(f'cargo test {test} -- --exact', shell=True, capture_output=True).stdout.decode('utf-8').split('\n')
                x = [i for i in out if 'test result' in i][0]
                break
            except:
                if err != traceback.format_exc():
                    print(colored(f'failed: {traceback.format_exc().rstrip()}', 'red'));
                    err = traceback.format_exc()
                    print(colored('Retrying ... ', 'yellow'), end='', flush=True)

        if 'FAILED' in x: 
            print(colored('\n'.join(out), 'red'))
            failed_tests.append(test)
        else: print(colored(x, 'green'))

    if len(failed_tests) > 0: 
        print(colored('done', 'red'))
        exit(1)
    else: print(colored('done', 'green'))

if __name__ == '__main__':
    ps = start_docker()

    conn = wait_for_database()
    cur = conn.cursor()

    test_results = run_rust_tests(conn, cur)

    stop_docker(ps)
