// use ctrlc;
use once_cell::sync::Lazy;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rug::{ops::Pow, Integer};
use std::collections::VecDeque;

static TWO: Lazy<Integer> = Lazy::new(|| "2".parse().unwrap());

fn bits_to_num(bits: &Vec<i8>, n: i32, saved: &Integer, saved_idx: i32) -> Integer {
    let mut ret = saved.clone();
    for i in (saved_idx as usize + 1)..(n as usize) {
        assert_ne!(bits[i], -1);
        if bits[i] == 1 {
            ret += TWO.clone().pow(i as u32) * bits[i]
        }
    }
    return ret;
}

fn check(p: &Integer, q: &Integer, n: &Integer, m: u32) -> bool {
    let two_pow = TWO.clone().pow(m);
    let mut tmp_n = p.clone();
    tmp_n *= q;
    tmp_n %= &two_pow;
    let mut n_two_pow = n.clone();
    n_two_pow %= &two_pow;
    return tmp_n == n_two_pow;
}

pub fn factor_dfs(
    n: &Integer,
    p_bits: Vec<i8>,
    q_bits: Vec<i8>,
    bit_len: usize,
    verbose: bool,
) -> Option<(Integer, Integer)> {
    let mut p: Integer = "0".parse().unwrap();
    for i in 0..p_bits.len() {
        if p_bits[i] == 1 {
            p += Integer::from(1) << (i as u32);
        }
    }
    let mut q: Integer = "0".parse().unwrap();
    for i in 0..q_bits.len() {
        if q_bits[i] == 1 {
            q += Integer::from(1) << (i as u32);
        }
    }
    fn dfs(
        n: &Integer,
        p: &mut Integer,
        q: &mut Integer,
        p_bits: &Vec<i8>,
        q_bits: &Vec<i8>,
        bit: usize,
        max_bit: usize,
    ) -> bool {
        let mut bit = bit;
        while bit < max_bit {
            if p_bits[bit] == -1 || q_bits[bit] == -1 {
                break;
            }
            bit += 1;
        }
        if bit == max_bit {
            let mut tmp_n = p.clone();
            tmp_n *= q.clone();
            return &tmp_n == n;
        }
        for i in 0..2 {
            for j in 0..2 {
                let mut p_ = Integer::new();
                if p_bits[bit] == -1 && i == 1 {
                    p_.assign(Integer::from(1) << bit as u32);
                } else {
                    p_.assign(0);
                }
                let mut q_ = Integer::new();
                if q_bits[bit] == -1 && j == 1 {
                    q_.assign(Integer::from(1) << bit as u32);
                } else {
                    q_.assign(0);
                }
                *p += &p_;
                *q += &q_;
                let mut tmp_n = p.clone();
                tmp_n *= q.clone();
                tmp_n -= n;
                tmp_n %= Integer::from(1) << (bit + 1) as u32;
                if tmp_n == 0 && dfs(n, p, q, p_bits, q_bits, bit + 1, max_bit) {
                    return true;
                }
                *p -= p_;
                *q -= q_;
            }
        }
        false
    }
    match dfs(n, &mut p, &mut q, &p_bits, &q_bits, 0, bit_len) {
        true => Some((p, q)),
        false => None,
    }
}

