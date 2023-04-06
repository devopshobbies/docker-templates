from flask import Flask, jsonify
from dotenv import load_dotenv

import subprocess
import psycopg2
import os


load_dotenv()


app = Flask(__name__)

@app.route('/')
def devops_hobbies():
    output = subprocess.check_output('helloworld-go').decode('utf-8')
    return output


@app.route('/db-status')
def db_status():
    db_host = os.getenv('DB_HOST')
    db_port = os.getenv('DB_PORT')
    db_name = os.getenv('POSTGRES_DB')
    db_user = os.getenv('POSTGRES_USER')
    db_password = os.getenv('POSTGRES_PASSWORD')

    try:
        conn = psycopg2.connect(
            host=db_host,
            port=db_port,
            dbname=db_name,
            user=db_user,
            password=db_password
        )
        conn.close()
        return jsonify({"status": "Ok"})

    except Exception as e:
        return jsonify({"status": "error", "message": str(e)})


if __name__ == '__main__':
    app.run(host="0.0.0.0", port="8088")