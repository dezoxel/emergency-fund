import pandas as pd
import pytest
from app.sharpe_ratio import calc_apy_last_year

def test_calc_apy_last_year_empty():
    df = pd.DataFrame(columns=['account_id', 'apy', 'month'])
    result = calc_apy_last_year(df)
    assert result.empty, "The result should be an empty DataFrame"

def test_calc_apy_last_year_several_changes_to_apy_during_month():
    data = {
        'account_id': [1, 1, 1],
        'apy': [0.01, 0.02, 0.03],
        'month': ['2023-01', '2023-01', '2023-01'],
    }
    df = pd.DataFrame(data)

    result = calc_apy_last_year(df)

    assert len(result) == 1, "The result should contain only one row"
    assert result.iloc[0]['apy'] == 0.03, "The latest apy should be 0.03"
    assert result.iloc[0]['date'] == '2023-01-01', "The date should be '2023-01-01'"

def test_calc_apy_last_year_less_than_one_year():
    data = {
        'account_id': [1, 1, 1, 1],
        'apy': [0.01, 0.02, 0.03, 0.04],
        'month': ['2023-01', '2023-02', '2023-03', '2023-04'],
    }
    df = pd.DataFrame(data)

    result = calc_apy_last_year(df)

    assert len(result) == 4, "The result should contain four rows"
    assert set(result['date']) == {'2023-01-01', '2023-02-01', '2023-03-01', '2023-04-01'}, "The dates should match the months in the data"

def test_calc_apy_last_year_max_12_records():
    data = {
        'account_id': [1] * 18 + [2] * 18,
        'apy': [2.5 + i * 0.1 for i in range(18)] + [3 + i * 0.1 for i in range(18)],
        'month': ['2022-{:02d}'.format((i % 12) + 1) for i in range(12)] + ['2023-{:02d}'.format((i % 6) + 1) for i in range(6)] +
                 ['2022-{:02d}'.format((i % 12) + 1) for i in range(12)] + ['2023-{:02d}'.format((i % 6) + 1) for i in range(6)],
    }
    df = pd.DataFrame(data)

    result = calc_apy_last_year(df)

    assert len(result) <= 24, "The result should contain at most 12 rows per account_id"
    assert all(result.groupby('account_id').size() <= 12), "Each account_id should have at most 12 records"
    assert all(result.groupby('account_id')['date'].nunique() <= 12), "Each account_id should have at most 12 unique months"

if __name__ == "__main__":
    pytest.main([__file__])
