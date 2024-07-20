from datetime import datetime
from sqlite3 import Connection

from domain.balance import calc_balance_for_all_accounts
from infra.savings_account_apy_history_repo import fetch_terms_history_last_year

def find_balance_for_all_accounts_last_year(conn: Connection, current_date: datetime, P: float):
    terms_history = fetch_terms_history_last_year(conn, current_date)

    return calc_balance_for_all_accounts(P, current_date, terms_history)
