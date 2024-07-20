import sqlite3

from libs.path import get_abs_path

def db_connect(db_path):
    return sqlite3.connect(get_abs_path('../..', db_path))

def db_disconnect(conn):
    if conn:
        conn.close()
    