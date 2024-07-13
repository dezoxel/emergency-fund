DROP TABLE IF EXISTS savings_accounts_apy_last_year;
DROP TABLE IF EXISTS savings_account_withdraw_terms_history;
DROP TABLE IF EXISTS savings_account_withdraw_types;
DROP TABLE IF EXISTS savings_accounts;
DROP TABLE IF EXISTS institutions;
DROP TABLE IF EXISTS savings_accounts_apy_history;

CREATE TABLE institutions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    address TEXT,
    phone TEXT,
    website TEXT
);

CREATE TABLE savings_accounts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    institution_id INTEGER NOT NULL,
    account_type TEXT CHECK(account_type IN ('High-Yield Savings', 'Money Market', 'Cash Management')),
    account_name TEXT NOT NULL,
    FOREIGN KEY (institution_id) REFERENCES institutions (id)
);

CREATE TABLE savings_account_withdraw_types (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id INTEGER NOT NULL,
    withdrawal_type TEXT NOT NULL,
    FOREIGN KEY (account_id) REFERENCES savings_accounts (id)
);

CREATE TABLE savings_account_withdraw_terms_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    withdrawal_type_id INTEGER NOT NULL,
    account_id INTEGER NOT NULL,
    fee REAL,
    currency_conversion_fee REAL,
    additional_fee TEXT,
    daily_limit REAL,
    monthly_limit REAL,
    notice_period_days INTEGER,
    transfer_limit REAL,
    fee_for_excessive_withdrawals REAL,
    identification_required TEXT,
    additional_restrictions TEXT,
    effective_date DATETIME NOT NULL,
    FOREIGN KEY (withdrawal_type_id) REFERENCES savings_account_withdraw_types (id),
    FOREIGN KEY (account_id) REFERENCES savings_accounts (id)
);

CREATE TABLE savings_accounts_apy_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id INTEGER NOT NULL,
    apy REAL,
    effective_date DATETIME NOT NULL,
    FOREIGN KEY (account_id) REFERENCES savings_accounts (id)
);

CREATE TABLE savings_accounts_apy_last_year (
    account_id INTEGER,
    apy REAL,
    date DATE,
    FOREIGN KEY (account_id) REFERENCES savings_accounts (id),
    UNIQUE(account_id, date)
);
