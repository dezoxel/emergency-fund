use std::str::FromStr;

pub fn comma_str_to_vec<T: FromStr>(s: &str) -> Result<Vec<T>, T::Err> {
    s.split(',')
        .map(|item| item.trim().parse::<T>())
        .collect()
}

pub fn vec_to_comma_str<T: ToString>(v: &[T]) -> String {
    v.iter()
        .map(|item| item.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}