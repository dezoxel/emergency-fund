def calc_csp_for_every_account(df):
    if df.empty:
        return []
    
    grouped = df.groupby('account_id')['apy'].agg(['mean', 'std']).reset_index()
    
    # Replace zero std with a small number to avoid division by zero
    grouped['std'] = grouped['std'].replace(0, 1e-10)
    
    grouped['csp'] = grouped['mean'] / grouped['std']
    return grouped

def calc_best_csp_savings_account(df):
    if df.empty:
        return None
    
    csp_df = calc_csp_for_every_account(df)
    
    best_account = csp_df.loc[csp_df['csp'].idxmax()].copy()
    account_name = df.loc[df['account_id'] == best_account['account_id'], 'account_name'].iloc[0]
    best_account['account_name'] = account_name
    
    return best_account

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
