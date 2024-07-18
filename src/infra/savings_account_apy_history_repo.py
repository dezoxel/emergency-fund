import pandas as pd
from datetime import timedelta, datetime
from sqlite3 import Connection

def fetch_apy_history_last_year(conn: Connection, current_date: datetime):
    # TODO: Handle leap years
    one_year_ago = current_date - timedelta(days=365)

    query = """
    SELECT account_id, sa.account_name, apy, 
           STRFTIME('%Y-%m', effective_date) AS month
    FROM savings_accounts_apy_history saah
    LEFT JOIN savings_accounts sa ON saah.account_id = sa.id
    WHERE effective_date >= ?
    ORDER BY account_id, month, effective_date DESC
    """

    df = pd.read_sql_query(query, conn, params=[one_year_ago])
    
    return df

def fetch_terms_history_last_year(conn: Connection, current_date: datetime):
    # TODO: Handle leap years
    one_year_ago = current_date - timedelta(days=365)

    query = """
    SELECT account_id, sa.account_name, apy, STRFTIME('%Y-%m-%d', effective_date) AS date, compound_frequency as n
    FROM savings_accounts_apy_history saah
    LEFT JOIN savings_accounts sa ON saah.account_id = sa.id
    WHERE effective_date >= ?
    ORDER BY account_id, date
    """

    df = pd.read_sql_query(query, conn, params=[one_year_ago])

    if df.empty:
       print('APY history is empty in DB.')
       return None

    df['apy'] = df['apy']/100

    return df
