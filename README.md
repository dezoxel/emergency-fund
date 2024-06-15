# Emergency Fund Management

Manage your family emergency fund based on data-driven decisions

Currently, only one simple script is available. It finds the best savings account based on the Coefficient of Stability and Profitability (CSP). It calculates the CSP for each savings account using their Annual Percentage Yield (APY) data over the past year.

## Table of Contents

- [Introduction](#introduction)
- [Prerequisites](#prerequisites)
- [Usage](#usage)
- [Testing](#testing)
- [Database Schema](#database-schema)
- [License](#license)

## Introduction

The `detect_best_csp_savings_account.py` script connects to an SQLite database containing historical APY data for various savings accounts. It updates the APY data for the past year, calculates the CSP for each account, and identifies the account with the highest CSP.

## Prerequisites

- Python 3.7+
- SQLite
- Required Python packages: `pandas`, `pytest`, `argparse`

## Usage

1. **Database Setup:**

   Before running the script, ensure your SQLite database is set up correctly using the provided `schema.sql` file.

   ```bash
   sqlite3 your_database.db < schema.sql
   ```

2. **Running the Script:**

   To run the script, you can provide the path to your SQLite database file via a command-line argument or an environment variable.

   ```bash
   python detect_best_csp_savings_account.py --db-path /path/to/your_database.db
   ```

   Or set the `DB_PATH` environment variable:

   ```bash
   export DB_PATH=/path/to/your_database.db
   python detect_best_csp_savings_account.py
   ```

3. **Output:**

   The script will output the account with the highest CSP along with its mean APY and variance of APY.

## Testing

```bash
pytest test_detect_best_csp_savings_account.py
```

## Database Schema

The `schema.sql` file sets up the necessary tables and inserts sample data for testing purposes. The main tables include:

- `institutions`: Information about financial institutions.
- `savings_accounts`: Details about savings accounts under interest.
- `savings_accounts_apy_history`: Historical APY data for savings accounts.
- `savings_accounts_apy_last_year`: APY data for the last year for every savings account.
- `savings_account_withdraw_types`: Different withdrawal types for savings accounts.
- `savings_account_withdraw_terms_history`: Historical data on withdrawal terms for savings accounts.

## License

This project is licensed under the MIT License.
