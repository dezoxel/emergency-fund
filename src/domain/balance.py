import pandas as pd
from datetime import datetime
from typing import Dict, Tuple, List, Union

from domain.compound_interest_math import calc_annual_rate_by_APY, calc_compound_future_value
from domain.datetime import calculate_time_diff_in_years

Terms = Dict[str, Union[float, int, datetime]]
TermsHistory = List[Terms]
FutureValueDetails = Tuple[float, List[float], List[float]]
DateRange = Tuple[datetime, datetime]

def get_terms_period(current_terms: Terms, next_terms: Terms | None, current_date: datetime) -> DateRange:
    terms_change_date = current_terms['date']
    next_terms_change_date = next_terms['date'] if next_terms else current_date
    return terms_change_date, next_terms_change_date

def calc_compound_future_value_with_dynamic_terms(P: float, terms_history: TermsHistory, current_date: datetime) -> FutureValueDetails:
    accumulated_amount = P
    balance_history = [P]
    return_rate_history = [0]
    
    for i in range(len(terms_history)):
        interest_rate = terms_history[i]['r_a']
        compounding_frequency = terms_history[i]['n']
        terms_change_date, next_terms_change_date = get_terms_period(terms_history[i], terms_history.get(i + 1, None), current_date)
        time_diff_years = calculate_time_diff_in_years(terms_change_date, next_terms_change_date)

        accumulated_amount = calc_compound_future_value(accumulated_amount, interest_rate, compounding_frequency, time_diff_years)
        return_rate = (accumulated_amount - P) / P

        balance_history.append(accumulated_amount)
        return_rate_history.append(return_rate)
    
    return accumulated_amount, balance_history, return_rate_history

def calc_returns(P: float, current_date: datetime, account_terms_history: pd.Series) -> List[float]:
    annual_rate_terms_history = convert_apy_to_annual_rate_terms_history(account_terms_history)
    _, _, return_rate_history = calc_compound_future_value_with_dynamic_terms(P, annual_rate_terms_history, current_date)
    return return_rate_history[1:]

def calc_balance(P: float, current_date: datetime, account_terms_history: pd.Series) -> List[float]:
    annual_rate_terms_history = convert_apy_to_annual_rate_terms_history(account_terms_history)
    balance, _, _ = calc_compound_future_value_with_dynamic_terms(P, annual_rate_terms_history, current_date)
    return balance

def convert_apy_to_annual_rate_terms_history(account_terms_history: pd.Series) -> pd.Series:
    terms = pd.Series(account_terms_history.apply(lambda t: {'APY': t['apy'], 'n': t['n'], 'date': t['date']}, axis=1)).reset_index(drop=True)

    return terms.apply(lambda term: {
        'r_a': calc_annual_rate_by_APY(term['n'], term['APY']),
        'n': term['n'],
        'date': datetime.strptime(term['date'], '%Y-%m-%d')
    })

def calc_balance_for_all_accounts(P: float, current_date: datetime, terms_history: pd.DataFrame) -> pd.DataFrame:
    def calc_balance_for_account(account_terms_history: pd.DataFrame) -> float:
        balance = calc_balance(P, current_date, account_terms_history)
        return balance

    df = terms_history.groupby('account_id').apply(calc_balance_for_account).reset_index()
    df.columns = ['account_id', 'balance']

    return df