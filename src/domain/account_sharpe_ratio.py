import numpy as np
import pandas as pd

from domain.balance import calc_compound_future_value_with_dynamic_terms
from domain.sharpe_ratio_math import calc_sharpe_ratio

def calc_compound_future_value_for_every_account(P, current_date, terms_history):
    terms_by_account = terms_history.groupby('account_id')

    stats_by_account = {
        account_id: calc_compound_future_value_with_dynamic_terms(P, terms.apply(lambda row: row.to_dict(), axis=1).reset_index(drop=True), current_date)
        for account_id, terms in terms_by_account
    }

    return stats_by_account

def calc_sharpe_ratio_for_every_account(future_value_by_account, rfr_history):
    risk_free_rates = rfr_history['rfr'].tolist()
    results = {'account_id': [], 'sr': []}
    
    for account_id, values in future_value_by_account.items():
        returns = values['return_rate_history'][1:]
        sharpe_ratio = calc_sharpe_ratio(returns, risk_free_rates)

        results['account_id'].append(account_id)
        results['sr'].append(sharpe_ratio)

    results_df = pd.DataFrame(results)

    return results_df

def calc_std_returns_for_every_account(future_value_by_account):
    return pd.DataFrame({
        'account_id': future_value_by_account.keys(),
        'std_returns': [np.std(values['return_rate_history'][1:], ddof=1) for values in future_value_by_account.values()]
    })

def calc_return_rate_for_every_account(future_value_by_account):
    return pd.DataFrame({
        'account_id': future_value_by_account.keys(),
        'return_rate': [values['return_rate_total'] for values in future_value_by_account.values()]
    })

def calc_best_savings_account_by_sharpe_ratio(sr_df):
    best_account = sr_df.loc[sr_df['sr'].idxmax()].to_dict()

    return best_account

# @deprecated against using the full history of APY changes
def calc_apy_last_year(df):
    df = df.copy()
    # Drop duplicates, keeping the latest effective_date for each account_id and month,
    # assuming the rows are sorted by effective_date
    df = df.drop_duplicates(subset=['account_id', 'month'], keep='last')
    
    # Filter to keep only the most recent 12 months per account_id
    df['rank'] = df.groupby('account_id')['month'].rank(method='first', ascending=False)
    df = df[df['rank'] <= 12].reset_index()

    df['date'] = df['month'] + '-01'
    df = df[['account_id', 'apy', 'date']]
    
    return df
