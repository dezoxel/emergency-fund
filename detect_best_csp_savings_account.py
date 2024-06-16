import sqlite3
import pandas as pd
from datetime import datetime, timedelta
import logging
import os
import argparse
import sys

def fetch_apy_last_year(conn):
    query = """
    SELECT account_id, apy
    FROM savings_accounts_apy_last_year
    """
    df = pd.read_sql_query(query, conn)
    return df

def calc_apy_last_year(df):
    df = df.copy()
    # Drop duplicates, keeping the latest effective_date for each account_id and month,
    # assuming the rows are sorted by effective_date
    df = df.drop_duplicates(subset=['account_id', 'month'], keep='last')
    
    # Filter to keep only the most recent 12 months per account_id
    df['rank'] = df.groupby('account_id')['month'].rank(method='first', ascending=False)
    df = df[df['rank'] <= 12]

    df['date'] = df['month'] + '-01'
    df = df[['account_id', 'apy', 'date']]
    
    return df

def update_apy_last_year(conn):
    current_date = datetime.now()
    one_year_ago = current_date - timedelta(days=365)

    query = """
    SELECT account_id, apy, 
           STRFTIME('%Y-%m', effective_date) AS month
    FROM savings_accounts_apy_history
    WHERE effective_date >= ?
    ORDER BY account_id, month, effective_date DESC
    """

    df = pd.read_sql_query(query, conn, params=[one_year_ago])

    df = calc_apy_last_year(df)

    conn.execute("DELETE FROM savings_accounts_apy_last_year")

    df.to_sql('savings_accounts_apy_last_year', conn, if_exists='append', index=False)

    conn.commit()

def calc_best_csp_savings_account(df):
    if df.empty:
        return None
    
    grouped = df.groupby('account_id')['apy'].agg(['mean', 'std']).reset_index()
    
    # Replace zero std with a small number to avoid division by zero
    grouped['std'] = grouped['std'].replace(0, 1e-10)
    
    grouped['csp'] = grouped['mean'] / grouped['std']
    
    best_account = grouped.loc[grouped['csp'].idxmax()]
    
    return best_account

def detect_and_print_best_csp_savings_account(conn):
    df = fetch_apy_last_year(conn)

    best_account = calc_best_csp_savings_account(df)

    if best_account is None:
        print("No data available to calculate CSP.")
    else:
        print(f"Best Account based on CSP:\nAccount ID: {best_account['account_id']}\nMean APY: {best_account['mean']:.4f}\nStandard Deviation of APY: {best_account['var']:.10f}\nCSP: {best_account['csp']:.4f}")

def get_database_path():
    parser = argparse.ArgumentParser(description='Specify the path to the database file.')
    parser.add_argument('--db-path', type=str, help='Path to the database file')
    args = parser.parse_args()

    if args.db_path:
        return args.db_path

    db_path = os.getenv('DB_PATH')
    if db_path:
        return db_path

    logging.error("Database path not specified. Please provide the database path using --db-path argument or DB_PATH environment variable.")
    sys.exit(1)

def main():
    logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')

    db_path = get_database_path()

    try:
        conn = sqlite3.connect(db_path)
        logging.info("Connected to the database successfully.")

        update_apy_last_year(conn)
        logging.info("Updated APY for the last year.")

        detect_and_print_best_csp_savings_account(conn)

    finally:
        if conn:
            conn.close()
            logging.info("Database connection closed.")

if __name__ == "__main__":
    main()
