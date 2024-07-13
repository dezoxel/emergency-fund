# Emergency Fund Management

Manage your family emergency fund based on data-driven decisions.

## Table of Contents

- [Introduction](#introduction)
- [Prerequisites](#prerequisites)
- [Usage](#usage)
- [Testing](#testing)
- [Database Schema](#database-schema)
- [License](#license)

## Install and Setup

Pre-requisites:
- Python 3.11 or higher
- SQLite 3

Docker is not supported.

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
EOF
```

Generate APY history data for every account:
```bash
python3 src/generate_apy_history.py
```

Insert newly generated APY history data to DB:
```bash
sqlite3 ./data/emergency_fund.db <<EOF
.mode csv
.import ./db/savings_accounts_apy_history.seed.csv savings_accounts_apy_history
EOF
```

At this point we have all data in DB we need so we can start using the application.

## Usage

### Clean up the DB

```bash
rm ./data/emergency_fund.db
```

### Find the best savings account based on the CSP metric

CSP - Coefficient of Stability and Profitability.

You can run as the CLI tool:
```bash
python3 src/find_best_csp.py
```

You can also use Jupyter Notebook `src/find_best_csp.ipynb`.

## Find all accounts' profitability

You can run as the CLI tool:
```bash
python3 src/find_profitability.py
```

You can also use Jupyter Notebook `src/find_profitability.ipynb`.

## Explore the data

You can use Jupyter Notebook `src/eda.ipynb`.

## Testing

```bash
pytest test_detect_best_csp_savings_account.py
```

## Troubleshooting

### APY history for the last year is empty

```bash
python3 src/find_best_csp.py
```

```
Unable to update APY data for the last year. APY history for the last year is empty.
Unable to find the best CSP savings account. APY data for the last year is empty.
Best CSP Savings Account not found.
```

The APY history is generated for the time range using `START_DATE` and `END_DATE` environment variables. On the other side, the `find_best_csp.py` script uses the last year using the current date. If `START_DATE` and `END_DATE` are more than a year in the past, the history for the last will be empty. To fix this, you can update the `START_DATE` and `END_DATE` environment variables in the `.env` file, clean up the DB, recreate the DB tables, insert seed data, and generate the history again. See commands in the "Clean up the DB" and "Install and Setup" sections.

## License

This project is licensed under the MIT License.
