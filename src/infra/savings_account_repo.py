import pandas as pd

def fetch_savings_accounts(conn):
    query = """
    SELECT id, account_type
    FROM savings_accounts
    """
    df = pd.read_sql_query(query, conn).to_dict(orient='records')
    return df

def find_savings_account_by_id(conn, account_id):
    query = """
    SELECT sa.id, sa.account_name, sa.account_type, i.name as institution_name
    FROM savings_accounts sa
    JOIN institutions i ON i.id = sa.institution_id
    WHERE sa.id = ?
    LIMIT 1
    """
    results = pd.read_sql_query(query, conn, params=[account_id]).to_dict(orient='records')

    if len(results) == 0:
        raise Exception(f'Unable to find account name by ID. ID: {account_id}')

    savings_account = results[0]

    return savings_account
