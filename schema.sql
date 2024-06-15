DROP TRIGGER IF EXISTS track_savings_account_withdraw_terms_changes;
DROP TRIGGER IF EXISTS track_savings_accounts_apy_changes;
DROP TRIGGER IF EXISTS update_savings_accounts_apy_last_year;

DROP TABLE IF EXISTS savings_accounts_apy_last_year;
DROP TABLE IF EXISTS savings_account_withdraw_terms_history;
DROP TABLE IF EXISTS savings_account_withdraw_types;
DROP TABLE IF EXISTS savings_accounts;
DROP TABLE IF EXISTS institutions;
DROP TABLE IF EXISTS savings_accounts_apy_history;

CREATE TABLE institutions (
    institution_id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    address TEXT,
    phone TEXT,
    website TEXT
);

CREATE TABLE savings_accounts (
    account_id INTEGER PRIMARY KEY AUTOINCREMENT,
    institution_id INTEGER NOT NULL,
    account_type TEXT CHECK(account_type IN ('High-Yield Savings', 'Money Market Accounts', 'Cash Management Accounts')),
    account_name TEXT NOT NULL,
    FOREIGN KEY (institution_id) REFERENCES institutions (institution_id)
);

CREATE TABLE savings_account_withdraw_types (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id INTEGER NOT NULL,
    withdrawal_type TEXT NOT NULL,
    FOREIGN KEY (account_id) REFERENCES savings_accounts (account_id)
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
    FOREIGN KEY (account_id) REFERENCES savings_accounts (account_id)
);

CREATE TABLE savings_accounts_apy_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    apy_id INTEGER NOT NULL,
    account_id INTEGER NOT NULL,
    apy REAL,
    effective_date DATETIME NOT NULL,
    FOREIGN KEY (apy_id) REFERENCES savings_accounts (account_id),
    FOREIGN KEY (account_id) REFERENCES savings_accounts (account_id)
);

CREATE TABLE savings_accounts_apy_last_year (
    account_id INTEGER,
    apy REAL,
    date DATE,
    FOREIGN KEY (account_id) REFERENCES savings_accounts (account_id),
    UNIQUE(account_id, date)
);

INSERT INTO institutions (name, address, phone, website) VALUES
('Bank of America', '100 N Tryon St, Charlotte, NC 28255', '1-800-432-1000', 'https://www.bankofamerica.com'),
('Chase Bank', '270 Park Ave, New York, NY 10017', '1-800-935-9935', 'https://www.chase.com'),
('Wells Fargo', '420 Montgomery St, San Francisco, CA 94104', '1-800-869-3557', 'https://www.wellsfargo.com');

INSERT INTO savings_accounts (institution_id, account_type, account_name) VALUES
(1, 'High-Yield Savings', 'Bank of America High Yield Savings'),
(1, 'Money Market Accounts', 'Bank of America Money Market Account'),
(1, 'Cash Management Accounts', 'Bank of America Cash Management Account'),
(2, 'High-Yield Savings', 'Chase High Yield Savings Account'),
(2, 'Money Market Accounts', 'Chase Money Market Account'),
(2, 'Cash Management Accounts', 'Chase Cash Management Account'),
(3, 'High-Yield Savings', 'Wells Fargo High Yield Savings Account'),
(3, 'Money Market Accounts', 'Wells Fargo Money Market Account'),
(3, 'Cash Management Accounts', 'Wells Fargo Cash Management Account');

INSERT INTO savings_account_withdraw_types (account_id, withdrawal_type) VALUES
(1, 'ATM'),
(1, 'Wire Transfer'),
(1, 'Zelle'),
(1, 'ACH Transfer'),
(1, 'Check Withdrawal'),
(1, 'Online Banking Transfer'),
(1, 'Point of Sale'),
(1, 'Cash Withdrawal at Branch'),
(1, 'Mobile Payment Apps'),
(1, 'Telephone Banking Transfer'),
(1, 'Preauthorized Debit'),

