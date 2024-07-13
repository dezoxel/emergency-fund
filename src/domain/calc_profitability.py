from datetime import timedelta, datetime
import pandas as pd

from domain.calc_balance import calc_balance_with_dynamic_terms

def calculate_value(df, initial_principal, current_date):
    unique_accounts = df['account_id'].unique()
    result = []

    for account in unique_accounts:
        account_data = df[df['account_id'] == account].sort_values(by='date')
        terms_history = [{'r': row['apy']/100, 'n': 12, 'date': row['date']} for _, row in account_data.iterrows()]
        balance = calc_balance_with_dynamic_terms(initial_principal, terms_history, current_date)
        result.append({'accountId': account, 'balance': balance, 'accountName': account_data['account_name'].iloc[0]})
    
    return pd.DataFrame(result, columns=['accountId', 'accountName', 'balance'])

def calc_balance_for_all_accounts(conn):
    P = 1000
    current_date = datetime.strptime('2024-06-01', '%Y-%m-%d')
    # TODO: Handle leap years
    one_year_ago = current_date - timedelta(days=365)

    query = """
    SELECT account_id, account_name, apy, effective_date AS date
    FROM savings_accounts_apy_history saah
    LEFT JOIN savings_accounts sa ON saah.account_id = sa.id
    WHERE effective_date >= ?
    ORDER BY account_id, date DESC
    """
    df = pd.read_sql_query(query, conn, params=[one_year_ago])

    return calculate_value(df, P, current_date.strftime('%Y-%m-%d'))
