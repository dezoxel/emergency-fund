# Emergency Fund Management

Manage your family emergency fund based on data-driven decisions.

## Environment pre-requisites

- Python 3.11 or higher
- SQLite 3

Docker is not supported.

## Install and Setup


Install libraries:
```bash
pip install -r requirements.txt
```

Create environment configuration file:
```bash
cp .env.example .env
```
Default values are good to go. You can change them if needed.

Create DB tables:
```bash
sqlite3 ./data/emergency_fund.db < ./db/schema.sql
```

Insert seed data:
```bash
sqlite3 ./data/emergency_fund.db <<EOF
.mode csv
.import ./db/institutions.seed.csv institutions
.import ./db/savings_accounts.seed.csv savings_accounts
.import ./db/savings_accounts_apy_history.seed.csv savings_accounts_apy_history
.import ./db/risk_free_rate.seed.csv risk_free_rate_history
EOF
```

At this point we have all data in DB we need so we can start using the application.

## Usage

### Find the best savings account based on the Sharpe Ratio

[Sharpe Ratio](https://www.investopedia.com/terms/s/sharperatio.asp) - a measure for calculating risk-adjusted return.

Sharpe Ratio respects low volatility and high return. The higher the Sharpe Ratio, the better the investment's return relative to the risk.

You can run as the CLI tool:
```bash
python src/find_best_by_sharpe_ratio.py
```

You can also use Jupyter Notebook `src/find_best_by_sharpe_ratio.ipynb`.

## Find all accounts' profitability

You can run as the CLI tool:
```bash
python src/find_profitability.py
```

You can also use Jupyter Notebook `src/find_profitability.ipynb`.

## Explore the data

You can use Jupyter Notebook `src/eda.ipynb`.

## Generate APY history for the last year

Generate APY history data for every account:
```bash
python src/generate_apy_history.py
```

Insert newly generated APY history data to DB:
```bash
sqlite3 ./data/emergency_fund.db <<EOF
.mode csv
.import ./db/savings_accounts_apy_history.seed.csv savings_accounts_apy_history
EOF
```

### Clean up the DB

```bash
rm ./data/emergency_fund.db
```

## Troubleshooting

### APY history for the last year is empty

```bash
python src/find_best_by_sharpe_ratio.py
```

```
Unable to update APY data for the last year. APY history for the last year is empty.
Unable to find the best savings account by Sharpe Ratio. APY data for the last year is empty.
Best Savings Account by Sharpe Ratio is not found.
```

The APY history is generated for the time range using `START_DATE` and `END_DATE` environment variables. On the other side, the `find_best_by_sharpe_ratio.py` script uses the last year using the current date. If `START_DATE` and `END_DATE` are more than a year in the past, the history for the last will be empty. To fix this, you can update the `START_DATE` and `END_DATE` environment variables in the `.env` file, clean up the DB, recreate the DB tables, insert seed data, and generate the history again. See commands in the "Clean up the DB" and "Install and Setup" sections.

## License

This project is licensed under the MIT License.