(2, 'ATM'),
(2, 'Wire Transfer'),
(2, 'Zelle'),
(2, 'ACH Transfer'),
(2, 'Check Withdrawal'),
(2, 'Online Banking Transfer'),
(2, 'Point of Sale'),
(2, 'Cash Withdrawal at Branch'),
(2, 'Mobile Payment Apps'),
(2, 'Telephone Banking Transfer'),
(2, 'Preauthorized Debit'),

(3, 'ATM'),
(3, 'Wire Transfer'),
(3, 'Zelle'),
(3, 'ACH Transfer'),
(3, 'Check Withdrawal'),
(3, 'Online Banking Transfer'),
(3, 'Point of Sale'),
(3, 'Cash Withdrawal at Branch'),
(3, 'Mobile Payment Apps'),
(3, 'Telephone Banking Transfer'),
(3, 'Preauthorized Debit');

INSERT INTO savings_account_withdraw_terms_history (withdrawal_type_id, account_id, fee, currency_conversion_fee, additional_fee,
    daily_limit, monthly_limit, notice_period_days, transfer_limit, fee_for_excessive_withdrawals, identification_required,
    additional_restrictions, effective_date) VALUES
(1, 1, 2.50, NULL, 'Out-of-network ATM fee applies', 1000.00, 5000.00, 0, 3000.00, 10.00, 'Yes', 'None', '2023-01-01 00:00:00'),
(2, 1, 30.00, 10.00, 'International wire transfer fee applies', NULL, NULL, 2, 50000.00, 0.00, 'Yes', 'Large transfers may require manager approval', '2023-01-01 00:00:00'),
(3, 1, 0.00, NULL, 'No fee', 1000.00, 5000.00, 0, 2500.00, 0.00, 'No', 'None', '2023-01-01 00:00:00'),
(4, 1, 0.00, NULL, 'No fee', 10000.00, 30000.00, 0, 20000.00, 5.00, 'No', 'None', '2023-01-01 00:00:00'),
(5, 1, 0.00, NULL, 'No fee for account holders', NULL, NULL, 0, NULL, 0.00, 'Yes', 'None', '2023-01-01 00:00:00'),
(6, 1, 0.00, NULL, 'No fee', 10000.00, 50000.00, 0, 20000.00, 0.00, 'No', 'None', '2023-01-01 00:00:00'),
(7, 1, 0.00, NULL, 'No fee for debit transactions', 5000.00, 20000.00, 0, 5000.00, 0.00, 'Yes', 'None', '2023-01-01 00:00:00'),
(8, 1, 0.00, NULL, 'No fee', NULL, NULL, 1, NULL, 0.00, 'Yes', 'May require advance notice for large amounts', '2023-01-01 00:00:00'),
(9, 1, 0.00, NULL, 'Fees may apply for instant transfers', 2000.00, 10000.00, 0, 5000.00, 1.00, 'No', 'Fees for instant transfers may apply', '2023-01-01 00:00:00'),
(10, 1, 0.00, NULL, 'No fee', 5000.00, 20000.00, 0, 10000.00, 2.00, 'Yes', 'None', '2023-01-01 00:00:00'),
(11, 1, 0.00, NULL, 'No fee', NULL, NULL, 0, NULL, 0.00, 'No', 'None', '2023-01-01 00:00:00'),

