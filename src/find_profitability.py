from dotenv import load_dotenv

from app.cli_args import get_database_path, parse_arguments
from domain.calc_profitability import calc_balance_for_all_accounts
from infra.db_connection import db_connect, db_disconnect

def main():
    load_dotenv()
    args = parse_arguments()
    db_path = get_database_path(args)

    try:
        conn = db_connect(db_path)
        df = calc_balance_for_all_accounts(conn)
        print(df)
    finally:
        db_disconnect(conn)

if __name__ == "__main__":
    main()
