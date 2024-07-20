import pandas as pd

def fetch_savings_accounts(conn):
    query = """
    SELECT id, account_type
    FROM savings_accounts
    """
    df = pd.read_sql_query(query, conn).to_dict(orient='records')
    return df