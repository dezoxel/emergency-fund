use std::str::FromStr;

pub fn comma_str_to_vec<T: FromStr>(s: &str) -> Result<Vec<T>, T::Err> {
    if s.trim().is_empty() {
        return Ok(Vec::new());
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::ParseIntError;

    #[test]
    fn converts_to_numbers() -> Result<(), ParseIntError> {
        assert_eq!(comma_str_to_vec::<i32>("1,2,3")?, vec![1, 2, 3]);
        Ok(())
    }

    #[test]
    fn converts_empty_str_to_empty_vec() -> Result<(), ParseIntError> {
        assert_eq!(comma_str_to_vec::<i32>("")?, Vec::<i32>::new());
        Ok(())
    }

    #[test]
    fn throws_if_trailing_comma() -> Result<(), ParseIntError> {
        let result = comma_str_to_vec::<i32>("1,");
        if let Err(e) = result {
            assert!(matches!(e, ParseIntError { .. } ));
        }
        Ok(())
    }

    #[test]
    fn throws_if_only_comma() -> Result<(), ParseIntError> {
        let result = comma_str_to_vec::<i32>(",");
        if let Err(e) = result {
            assert!(matches!(e, ParseIntError { .. } ));
        }
        Ok(())
    }

    #[test]
    fn throws_if_not_numbers() -> Result<(), ParseIntError> {
        let result = comma_str_to_vec::<i32>("a,b,c");
        if let Err(e) = result {
            assert!(matches!(e, ParseIntError { .. } ));
        }
        Ok(())
    }
}