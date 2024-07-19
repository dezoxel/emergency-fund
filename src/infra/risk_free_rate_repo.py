from datetime import timedelta
import pandas as pd

def fetch_rfr_history_last_year(conn, current_date):
    # TODO: Handle leap years
    one_year_ago = current_date - timedelta(days=365)

    query = """
    SELECT rate as rfr, effective_date as date
    FROM risk_free_rate_history
    WHERE effective_date >= ?
    ORDER BY effective_date DESC
    """
    rfr_df = pd.read_sql_query(query, conn, params=[one_year_ago])

    # ensure that we have the rate for the beginning of the period
    if rfr_df[rfr_df['date'] == one_year_ago.strftime('%Y-%m-%d')].empty:
        q = """
        SELECT rate as rfr, effective_date as date
        FROM risk_free_rate_history
        WHERE effective_date < :one_year_ago
        ORDER BY effective_date DESC
        LIMIT 1
        """
        last_record_before_year_ago_df = pd.read_sql_query(q, conn, params=[one_year_ago])

    if not last_record_before_year_ago_df.empty and not rfr_df.empty:
        df = pd.concat([last_record_before_year_ago_df, rfr_df])
    elif not last_record_before_year_ago_df.empty:
        df = last_record_before_year_ago_df
    elif not rfr_df.empty:
        df = rfr_df
    
    df['rfr'] = df['rfr']/100

    if df.empty:
        print('Risk-Free Rate history is absent in DB.')
        return None

    return df
