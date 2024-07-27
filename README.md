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

### Find the best savings account by Sharpe Ratio for the last year

[Sharpe Ratio](https://www.investopedia.com/terms/s/sharperatio.asp) - a measure for calculating risk-adjusted return.

Sharpe Ratio respects low volatility and high return. The higher the Sharpe Ratio, the better the investment's return relative to the risk.

Use Jupyter Notebook to calculate. Open and run: `src/sharpe_ratio.ipynb`.

## Explore the data

Open and run: `src/eda.ipynb`.

## Generate APY history for the last year

Open and run: `sharpe_ratio/src/generate_apy_history.ipynb`.

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

## License

This project is licensed under the MIT License.
