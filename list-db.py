import psycopg2

def get_connection():
    try:
        return psycopg2.connect(
            database="pos_db",
            user="pos_user",
            password="pos_password",
            host="127.0.0.1",
            port=5432,
        )
    except:
        return False

conn = get_connection()

if not conn: exit(1)

curr = conn.cursor()

curr.execute("SELECT * FROM purchase;")

data = curr.fetchall()

for row in data:
    print(row)

conn.close()
