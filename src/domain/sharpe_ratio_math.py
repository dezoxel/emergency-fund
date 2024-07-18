import numpy as np
from typing import Tuple, List

def sharpe_ratio_formula(R_p: float, R_f: float, sigma_p: float) -> float:
    return (R_p - R_f) / sigma_p

def calc_sharpe_ratio_statistics(returns: List, risk_free_rates: List) -> Tuple[float, float, float]:
    R_p = np.mean(returns)
    R_f = np.mean(risk_free_rates)
    sigma_p = np.std(returns, ddof=1)
    return R_p, R_f, sigma_p

def calc_sharpe_ratio(returns: List, risk_free_rates: List) -> float:
    R_p, R_f, sigma_p = calc_sharpe_ratio_statistics(returns, risk_free_rates)
    return sharpe_ratio_formula(R_p, R_f, sigma_p)
