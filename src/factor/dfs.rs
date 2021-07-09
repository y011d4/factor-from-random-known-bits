use rug::{Assign, Integer};

pub fn factor_dfs(
    n: &Integer,
    p_bits: Vec<i8>,
    q_bits: Vec<i8>,
    bit_len: usize,
    _verbose: bool,
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
            factor_dfs(&n, p_bits.clone(), q_bits.clone(), bit_len, false),
            expected
        );
    }
}
