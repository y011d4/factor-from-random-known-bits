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
    // assert!(bits_str.len() <= bit_len);
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
