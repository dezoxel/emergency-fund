DROP TABLE IF EXISTS savings_accounts_apy_history;
DROP TABLE IF EXISTS savings_accounts;
DROP TABLE IF EXISTS institutions;

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
    terms_and_conditions_source_url TEXT,
    FOREIGN KEY (institution_id) REFERENCES institutions (id)
);

CREATE TABLE savings_accounts_apy_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id INTEGER NOT NULL,
    apy REAL NOT NULL,
    compound_frequency INTEGER NOT NULL DEFAULT 365,
    effective_date DATETIME NOT NULL,
    FOREIGN KEY (account_id) REFERENCES savings_accounts (id),
    UNIQUE(account_id, effective_date)
);

CREATE TABLE risk_free_rate_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    rate REAL NOT NULL,
    effective_date DATETIME NOT NULL
);