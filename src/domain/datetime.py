from datetime import datetime
from dateutil.relativedelta import relativedelta
import calendar

def calculate_time_diff_in_years(begin_date: datetime, end_date: datetime) -> float:
    rd = relativedelta(end_date, begin_date)
    full_months = rd.years * 12 + rd.months
    
    if rd.days > 0:
        _, days_in_month = calendar.monthrange(begin_date.year, begin_date.month)
        partial_month = rd.days / days_in_month
        time_diff_months = full_months + partial_month
    else:
        time_diff_months = full_months

    time_diff_years = time_diff_months / 12

    return time_diff_years
