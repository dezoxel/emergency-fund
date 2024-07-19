from datetime import datetime
import pandas as pd
from typing import Dict, List

from domain.balance import calc_returns 
from domain.sharpe_ratio_math import calc_sharpe_ratio

# TODO: replace with dataclass?
ShareRatioStatsByAccount = Dict[str, Dict[str, List[float]]]

def calc_sr_stats_by_account(P: float, current_date: datetime, terms_history: pd.DataFrame, rfr_history: pd.DataFrame) -> pd.DataFrame:
    def calc_returns_for_account(account_terms_history: pd.DataFrame) -> float:
        returns = calc_returns(P, current_date, account_terms_history)
        return returns

    returns_df = terms_history.groupby('account_id').apply(calc_returns_for_account).reset_index()
    returns_df.columns = ['account_id', 'returns']

    rfr_list = rfr_history['rfr'].tolist()
    stats = {row['account_id']: {'returns': row['returns'], 'rfr': rfr_list} for _, row in returns_df.iterrows()}

    return stats

def calc_sharpe_ratios_by_account(sr_stats_by_acc: ShareRatioStatsByAccount) -> pd.DataFrame:
    results = {'account_id': [], 'sr': []}
    
    for account_id, values in sr_stats_by_acc.items():
        returns = values['returns']
        risk_free_rates = values['rfr']
        sharpe_ratio = calc_sharpe_ratio(returns, risk_free_rates)
        results['account_id'].append(account_id)
        results['sr'].append(sharpe_ratio)
    
    return pd.DataFrame(results)

def calc_sharpe_ratio_for_all_accounts(P, current_date, terms_history_df, rfr_history_df):
    sr_stats_by_acc = calc_sr_stats_by_account(P, current_date, terms_history_df, rfr_history_df)
    sr_df = calc_sharpe_ratios_by_account(sr_stats_by_acc)
    
    return sr_df

def calc_best_savings_account_by_sharpe_ratio(P, current_date, terms_history_df, rfr_history_df):
    sr_df = calc_sharpe_ratio_for_all_accounts(P, current_date, terms_history_df, rfr_history_df)
    
    best_account = sr_df.loc[sr_df['sr'].idxmax()]
    
    return best_account.to_dict()

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
