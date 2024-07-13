from datetime import datetime
from sqlite3 import Connection

from infra.savings_account_apy_history_repo import fetch_apy_history_last_year
from domain.csp import calc_apy_last_year, calc_best_csp_savings_account
from infra.savings_account_apy_last_year_repo import clear_apy_last_year, fetch_apy_last_year

def update_apy_last_year(conn: Connection, current_date: datetime):
    df = fetch_apy_history_last_year(conn, current_date)
    if df.empty:
        print('Unable to update APY data for the last year. APY history for the last year is empty.')
    clear_apy_last_year(conn)
    df = calc_apy_last_year(df)
    df.to_sql('savings_accounts_apy_last_year', conn, if_exists='append', index=False)
    conn.commit()

def find_best_csp_savings_account(conn, current_date):
    update_apy_last_year(conn, current_date)
    df = fetch_apy_last_year(conn)
    if df.empty:
        print('Unable to find the best CSP savings account. APY data for the last year is empty.')
        return None
    return calc_best_csp_savings_account(df)

def print_best_csp_savings_account(best_account):
    if best_account is None:
        print('Best CSP Savings Account not found.')
    else:
        print(f"Best Account based on CSP:\nAccount ID: {best_account['account_id']}\nMean APY: {best_account['mean']:.4f}\nStandard Deviation of APY: {best_account['std']:.10f}\nCSP: {best_account['csp']:.4f}")

def find_and_print_best_csp_savings_account(conn: Connection, current_date: datetime):
    best_account = find_best_csp_savings_account(conn, current_date)
    print_best_csp_savings_account(best_account)

