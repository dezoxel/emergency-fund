import numpy as np

def sharpe_ratio_formula(R_p, R_f, sigma_p):
    return (R_p - R_f) / sigma_p

def calc_sharpe_ratio_statistics(returns, risk_free_rates):
    R_p = np.mean(returns)
    R_f = np.mean(risk_free_rates)
    sigma_p = np.std(returns, ddof=1)
    return R_p, R_f, sigma_p

def calc_sharpe_ratio(returns, risk_free_rates):
    R_p, R_f, sigma_p = calc_sharpe_ratio_statistics(returns, risk_free_rates)
    return sharpe_ratio_formula(R_p, R_f, sigma_p)
