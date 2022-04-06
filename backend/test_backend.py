import toml
import subprocess
import sqlite3
import traceback
import time
from pprint import pp as pprint
from glob import glob
from termcolor import colored

data = toml.load('config/pos_config.toml')

def start_database():
    print('Starting database ... ', end='', flush=True)
    pid = subprocess.Popen(['cargo', 'run'])
    print(colored('done', 'green'))
    return pid

def edit_pos_config():
    print('Moving `config/pos_config.toml` -> `config/.pos_config.toml.bk` ... ', end='', flush=True)
    subprocess.run('mv config/pos_config.toml config/.pos_config.toml.bk', shell=True)
    print(colored('done', 'green'))

    print('Editing `config/pos_config.toml` ... ', end='', flush=True)
    body = '[database]\nfile_path = "pos_testing_db.db"'
    with open('config/pos_config.toml', 'w') as f: f.write(body)
    print(colored('done', 'green'))

def restore_pos_config():
    print('Restoring `config/pos_config.toml` ... ', end='', flush=True)
    subprocess.run('mv config/.pos_config.toml.bk config/pos_config.toml', shell=True)
    print(colored('done', 'green'))

def load_schema(conn, cur):
    schema = open(glob('migrations/*_create_items/up.sql')[0], 'r').read()
    cur.executescript(schema)
    conn.commit()

    schema = open(glob('migrations/*_create_purchases/up.sql')[0], 'r').read()
    cur.executescript(schema)
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
        cur.execute(f"""INSERT INTO items (name, price, tax) VALUES (:name, :price, :tax)""", {'name': name, 'price': price, 'tax': tax})
        conn.commit()

    def add_purchase(id, items, total):
        cur.execute(f"""INSERT INTO purchases (id, items, total, ctime) VALUES (:id, :items, :total, :ctime)""", {'id': id, 'items': items, 'total': total, 'ctime': '2022-04-05 02:20:23.870212463'})
        conn.commit()

    add_item('single glazed donut', 120, 1.00)
    add_item('half dozen glazed donuts', 625, 1.00)
    add_item('dozen glazed donuts', 1099, 1.00)

    add_purchase(1, '[{"name": "single glazed donut","price": 120,"quantity": 1}]', 120)
    add_purchase(2, '[{"name": "half dozen glazed donuts","price": 625,"quantity": 2}]', 1250)
    add_purchase(3, '[{"name": "half dozen glazed donuts","price": 625,"quantity": 1},{"name": "dozen glazed donuts","price": 1099,"quantity": 2}]', 2823)

def run_rust_tests():
    conn = sqlite3.connect('pos_testing_db.db')
    cur = conn.cursor()
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
                if 'FAILED' in x:
                    print(colored('\n'.join(out), 'red'))
                    failed_tests.append(test)
                else: print(colored(x, 'green'))
                break
            except:
                if err != traceback.format_exc():
                    print(colored(f'failed: {traceback.format_exc().rstrip()}', 'red'));
                    err = traceback.format_exc()
                    print(colored('Retrying ... ', 'yellow'), end='', flush=True)

    if len(failed_tests) > 0:
        print(colored('done', 'red'))
        print(colored(f'{len(failed_tests)} tests failed:', 'red'))
        for test in failed_tests: print(colored(f'\t{test}', 'red'))
        return False
    else: print(colored('done', 'green'))
    return True

def clean_up(pid):
    print('Cleaning up ... ', end='', flush=True)
    pid.terminate()
    restore_pos_config()
    remove_db()
    print(colored('done', 'green'))

def remove_db():
    print('Removing `pos_testing_db.db` ... ', end='', flush=True)
    subprocess.run('rm pos_testing_db.db', shell=True)
    print(colored('done', 'green'))

if __name__ == '__main__':
    remove_db()
    edit_pos_config()

    pid = start_database()

    if not run_rust_tests():
        clean_up(pid)
        exit(1)

    clean_up(pid)