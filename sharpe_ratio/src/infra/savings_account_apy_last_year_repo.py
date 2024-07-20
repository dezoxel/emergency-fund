import pandas as pd

def fetch_apy_last_year(conn):
    query = """
    SELECT account_id, account_name, apy
    FROM savings_accounts_apy_last_year saaly
    LEFT JOIN savings_accounts sa ON saaly.account_id = sa.id
    """
    df = pd.read_sql_query(query, conn)
    return df

def clear_apy_last_year(conn):
    conn.execute("DELETE FROM savings_accounts_apy_last_year")