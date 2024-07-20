def calc_compound_future_value(P, r, n, t):
    A = P * (1 + r/n)**(n * t)
    return A

def calc_annual_rate_by_APY(n, APY):
    r_a = n * ((1 + APY)**(1 / n) - 1)
    return r_a

def calc_return_rate(value_begin, value_end):
    return_rate = (value_end - value_begin) / value_begin
    return return_rate