pub fn factor_bfs(
    n: &Integer,
    p_bits: Vec<i8>,
    q_bits: Vec<i8>,
    bit_len: usize,
    verbose: bool,
) -> Option<(Integer, Integer)> {
    let mut ans: Option<(Integer, Integer)> = None;
    let mut queue: VecDeque<((Vec<i8>, Integer), (Vec<i8>, Integer), i32)> = VecDeque::new();
    queue.push_back((
        (p_bits, "0".parse().unwrap()),
        (q_bits, "0".parse().unwrap()),
        -1,
    ));
    let mut fixed_idx: i32;
    while queue.len() != 0 {
        let tmp = queue.pop_front().unwrap();
        let (p_bits, saved_p) = tmp.0;
        let (q_bits, saved_q) = tmp.1;
        let saved_idx = tmp.2;
        let mut idx = 0.max(saved_idx);
        while idx < bit_len as i32 {
            if p_bits[idx as usize] == -1 || q_bits[idx as usize] == -1 {
                break;
            }
            idx += 1;
        }
        fixed_idx = idx - 1;
        if verbose {
            print!(
                "\rSearched index: {:<8?} Queue size: {:<8?}",
                fixed_idx,
                queue.len()
            );
        }
        let tmp_p: Integer = bits_to_num(&p_bits, fixed_idx + 1, &saved_p, saved_idx);
        let tmp_q: Integer = bits_to_num(&q_bits, fixed_idx + 1, &saved_q, saved_idx);
        let two_pow = TWO.clone().pow((fixed_idx + 1) as u32);
        let mut tmp_n = tmp_p.clone();
        tmp_n *= &tmp_q;
        if &tmp_n == n {
            ans = Some((tmp_p, tmp_q));
            break;
        }
        tmp_n %= &two_pow;
        let mut n_two_pow = n.clone();
        n_two_pow %= &two_pow;
        if tmp_n != n_two_pow {
            continue;
        }
        if fixed_idx + 1 == bit_len as i32 {
            continue;
        }
        let tmp_p_two_pow = match p_bits[idx as usize] {
            -1 => {
                let mut ret = tmp_p.clone();
                ret += &two_pow;
                ret
            }
            _ => "0".parse().unwrap(),
        };
        let tmp_q_two_pow = match q_bits[idx as usize] {
            -1 => {
                let mut ret = tmp_q.clone();
                ret += &two_pow;
                ret
            }
            _ => "0".parse().unwrap(),
        };
        if (p_bits[idx as usize] == -1) && (q_bits[idx as usize] == -1) {
            if check(&tmp_p, &tmp_q, &n, (fixed_idx + 2) as u32) {
                let mut tmp_p_bits_0 = p_bits.clone();
                tmp_p_bits_0[idx as usize] = 0;
                let mut tmp_q_bits_0 = q_bits.clone();
                tmp_q_bits_0[idx as usize] = 0;
                queue.push_back((
                    (tmp_p_bits_0, tmp_p.clone()),
                    (tmp_q_bits_0, tmp_q.clone()),
                    fixed_idx + 1,
                ));
            }
            if check(&tmp_p, &tmp_q_two_pow, &n, (fixed_idx + 2) as u32) {
                let mut tmp_p_bits_0 = p_bits.clone();
                tmp_p_bits_0[idx as usize] = 0;
                let mut tmp_q_bits_1 = q_bits.clone();
                tmp_q_bits_1[idx as usize] = 1;
                queue.push_back((
                    (tmp_p_bits_0, tmp_p.clone()),
                    (tmp_q_bits_1, tmp_q_two_pow.clone()),
                    fixed_idx + 1,
                ));
            }
            if check(&tmp_p_two_pow, &tmp_q, &n, (fixed_idx + 2) as u32) {
                let mut tmp_p_bits_1 = p_bits.clone();
                tmp_p_bits_1[idx as usize] = 1;
                let mut tmp_q_bits_0 = q_bits.clone();
                tmp_q_bits_0[idx as usize] = 0;
                queue.push_back((
                    (tmp_p_bits_1, tmp_p_two_pow.clone()),
                    (tmp_q_bits_0, tmp_q.clone()),
                    fixed_idx + 1,
                ));
            }
            if check(&tmp_p_two_pow, &tmp_q_two_pow, &n, (fixed_idx + 2) as u32) {
                let mut tmp_p_bits_1 = p_bits.clone();
                tmp_p_bits_1[idx as usize] = 1;
                let mut tmp_q_bits_1 = q_bits.clone();
                tmp_q_bits_1[idx as usize] = 1;
                queue.push_back((
                    (tmp_p_bits_1, tmp_p_two_pow),
                    (tmp_q_bits_1, tmp_q_two_pow),
                    fixed_idx + 1,
                ));
            }
        } else if p_bits[idx as usize] == -1 {
            let tmp_q_next = match q_bits[(fixed_idx + 1) as usize] {
                0 => tmp_q.clone(),
                1 => {
                    let mut ret = tmp_q.clone();
                    ret += two_pow;
                    ret
                }
                _ => panic!(),
            };
            if check(&tmp_p, &tmp_q_next, &n, (fixed_idx + 2) as u32) {
                let mut tmp_p_bits_0 = p_bits.clone();
                tmp_p_bits_0[idx as usize] = 0;
                queue.push_back((
                    (tmp_p_bits_0, tmp_p.clone()),
                    (q_bits.clone(), tmp_q_next.clone()),
                    fixed_idx + 1,
                ));
            }
            if check(&tmp_p_two_pow, &tmp_q_next, &n, (fixed_idx + 2) as u32) {
                let mut tmp_p_bits_1 = p_bits.clone();
                tmp_p_bits_1[idx as usize] = 1;
                queue.push_back((
                    (tmp_p_bits_1, tmp_p_two_pow.clone()),
                    (q_bits.clone(), tmp_q_next.clone()),
                    fixed_idx + 1,
                ));
            }
        } else if q_bits[idx as usize] == -1 {
            let tmp_p_next = match p_bits[(fixed_idx + 1) as usize] {
                0 => tmp_p.clone(),
                1 => {
                    let mut ret = tmp_p.clone();
                    ret += two_pow;
                    ret
                }
                _ => panic!(),
            };
            if check(&tmp_p_next, &tmp_q, &n, (fixed_idx + 2) as u32) {
                let mut tmp_q_bits_0 = q_bits.clone();
                tmp_q_bits_0[idx as usize] = 0;
                queue.push_back((
                    (p_bits.clone(), tmp_p_next.clone()),
                    (tmp_q_bits_0, tmp_q.clone()),
                    fixed_idx + 1,
                ));
            }
            if check(&tmp_p_next, &tmp_q_two_pow, &n, (fixed_idx + 2) as u32) {
                let mut tmp_q_bits_1 = q_bits.clone();
                tmp_q_bits_1[idx as usize] = 1;
                queue.push_back((
                    (p_bits.clone(), tmp_p_next.clone()),
                    (tmp_q_bits_1, tmp_q_two_pow.clone()),
                    fixed_idx + 1,
                ));
            }
        } else {
            panic!("そうはならんやろ");
        }
    }
    if verbose {
        println!();
    }
    ans
}

