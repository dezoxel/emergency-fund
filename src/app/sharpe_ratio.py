import pandas as pd
from datetime import datetime
from sqlite3 import Connection

from infra.risk_free_rate_repo import fetch_rfr_history_last_year
from infra.savings_account_apy_history_repo import fetch_apy_history_last_year, fetch_terms_history_last_year
from domain.account_sharpe_ratio import calc_apy_last_year, calc_best_savings_account_by_sharpe_ratio, calc_sharpe_ratio_for_all_accounts
from infra.savings_account_apy_last_year_repo import clear_apy_last_year
from infra.savings_account_repo import find_savings_account_by_id

def update_apy_last_year(conn: Connection, current_date: datetime):
    df = fetch_apy_history_last_year(conn, current_date)
    if df.empty:
        print('Unable to update APY data for the last year. APY history for the last year is empty.')
    clear_apy_last_year(conn)
    df = calc_apy_last_year(df)
    df.to_sql('savings_accounts_apy_last_year', conn, if_exists='append', index=False)
    conn.commit()

def find_best_savings_account_by_sharpe_ratio(conn: Connection, current_date: datetime, P: float) -> dict:
    terms_history_df = fetch_terms_history_last_year(conn, current_date)
    rfr_history_df = fetch_rfr_history_last_year(conn, current_date)

    return calc_best_savings_account_by_sharpe_ratio(P, current_date, terms_history_df, rfr_history_df)

def find_sharpe_ratio_for_all_accounts(conn: Connection, current_date: datetime, P: float) -> pd.DataFrame:
    terms_history_df = fetch_terms_history_last_year(conn, current_date)
    rfr_history_df = fetch_rfr_history_last_year(conn, current_date)

    return calc_sharpe_ratio_for_all_accounts(P, current_date, terms_history_df, rfr_history_df)

def print_best_savings_account_by_sharpe_ratio(best_account: dict):
    print(
        f"Best Savings Account by Sharpe Ratio is:\n"
        f"Institution: {best_account['institution_name']}\n"
        f"Account Type: {best_account['account_type']}\n"
        f"Account Name: {best_account['account_name']}\n"
        f"Account ID: {best_account['account_id']:.0f}\n"
        f"Sharpe Ratio: {best_account['sr']:.4f}\n"
    )

def find_and_print_best_savings_account_by_sharpe_ratio(conn: Connection, current_date: datetime, P: float):
    best_account_id_and_sr = find_best_savings_account_by_sharpe_ratio(conn, current_date, P)
    account_details = find_savings_account_by_id(conn, best_account_id_and_sr['account_id'])
    best_account = best_account_id_and_sr | account_details
    print_best_savings_account_by_sharpe_ratio(best_account)

