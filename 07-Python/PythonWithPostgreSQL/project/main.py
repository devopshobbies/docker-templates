import psycopg2


def main():

    try:
        conn = psycopg2.connect(
            host="postgres",
            port="5432",
            database="postgres",
            user="postgresuser",
            password="postgrespassword"
        )
        print("Connection successful!")
    except psycopg2.Error as e:
        print("Unable to connect to the database:", e)
    finally:
        conn.close()


if __name__ == '__main__':
    main()