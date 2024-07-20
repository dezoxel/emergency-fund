import pytest
from domain.balance import calc_compound_future_value_with_dynamic_terms

# APY 10% and constant, 12 months
def test_single_term():
    P = 1000
    current_date = '2024-01-01'
    terms_history = [
        {'r': 0.1, 'n': 12, 'date': '2023-01-01'},
    ]

    future_value = calc_compound_future_value_with_dynamic_terms(P, terms_history, current_date)

    assert round(future_value, 2) == 1_104.71, "calcs correctly the future value with a single term"

def test_apy_change_two_terms():
    P = 1000
    current_date = '2024-01-01'
    terms_history = [
        {'r': 0.1,  'n': 12, 'date': '2023-01-01'},
        {'r': 0.05, 'n': 12, 'date': '2023-07-01'},
    ]

    future_value = calc_compound_future_value_with_dynamic_terms(P, terms_history, current_date)

    assert round(future_value, 2) == 1_077.60, "calcs correctly the future value with a one APY change after a half year"

def test_apy_change_middle_month():
    P = 1000
    current_date = '2024-01-01'
    terms_history = [
        {'r': 0.1,  'n': 12, 'date': '2023-01-01'},
        {'r': 0.05, 'n': 12, 'date': '2023-07-15'},
    ]

    future_value = calc_compound_future_value_with_dynamic_terms(P, terms_history, current_date)

    assert round(future_value, 2) == 1_079.62, "calcs correctly the future value with a one APY change after a half year and 15 days"

if __name__ == "__main__":
    pytest.main([__file__])
