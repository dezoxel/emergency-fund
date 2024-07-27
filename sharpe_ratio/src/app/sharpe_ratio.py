from domain.balance import convert_apy_to_annual_rate_terms_history
from infra.risk_free_rate_repo import fetch_rfr_history_last_year
from infra.savings_account_apy_history_repo import fetch_terms_history_last_year
from domain.account_sharpe_ratio import calc_balance_for_every_account, calc_best_savings_account_by_sharpe_ratio, calc_compound_future_value_for_every_account, calc_return_rate_for_every_account, calc_sharpe_ratio_for_every_account, calc_std_returns_for_every_account

def calc_risk_place(df):
    df['risk_place'] = df['std_returns'].rank(method='first').astype(int)
    return df

def calc_profitability_place(df):
    df['profitability_place'] = df['balance'].rank(method='first', ascending=False).astype(int)
    return df

def get_profitability_place_by_id(df, account_id):
    place = df[df['account_id'] == account_id]['profitability_place'].iloc[0]
    return place

def get_risk_place_by_id(df, account_id):
    place = df[df['account_id'] == account_id]['risk_place'].iloc[0]
    return place

def get_balance_by_id(df, account_id):
    balance = df[df['account_id'] == account_id]['balance'].iloc[0]
    return balance

def find_and_print_best_savings_account_by_sharpe_ratio_last_year(conn, current_date, P):
    apy_terms_history = fetch_terms_history_last_year(conn, current_date)
    rfr_history = fetch_rfr_history_last_year(conn, current_date)

    annual_rate_terms_history = convert_apy_to_annual_rate_terms_history(apy_terms_history)

    future_value_by_account = calc_compound_future_value_for_every_account(P, current_date, annual_rate_terms_history)
    sharpe_ratio_df = calc_sharpe_ratio_for_every_account(future_value_by_account, rfr_history)
    std_returns_df = calc_std_returns_for_every_account(future_value_by_account)
    return_rate_df = calc_return_rate_for_every_account(future_value_by_account)
    balance_df = calc_balance_for_every_account(future_value_by_account)

    account_meta_df = apy_terms_history[['account_id', 'account_name', 'account_type', 'institution_name']].drop_duplicates()
    finance_info = (
        sharpe_ratio_df
            .merge(std_returns_df, on='account_id')
            .merge(return_rate_df, on='account_id')
            .merge(balance_df, on='account_id')
            .merge(account_meta_df, on='account_id', how='left')
    )
    finance_info = calc_risk_place(finance_info)
    finance_info = calc_profitability_place(finance_info)
    finance_info['principal_amount'] = P

    best_account_stats = calc_best_savings_account_by_sharpe_ratio(finance_info)

    total_accounts = annual_rate_terms_history['account_id'].nunique()
    best_account_report = best_account_stats | {
        'total_accounts': total_accounts,
    }

    print_best_savings_account_by_sharpe_ratio(best_account_report)

def find_sharpe_ratio_for_all_accounts_last_year(conn, current_date, P):
    apy_terms_history = fetch_terms_history_last_year(conn, current_date)
    rfr_history = fetch_rfr_history_last_year(conn, current_date)

    annual_rate_terms_history = convert_apy_to_annual_rate_terms_history(apy_terms_history)

    future_value_by_account = calc_compound_future_value_for_every_account(P, current_date, annual_rate_terms_history)
    sharpe_ratio_df = calc_sharpe_ratio_for_every_account(future_value_by_account, rfr_history)
    std_returns_df = calc_std_returns_for_every_account(future_value_by_account)
    return_rate_df = calc_return_rate_for_every_account(future_value_by_account)
    balance_df = calc_balance_for_every_account(future_value_by_account)

    account_meta_df = apy_terms_history[['account_id', 'account_name', 'account_type', 'institution_name']].drop_duplicates()
    finance_info = (
        sharpe_ratio_df
            .merge(std_returns_df, on='account_id')
            .merge(return_rate_df, on='account_id')
            .merge(balance_df, on='account_id')
            .merge(account_meta_df, on='account_id', how='left')
    )
    finance_info = calc_risk_place(finance_info)
    finance_info = calc_profitability_place(finance_info)
    finance_info['principal_amount'] = P

    df = finance_info.sort_values(by='sr', ascending=False).reset_index(drop=True)

    return df

def print_best_savings_account_by_sharpe_ratio(best_account):
    print(
        f"Institution: {best_account['institution_name']}\n"
        f"Account Type: {best_account['account_type']}\n"
        f"Account Name: {best_account['account_name']}\n"
        f"Account ID: {best_account['account_id']:.0f}\n"
        f"Principal Amount: {best_account['principal_amount']:.2f}\n"
        f"Current Balance: {best_account['balance']:.2f}\n"
        f"Sharpe Ratio: {best_account['sr']:.4f}\n"
        f"ROI: {best_account['return_rate']*100:.2f}%\n"
        f"Risk: {best_account['risk_place']:.0f} place among {best_account['total_accounts']} accounts (less is better)\n"
        f"Profitability: {best_account['profitability_place']} place among {best_account['total_accounts']} accounts (less is better)\n"
    )
