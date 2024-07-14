from dotenv import load_dotenv
from datetime import datetime

from app.cli_args import get_database_path, parse_arguments
from app.sharpe_ratio import find_and_print_best_savings_account_by_sharpe_ratio
from infra.db_connection import db_connect, db_disconnect

def main():
    load_dotenv()
    args = parse_arguments()
    db_path = get_database_path(args)

    try:
        conn = db_connect(db_path)
        current_date = datetime.now()
        find_and_print_best_savings_account_by_sharpe_ratio(conn, current_date)
    finally:
        db_disconnect(conn)

if __name__ == "__main__":
    main()
