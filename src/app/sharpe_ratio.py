from datetime import datetime
from sqlite3 import Connection

from infra.savings_account_apy_history_repo import fetch_apy_history_last_year
from domain.calc_sharpe_ratio import calc_apy_last_year, calc_best_savings_account_by_sharpe_ratio
from infra.savings_account_apy_last_year_repo import clear_apy_last_year, fetch_apy_last_year

def update_apy_last_year(conn: Connection, current_date: datetime):
    df = fetch_apy_history_last_year(conn, current_date)
    if df.empty:
        print('Unable to update APY data for the last year. APY history for the last year is empty.')
    clear_apy_last_year(conn)
    df = calc_apy_last_year(df)
    df.to_sql('savings_accounts_apy_last_year', conn, if_exists='append', index=False)
    conn.commit()

def find_best_savings_account_by_sharpe_ratio(conn: Connection, current_date: datetime):
    update_apy_last_year(conn, current_date)
    df = fetch_apy_last_year(conn)
    if df.empty:
        print('Unable to find the best savings account by Sharpe Ratio. APY data for the last year is empty.')
        return None
    return calc_best_savings_account_by_sharpe_ratio(df)

def print_best_savings_account_by_sharpe_ratio(best_account):
    if best_account is None:
        print('Best Savings Account by Sharpe Ratio is not found.')
    else:
        print(f"Best Savings Account by Sharpe Ratio:\nAccount ID: {best_account['account_id']}\nMean APY: {best_account['mean']:.4f}\nStandard Deviation of APY: {best_account['std']:.10f}\nSharpe Ratio: {best_account['sr']:.4f}")

def find_and_print_best_savings_account_by_sharpe_ratio(conn: Connection, current_date: datetime):
    best_account = find_best_savings_account_by_sharpe_ratio(conn, current_date)
    print_best_savings_account_by_sharpe_ratio(best_account)

