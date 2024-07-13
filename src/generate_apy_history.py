from dotenv import load_dotenv
import pandas as pd

from app.cli_args import parse_arguments, get_database_path, get_start_date, get_end_date, get_apy_ranges, get_apy_history_csv_path
from app.apy_history_generation import generate_apy_history, save_apy_history_to_csv
from infra.savings_account_repo import fetch_savings_accounts
from infra.db_connection import db_connect, db_disconnect

def main():
    load_dotenv()

    args = parse_arguments()
    db_path = get_database_path(args)
    start_date = get_start_date(args)
    end_date = get_end_date(args)
    apy_ranges = get_apy_ranges(args)
    apy_history_csv_path = get_apy_history_csv_path(args)
    
    date_range = pd.date_range(start_date, end_date, freq='MS')

    try:
        conn = db_connect(db_path)
        accounts = fetch_savings_accounts(conn)
        apy_history = generate_apy_history(accounts, date_range, apy_ranges)
        save_apy_history_to_csv(apy_history, apy_history_csv_path)
    finally:
        db_disconnect(conn)

if __name__ == "__main__":
    main()
