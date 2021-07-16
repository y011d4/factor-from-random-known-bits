use once_cell::sync::Lazy;
use rug::{ops::Pow, Integer};

pub static TWO: Lazy<Integer> = Lazy::new(|| "2".parse().unwrap());

pub fn bits_to_num(bits: &Vec<i8>, n: i32, saved: &Integer, saved_idx: i32) -> Integer {
    let mut ret = saved.clone();
    for i in (saved_idx as usize + 1)..(n as usize) {
        assert_ne!(bits[i], -1);
        if bits[i] == 1 {
            ret += TWO.clone().pow(i as u32) * bits[i]
        }
    }
    return ret;
}

pub fn str_to_vec(bits_str: &[u8], bit_len: usize) -> Vec<i8> {
    assert!(bits_str.len() <= bit_len);
    let mut bits: Vec<i8> = vec![];
    for i in 0..bit_len {
        if bits_str.len() < i + 1 {
            bits.push(0);
            continue;
        }
        match bits_str[bits_str.len() - i - 1] as char {
            '0' => bits.push(0),
            '1' => bits.push(1),
            _ => bits.push(-1),
        };
    }
    bits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_to_num() {
        let bits: Vec<i8> = vec![1, 0, 1, 0, 1];
        let n = 3;
        let saved: Integer = "1".parse().unwrap();
        let saved_idx = 1;
        let expected: Integer = "5".parse().unwrap();
        assert_eq!(bits_to_num(&bits, n, &saved, saved_idx), expected);

        let n = 4;
        let saved: Integer = "1".parse().unwrap();
        let saved_idx = 1;
        let expected: Integer = "5".parse().unwrap();
        assert_eq!(bits_to_num(&bits, n, &saved, saved_idx), expected);

        let n = 5;
        let saved: Integer = "1".parse().unwrap();
        let saved_idx = 1;
        let expected: Integer = "21".parse().unwrap();
        assert_eq!(bits_to_num(&bits, n, &saved, saved_idx), expected);
    }

    #[test]
    fn test_str_to_vec() {
        assert_eq!(str_to_vec("101".as_bytes(), 3), [1, 0, 1]);
        assert_eq!(str_to_vec("101".as_bytes(), 4), [1, 0, 1, 0]);
        assert_eq!(str_to_vec("101".as_bytes(), 5), [1, 0, 1, 0, 0]);

        assert_eq!(str_to_vec("_01".as_bytes(), 3), [1, 0, -1]);
        assert_eq!(str_to_vec("?01".as_bytes(), 3), [1, 0, -1]);
    }

    #[test]
    #[should_panic]
    fn test_str_to_vec_panic() {
        str_to_vec("101".as_bytes(), 2);
    }
}
