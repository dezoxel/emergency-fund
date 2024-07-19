import pandas as pd
from datetime import datetime
from sqlite3 import Connection

from domain.balance import convert_apy_to_annual_rate_terms_history
from infra.risk_free_rate_repo import fetch_rfr_history_last_year
from infra.savings_account_apy_history_repo import fetch_apy_history_last_year, fetch_terms_history_last_year
from domain.account_sharpe_ratio import calc_apy_last_year, calc_best_savings_account_by_sharpe_ratio, calc_compound_future_value_for_every_account, calc_return_rate_for_every_account, calc_sharpe_ratio_for_every_account, calc_std_returns_for_every_account
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

def print_best_savings_account_by_sharpe_ratio(best_account: dict):
    print(
        f"Best Savings Account by Sharpe Ratio is:\n"
        f"Institution: {best_account['institution_name']}\n"
        f"Account Type: {best_account['account_type']}\n"
        f"Account Name: {best_account['account_name']}\n"
        f"Account ID: {best_account['account_id']:.0f}\n"
        f"Sharpe Ratio: {best_account['sr']:.4f}\n"
        f"ROI: {best_account['return_rate']*100:.2f}%\n"
        f"Risk: {best_account['risk_place']} place among {best_account['total_accounts']} accounts (less is better)\n"
    )

def calc_risk_place(df, account_id):
    risk_place = df.sort_values(by='std_returns')['account_id'].tolist().index(account_id) + 1
    return risk_place

def find_and_print_best_savings_account_by_sharpe_ratio_last_year(conn: Connection, current_date: datetime, P: float):
    apy_terms_history = fetch_terms_history_last_year(conn, current_date)
    rfr_history = fetch_rfr_history_last_year(conn, current_date)

    annual_rate_terms_history = convert_apy_to_annual_rate_terms_history(apy_terms_history)

    future_value_by_account = calc_compound_future_value_for_every_account(P, current_date, annual_rate_terms_history)
    sharpe_ratio_df = calc_sharpe_ratio_for_every_account(future_value_by_account, rfr_history)
    std_returns_df = calc_std_returns_for_every_account(future_value_by_account)
    return_rate_df = calc_return_rate_for_every_account(future_value_by_account)
    finance_info = sharpe_ratio_df.merge(std_returns_df, on='account_id').merge(return_rate_df, on='account_id')

    best_account_stats = calc_best_savings_account_by_sharpe_ratio(finance_info)
    risk_place = calc_risk_place(finance_info, best_account_stats['account_id'])

    account_details = find_savings_account_by_id(conn, best_account_stats['account_id'])

    total_accounts = annual_rate_terms_history['account_id'].nunique()
    best_account_report = best_account_stats | account_details | {'total_accounts': total_accounts, 'risk_place': risk_place}

    print_best_savings_account_by_sharpe_ratio(best_account_report)

