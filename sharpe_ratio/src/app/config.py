import os
from datetime import datetime

def get_database_path():
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

def get_start_date():
    start_date = os.getenv('START_DATE')
    if start_date:
        return datetime.strptime(start_date, date_format)

    print("Start date is not specified. Please provide the start date using --start-date argument or START_DATE environment variable.")
    exit(1)

def get_end_date():
    end_date = os.getenv('END_DATE')
    if end_date:
        return datetime.strptime(end_date, date_format)

    print("End date is not specified. Please provide the end date using --end-date argument or END_DATE environment variable.")
    exit(1)

def get_apy_ranges():
    apy_ranges = os.getenv('APY_RANGES')
    if apy_ranges:
        return parse_apy_ranges(apy_ranges)

    print("APY ranges are not specified. Please provide the APY ranges using --apy-ranges argument or APY_RANGES environment variable.")
    exit(1)

def get_apy_history_csv_path():
    apy_history_csv_path = os.getenv('APY_HISTORY_CSV_PATH')
    if apy_history_csv_path:
        return apy_history_csv_path

    print("CSV path to APY history file is not specified. Please provide the CSV path using --csv-path argument or APY_HISTORY_CSV_PATH environment variable.")
    exit(1)