(12, 2, 2.50, NULL, 'Out-of-network ATM fee applies', 1500.00, 6000.00, 0, 3500.00, 12.00, 'Yes', 'None', '2023-01-01 00:00:00'),
(13, 2, 25.00, 8.00, 'International wire transfer fee applies', NULL, NULL, 1, 40000.00, 0.00, 'Yes', 'Large transfers may require manager approval', '2023-01-01 00:00:00'),
(14, 2, 0.00, NULL, 'No fee', 1200.00, 5200.00, 0, 2000.00, 0.00, 'No', 'None', '2023-01-01 00:00:00'),
(15, 2, 0.00, NULL, 'No fee', 15000.00, 40000.00, 0, 25000.00, 6.00, 'No', 'None', '2023-01-01 00:00:00'),
(16, 2, 0.00, NULL, 'No fee for account holders', NULL, NULL, 0, NULL, 0.00, 'Yes', 'None', '2023-01-01 00:00:00'),
(17, 2, 0.00, NULL, 'No fee', 15000.00, 60000.00, 0, 30000.00, 0.00, 'No', 'None', '2023-01-01 00:00:00'),
(18, 2, 0.00, NULL, 'No fee for debit transactions', 6000.00, 25000.00, 0, 6000.00, 0.00, 'Yes', 'None', '2023-01-01 00:00:00'),
(19, 2, 0.00, NULL, 'No fee', NULL, NULL, 1, NULL, 0.00, 'Yes', 'May require advance notice for large amounts', '2023-01-01 00:00:00'),
(20, 2, 0.00, NULL, 'Fees may apply for instant transfers', 2500.00, 15000.00, 0, 6000.00, 1.50, 'No', 'Fees for instant transfers may apply', '2023-01-01 00:00:00'),
(21, 2, 0.00, NULL, 'No fee', 6000.00, 25000.00, 0, 12000.00, 2.50, 'Yes', 'None', '2023-01-01 00:00:00'),
(22, 2, 0.00, NULL, 'No fee', NULL, NULL, 0, NULL, 0.00, 'No', 'None', '2023-01-01 00:00:00'),

(23, 3, 2.50, NULL, 'Out-of-network ATM fee applies', 1200.00, 5500.00, 0, 3200.00, 15.00, 'Yes', 'None', '2023-01-01 00:00:00'),
(24, 3, 35.00, 12.00, 'International wire transfer fee applies', NULL, NULL, 3, 60000.00, 0.00, 'Yes', 'Large transfers may require manager approval', '2023-01-01 00:00:00'),
(25, 3, 0.00, NULL, 'No fee', 900.00, 4500.00, 0, 1500.00, 0.00, 'No', 'None', '2023-01-01 00:00:00'),
(26, 3, 0.00, NULL, 'No fee', 12000.00, 35000.00, 0, 22000.00, 5.50, 'No', 'None', '2023-01-01 00:00:00'),
(27, 3, 0.00, NULL, 'No fee for account holders', NULL, NULL, 0, NULL, 0.00, 'Yes', 'None', '2023-01-01 00:00:00'),
(28, 3, 0.00, NULL, 'No fee', 12000.00, 55000.00, 0, 25000.00, 0.00, 'No', 'None', '2023-01-01 00:00:00'),
(29, 3, 0.00, NULL, 'No fee for debit transactions', 5500.00, 23000.00, 0, 5500.00, 0.00, 'Yes', 'None', '2023-01-01 00:00:00'),
(30, 3, 0.00, NULL, 'No fee', NULL, NULL, 1, NULL, 0.00, 'Yes', 'May require advance notice for large amounts', '2023-01-01 00:00:00'),
(31, 3, 0.00, NULL, 'Fees may apply for instant transfers', 1800.00, 12000.00, 0, 5500.00, 1.25, 'No', 'Fees for instant transfers may apply', '2023-01-01 00:00:00'),
(32, 3, 0.00, NULL, 'No fee', 5500.00, 22000.00, 0, 11000.00, 2.25, 'Yes', 'None', '2023-01-01 00:00:00'),
(33, 3, 0.00, NULL, 'No fee', NULL, NULL, 0, NULL, 0.00, 'No', 'None', '2023-01-01 00:00:00');

