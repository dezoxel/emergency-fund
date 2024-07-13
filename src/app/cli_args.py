import os
import argparse
from datetime import datetime

def parse_arguments():
    parser = argparse.ArgumentParser(description='Generate APY history for savings accounts.')
    parser.add_argument('--db-path', type=str, help='Path to the database file')
    parser.add_argument('--start-date', type=str, help='Start date in YYYY-MM-DD format')
    parser.add_argument('--end-date', type=str, help='End date in YYYY-MM-DD format')
    parser.add_argument('--apy-ranges', type=str, help='APY ranges in the format: account_type:min_apy:max_apy:mean_low:mean_high:std,...')
    parser.add_argument('--csv-path', type=str, help='Path to save the APY history CSV file')
    return parser.parse_args()

def get_database_path(args = None):
    if hasattr(args, 'db_path') and args.db_path:
        return args.db_path

    db_path = os.getenv('DB_PATH')
    if db_path:
        return db_path

    print("Database path not specified. Please provide the database path using --db-path argument or DB_PATH environment variable.")
    exit(1)

def parse_apy_ranges(apy_ranges_str):
    apy_ranges = {}
    for entry in apy_ranges_str.split(','):
        account_type, min_apy, max_apy, mean_low, mean_high, std = entry.split(':')
        apy_ranges[account_type] = (float(min_apy), float(max_apy), float(mean_low), float(mean_high), float(std))
    return apy_ranges

# TODO: don't hardcode here
date_format = '%Y-%m-%d'

def get_start_date(args = None):
    if hasattr(args, 'start_date') and args.start_date:
        return datetime.strptime(args.start_date, date_format)

    start_date = os.getenv('START_DATE')
    if start_date:
        return datetime.strptime(start_date, date_format)

    print("Start date is not specified. Please provide the start date using --start-date argument or START_DATE environment variable.")
    exit(1)

def get_end_date(args = None):
    if hasattr(args, 'end_date') and args.end_date:
        return datetime.strptime(args.end_date, date_format)

    end_date = os.getenv('END_DATE')
    if end_date:
        return datetime.strptime(end_date, date_format)

    print("End date is not specified. Please provide the end date using --end-date argument or END_DATE environment variable.")
    exit(1)

def get_apy_ranges(args = None):
    if hasattr(args, 'apy_ranges') and args.apy_ranges:
        return parse_apy_ranges(args.apy_ranges)

    apy_ranges = os.getenv('APY_RANGES')
    if apy_ranges:
        return parse_apy_ranges(apy_ranges)

    print("APY ranges are not specified. Please provide the APY ranges using --apy-ranges argument or APY_RANGES environment variable.")
    exit(1)

def get_apy_history_csv_path(args = None):
    if hasattr(args, 'apy_history_csv_path') and args.apy_history_csv_path:
        return args.apy_history_csv_path

    apy_history_csv_path = os.getenv('APY_HISTORY_CSV_PATH')
    if apy_history_csv_path:
        return apy_history_csv_path

    print("CSV path to APY history file is not specified. Please provide the CSV path using --csv-path argument or APY_HISTORY_CSV_PATH environment variable.")
    exit(1)
