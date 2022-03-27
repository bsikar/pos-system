import toml
import subprocess
import psycopg2
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

    print('Starting docker ... ', end='')
    ps = subprocess.run(command, shell=True, capture_output=True)
    print(colored('done', 'green'))

    return ps

def stop_docker(ps):
    command = f"""docker stop {ps.stdout.decode('utf-8')}"""
    print('Stopping docker ... ', end='')
    subprocess.run(command, shell=True, capture_output=True)
    print(colored('done', 'green'))

def wait_for_database():
    print('Waiting for docker to start ... ', end='')
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
            pass

def load_schema(conn, cur):
    schema = open(glob('migrations/*_create_items/up.sql')[0], 'r').read()
    cur.execute(schema)
    conn.commit()

    schema = open(glob('migrations/*_create_purchases/up.sql')[0], 'r').read()
    schema += 'ALTER SEQUENCE purchases_id_seq RESTART WITH 1000;'
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
    cur.execute("""SELECT * FROM items""")
    pprint(cur.fetchall())

    cur.execute("""SELECT * FROM purchases""")
    pprint(cur.fetchall())

def run_rust_tests():
    print('Running rust tests:')
    subprocess.run('cargo test -- --test-threads=1 --nocapture --color=always', shell=True)
    print(colored('done', 'green'))

if __name__ == '__main__':
    ps = start_docker()
    conn = wait_for_database()
    cur = conn.cursor()

    load_schema(conn, cur)
    generate_test_data(conn, cur)

    print_database(cur)
    run_rust_tests()

    stop_docker(ps)