pub fn factor_core(
    n: &Integer,
    p_bits: Vec<i8>,
    q_bits: Vec<i8>,
    bit_len: usize,
    verbose: bool,
    search: String,
) -> Option<(Integer, Integer)> {
    match search.as_str() {
        "dfs" => factor_dfs(n, p_bits, q_bits, bit_len, verbose),
        _ => factor_bfs(n, p_bits, q_bits, bit_len, verbose),
    }
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

/// from_vector(n_str, p_bits, q_bits)
/// --
///
/// factor `n` from `p` (`list` like `[-1, 1, 0, -1, 0, 1, -1]`) and `q` whose bits are known around more than 50%.
///
/// Args:
///     n_str (str): `n` to be factored. `str` of decimal number.
///     p_bits (list[int]): `list[int]` of `p`'s bits like `[-1, 1, 0, -1, 0, 1, -1]` (big endian).
///     q_bits (list[int]): the same as `p_bits`
/// Returns:
///     (str, str) or None: (p, q) string in decimal or None if not found
/// Examples:
///     >>> import factor
///     >>> factor.from_vector("35", [1, 1, -1], [-1, 0, 1])
///         ('7', '5')
///     >>> factor.from_vector("35", [0, 1, -1], [-1, 0, 1]) is None
///         True
#[pyfunction]
fn from_vector(
    n_str: String,
    p_bits: Vec<i8>,
    q_bits: Vec<i8>,
    verbose: Option<bool>,
) -> PyResult<Option<(String, String)>> {
    // ctrlc::set_handler(|| std::process::exit(2)).unwrap();
    let n: Integer = n_str.parse().unwrap();
    let bit_len = p_bits.len().max(q_bits.len());
    let mut p_bits = p_bits.clone();
    let mut q_bits = q_bits.clone();
    p_bits.reverse();
    q_bits.reverse();
        Some((p, q)) => Ok(Some((p.to_string_radix(10), q.to_string_radix(10)))),
    match factor_core(
        &n,
        p_bits,
        q_bits,
        bit_len,
        verbose.unwrap_or(false),
        search.unwrap_or("bfs".to_string()),
    ) {
        None => Ok(None),
    }
}

/// from_str(n_str, p_bits_str, q_bits_str)
/// --
///
/// factor `n` from `p` (str like "_10_01__1") and `q` whose bits are known around more than 50%.
///
/// Args:
///     n_str (str): `n` to be factored. `str` of decimal number.
///     p_bits_str (str): string of `p`'s bits like "?10?01??1" (big endian).
///     q_bits_str (str): the same as `p_bits_str`
/// Returns:
///     (str, str) or None: (p, q) string in decimal
/// Examples:
///     >>> import factor
///     >>> factor.from_str("35", "11_", "_01")
///         ('7', '5')
///     >>> factor.from_str("35", "11?", "?01")
///         ('7', '5')
///     >>> factor.from_str("35", "01_", "_01") is None
///         True
#[pyfunction]
fn from_str(
    n_str: String,
    p_bits_str: String,
    q_bits_str: String,
    verbose: Option<bool>,
) -> PyResult<Option<(String, String)>> {
    // ctrlc::set_handler(|| std::process::exit(2)).unwrap();
    let n: Integer = n_str.parse().unwrap();
    let bit_len = p_bits_str.len().max(q_bits_str.len());
    let p_bits = str_to_vec(p_bits_str.as_bytes(), bit_len);
    let q_bits = str_to_vec(q_bits_str.as_bytes(), bit_len);
        Some((p, q)) => Ok(Some((p.to_string_radix(10), q.to_string_radix(10)))),
    match factor_core(
        &n,
        p_bits,
        q_bits,
        bit_len,
        verbose.unwrap_or(false),
        search.unwrap_or("bfs".to_string()),
    ) {
        None => Ok(None),
    }
}

#[pymodule]
fn factor(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(from_vector, m)?)?;
    m.add_function(wrap_pyfunction!(from_str, m)?)?;

    Ok(())
}
