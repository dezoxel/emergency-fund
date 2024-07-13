from datetime import datetime
from dateutil.relativedelta import relativedelta
import calendar

def calc_compound_value(P, r, n, t):
    A = P * (1 + r/n)**(n * t)
    return A

def calc_balance_with_dynamic_terms(P, terms_history, current_date):
    A = P
    
    for i in range(len(terms_history)):
        r = terms_history[i]['r']
        n = terms_history[i]['n']
        date = datetime.strptime(terms_history[i]['date'], '%Y-%m-%d')
        
        if i < len(terms_history) - 1:
            next_date = datetime.strptime(terms_history[i + 1]['date'], '%Y-%m-%d')
        else:
            next_date = datetime.strptime(current_date, '%Y-%m-%d')

        rd = relativedelta(next_date, date)
        full_months = rd.years * 12 + rd.months
        
        # handle partial months
        if rd.days > 0:
            _, days_in_month = calendar.monthrange(date.year, date.month)
            partial_month = rd.days / days_in_month
            time_diff_months = full_months + partial_month
        else:
            time_diff_months = full_months
        
        time_diff_years = time_diff_months / 12
        
        A = calc_compound_value(A, r, n, time_diff_years)
    
    return A