INSERT INTO savings_accounts_apy_history (apy_id, account_id, apy, effective_date) VALUES
(1, 1, 0.01, '2023-01-01 00:00:00'),
(1, 1, 0.012, '2023-02-01 00:00:00'),
(1, 1, 0.011, '2023-03-01 00:00:00'),
(1, 1, 0.013, '2023-04-01 00:00:00'),
(1, 1, 0.012, '2023-05-01 00:00:00'),
(1, 1, 0.011, '2023-06-01 00:00:00'),
(1, 1, 0.01, '2023-07-01 00:00:00'),
(1, 1, 0.012, '2023-08-01 00:00:00'),
(1, 1, 0.011, '2023-09-01 00:00:00'),
(1, 1, 0.013, '2023-10-01 00:00:00'),
(1, 1, 0.01, '2023-11-01 00:00:00'),
(1, 1, 0.012, '2023-12-01 00:00:00'),

(2, 2, 0.015, '2023-01-01 00:00:00'),
(2, 2, 0.014, '2023-02-01 00:00:00'),
(2, 2, 0.013, '2023-03-01 00:00:00'),
(2, 2, 0.015, '2023-04-01 00:00:00'),
(2, 2, 0.014, '2023-05-01 00:00:00'),
(2, 2, 0.013, '2023-06-01 00:00:00'),
(2, 2, 0.015, '2023-07-01 00:00:00'),
(2, 2, 0.014, '2023-08-01 00:00:00'),
(2, 2, 0.013, '2023-09-01 00:00:00'),
(2, 2, 0.015, '2023-10-01 00:00:00'),
(2, 2, 0.014, '2023-11-01 00:00:00'),
(2, 2, 0.013, '2023-12-01 00:00:00'),

(3, 3, 0.02, '2023-01-01 00:00:00'),
(3, 3, 0.018, '2023-02-01 00:00:00'),
(3, 3, 0.019, '2023-03-01 00:00:00'),
(3, 3, 0.02, '2023-04-01 00:00:00'),
(3, 3, 0.018, '2023-05-01 00:00:00'),
(3, 3, 0.019, '2023-06-01 00:00:00'),
(3, 3, 0.02, '2023-07-01 00:00:00'),
(3, 3, 0.018, '2023-08-01 00:00:00'),
(3, 3, 0.019, '2023-09-01 00:00:00'),
(3, 3, 0.02, '2023-10-01 00:00:00'),
(3, 3, 0.018, '2023-11-01 00:00:00'),
(3, 3, 0.019, '2023-12-01 00:00:00');

INSERT INTO savings_accounts_apy_last_year (account_id, apy, date) VALUES
(1, 0.01, '2023-01-01'),
(1, 0.012, '2023-02-01'),
(1, 0.011, '2023-03-01'),
(1, 0.013, '2023-04-01'),
(1, 0.012, '2023-05-01'),
(1, 0.011, '2023-06-01'),
(1, 0.01, '2023-07-01'),
(1, 0.012, '2023-08-01'),
(1, 0.011, '2023-09-01'),
(1, 0.013, '2023-10-01'),
(1, 0.01, '2023-11-01'),
(1, 0.012, '2023-12-01'),

(2, 0.015, '2023-01-01'),
(2, 0.014, '2023-02-01'),
(2, 0.013, '2023-03-01'),
(2, 0.015, '2023-04-01'),
(2, 0.014, '2023-05-01'),
(2, 0.013, '2023-06-01'),
(2, 0.015, '2023-07-01'),
(2, 0.014, '2023-08-01'),
(2, 0.013, '2023-09-01'),
(2, 0.015, '2023-10-01'),
(2, 0.014, '2023-11-01'),
(2, 0.013, '2023-12-01'),

(3, 0.02, '2023-01-01'),
(3, 0.018, '2023-02-01'),
(3, 0.019, '2023-03-01'),
(3, 0.02, '2023-04-01'),
(3, 0.018, '2023-05-01'),
(3, 0.019, '2023-06-01'),
(3, 0.02, '2023-07-01'),
(3, 0.018, '2023-08-01'),
(3, 0.019, '2023-09-01'),
(3, 0.02, '2023-10-01'),
(3, 0.018, '2023-11-01'),
(3, 0.019, '2023-12-01');
