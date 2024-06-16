import pandas as pd
import numpy as np
from datetime import datetime
import sqlite3
import os
import sys
import argparse
import logging

def fetch_savings_accounts(conn):
    query = """
    SELECT id, account_type
    FROM savings_accounts
    """
    df = pd.read_sql_query(query, conn).to_dict(orient='records')
    return df

def generate_apy(account_type, dates, apy_ranges):
    min_apy, max_apy, mean_low, mean_high, std = apy_ranges[account_type]
    apys = []
    for date in dates:
        mean = np.random.uniform(mean_low, mean_high)
        apy = np.clip(np.random.normal(mean, std), min_apy, max_apy)
        apys.append(apy)
    return apys

def generate_apy_history(accounts, date_range, apy_ranges):
    apy_history = []
    for account in accounts:
        apys = generate_apy(account['account_type'], date_range, apy_ranges)
        for date, apy in zip(date_range, apys):
            apy_history.append({
                "account_id": account['id'],
                "apy": apy,
                "effective_date": date
            })
    return apy_history

def save_apy_history_to_csv(apy_history, file_path):
    apy_history_df = pd.DataFrame(apy_history)
    apy_history_df.to_csv(file_path, index=False)

def parse_arguments():
    parser = argparse.ArgumentParser(description='Generate APY history for savings accounts.')
    parser.add_argument('--db-path', type=str, help='Path to the database file')
    parser.add_argument('--start-date', type=str, help='Start date in YYYY-MM-DD format')
    parser.add_argument('--end-date', type=str, help='End date in YYYY-MM-DD format')
    parser.add_argument('--apy-ranges', type=str, help='APY ranges in the format: account_type:min_apy:max_apy:mean_low:mean_high:std,...')
    parser.add_argument('--csv-path', type=str, help='Path to save the APY history CSV file')
    return parser.parse_args()

def get_database_path(args):
    if args.db_path:
        return args.db_path

    db_path = os.getenv('DB_PATH')
    if db_path:
        return db_path

    logging.error("Database path not specified. Please provide the database path using --db-path argument or DB_PATH environment variable.")
    sys.exit(1)

def parse_apy_ranges(apy_ranges_str):
    apy_ranges = {}
    for entry in apy_ranges_str.split(','):
        account_type, min_apy, max_apy, mean_low, mean_high, std = entry.split(':')
        apy_ranges[account_type] = (float(min_apy), float(max_apy), float(mean_low), float(mean_high), float(std))
    return apy_ranges

def get_start_date(args):
    if not args.start_date and not os.getenv('START_DATE'):
        logging.error("Start date is not specified. Please provide the start date using --start-date argument or START_DATE environment variable.")
        sys.exit(1)

    start_date = args.start_date if args.start_date else os.getenv('START_DATE')
    return datetime.strptime(start_date, '%Y-%m-%d')

def get_end_date(args):
    if not args.end_date and not os.getenv('END_DATE'):
        logging.error("End date is not specified. Please provide the end date using --end-date argument or END_DATE environment variable.")
        sys.exit(1)

    end_date = args.end_date if args.end_date else os.getenv('END_DATE')
    return datetime.strptime(end_date, '%Y-%m-%d')

def get_apy_ranges(args):
    if not args.apy_ranges and not os.getenv('APY_RANGES'):
        logging.error("APY ranges is not specified. Please provide the APY ranges using --apy-ranges argument or APY_RANGES environment variable.")
        sys.exit(1)

    apy_ranges_str = args.apy_ranges if args.apy_ranges else os.getenv('APY_RANGES')
    return parse_apy_ranges(apy_ranges_str)

def get_csv_path(args):
    if not args.apy_ranges and not os.getenv('CSV_PATH'):
        logging.error("CSV path is not specified. Please provide the CSV path using --csv-path argument or CSV_PATH environment variable.")
        sys.exit(1)

    csv_path = args.csv_path if args.csv_path else os.getenv('CSV_PATH')
    return csv_path

def main():
    logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')

    args = parse_arguments()
    db_path = get_database_path(args)
    start_date = get_start_date(args)
    end_date = get_end_date(args)
    apy_ranges = get_apy_ranges(args)
    csv_path = get_csv_path(args)
    
    date_range = pd.date_range(start_date, end_date, freq='MS')

    conn = sqlite3.connect(db_path)
    accounts = fetch_savings_accounts(conn)
    conn.close()

    apy_history = generate_apy_history(accounts, date_range, apy_ranges)
    save_apy_history_to_csv(apy_history, csv_path)
    logging.info("The APY history has been generated and saved to the CSV file.")

if __name__ == "__main__":
    main()
