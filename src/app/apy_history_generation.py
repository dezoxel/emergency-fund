import pandas as pd
import numpy as np

from libs.path import get_abs_path

def generate_apy(account_type, dates, apy_ranges):
    min_apy, max_apy, mean_low, mean_high, std = apy_ranges[account_type]
    apys = []
    for date in dates:
        mean = np.random.uniform(mean_low, mean_high)
        apy = np.clip(np.random.normal(mean, std), min_apy, max_apy)
        apys.append(apy)
    return apys

def generate_apy_history(accounts, date_range, apy_ranges):
    apy_history = []
    id = 1
    for account in accounts:
        apys = generate_apy(account['account_type'], date_range, apy_ranges)
        for date, apy in zip(date_range, apys):
            apy_history.append({
                'id': id,
                'account_id': account['id'],
                'apy': apy,
                'effective_date': date
            })
            id += 1
    return apy_history

def save_apy_history_to_csv(apy_history, file_path):
    apy_history_df = pd.DataFrame(apy_history)
    file_path_abs = get_abs_path('../..', file_path)
    apy_history_df.to_csv(file_path_abs, index=False, header=False)
    print(f'The APY history has been generated and saved to the CSV file: {file_path_abs}')
