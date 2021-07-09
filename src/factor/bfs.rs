use crate::util::{bits_to_num, TWO};
use rug::{ops::Pow, Integer};
use std::collections::VecDeque;

fn check(p: &Integer, q: &Integer, n: &Integer, m: u32) -> bool {
    let two_pow = TWO.clone().pow(m);
    let mut tmp_n = p.clone();
    tmp_n *= q;
    tmp_n %= &two_pow;
    let mut n_two_pow = n.clone();
    n_two_pow %= &two_pow;
    return tmp_n == n_two_pow;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factor() {
        let n: Integer = "323".parse().unwrap();
        let p_bits = vec![-1, -1, -1, -1, -1]; // NNNNN
        let q_bits = vec![-1, 1, -1, -1, -1]; // NNN1N
        let bit_len = 5;
        let expected: Option<(Integer, Integer)> =
            Some(("17".parse().unwrap(), "19".parse().unwrap()));
        assert_eq!(
            factor_bfs(&n, p_bits.clone(), q_bits.clone(), bit_len, false),
            expected
        );
    }
}
