#![feature(test)]
use rug::Integer;
use std::env;
extern crate test;
mod lib;
use lib::{factor_core, str_to_vec};

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: Integer = args[1].parse().unwrap();
    let p_bits_str = args[2].as_bytes();
    let q_bits_str = args[3].as_bytes();
    let bit_len = p_bits_str.len().max(q_bits_str.len());
    let p_bits = str_to_vec(p_bits_str, bit_len);
    let q_bits = str_to_vec(q_bits_str, bit_len);
    assert_eq!(p_bits.len(), q_bits.len());
    match factor_core(&n, p_bits, q_bits, bit_len, false, "bfs".to_string()) {
        Some((p, q)) => println!("{}, {}", p, q),
        None => println!("Not found"),
    };
}

mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_factor() {
        let n: Integer = "323".parse().unwrap();
        let p_bits = vec![-1, -1, -1, -1, -1]; // NNNNN
        let q_bits = vec![-1, 1, -1, -1, -1]; // NNN1N
        let bit_len = 5;
        let expected: Option<(Integer, Integer)> =
            Some(("17".parse().unwrap(), "19".parse().unwrap()));
        assert_eq!(
            factor_core(&n, p_bits.clone(), q_bits.clone(), bit_len, false, "bfs".to_string()),
            expected
        );
    }

    #[bench]
    fn bench_factor(b: &mut Bencher) {
        // 110,366,524 ns/iter (+/- 8,439,610)
        //   8,601,455 ns/iter (+/- 263,670)  bits_to_num に saved_p を導入
        //   7,060,782 ns/iter (+/- 606,827) 細かい部分で Integer を再利用
        let n: Integer = "104158954646372695568095796310479805403678314919693272509836778997179683485437763692891984254171869987446475357518587344178264028334102088429629785065036660148146855007349113784322098795994839040721664806905084554147298456659074384855277678993200563966327086005547016327991986225930798076081014377904788085807".parse().unwrap();
        let bit_len = 512;
        let p_bits = str_to_vec("1010101111000NNN11000NNN11100NNN11010NNN0NNN0NNN0NNN100110000NNN0NNN0NNN0NNN11000NNN0NNN0NNN110110010NNN11001100100111010NNN100011000NNN0NNN0NNN0NNN11111000111111100NNN1101110010000NNN0NNN0NNN0NNN10110NNN0NNN0NNN0NNN0NNN0NNN1100101111000NNN0NNN1001111011110NNN0NNN10000NNN0NNN0NNN11010NNN1010101110110NNN0NNN0NNN0NNN0NNN10010NNN1011101011100NNN110111010NNN0NNN0NNN0NNN101010110NNN0NNN10000NNN1000101011000NNN0NNN0NNN0NNN101010000NNN11010NNN111010000NNN0NNN11110NNN0NNN10010NNN111010010NNN0NNN0NNN10100NNN0NNN0NNN".as_bytes(), bit_len);
        let q_bits = str_to_vec("110111010NNN111011110NNN0NNN1000100110001110100111100NNN0NNN10110NNN11000NNN0NNN10110NNN11100NNN10000NNN0NNN0NNN11111100110010100NNN10000NNN11100NNN0NNN110010110NNN101110010NNN10010NNN11110NNN11110NNN0NNN1101111011000NNN101010110NNN10100NNN0NNN10100NNN1010101011010NNN0NNN0NNN100110110NNN0NNN10000NNN0NNN0NNN1000101110010NNN1111110010110NNN0NNN0NNN0NNN101110100NNN0NNN1100101111000NNN10100NNN0NNN0NNN0NNN0NNN0NNN0NNN10010NNN0NNN0NNN10100NNN10010NNN0NNN0NNN0NNN101011110NNN0NNN111110000NNN0NNN11110NNN0NNN10100NNN".as_bytes(), bit_len);
        b.iter(|| factor_core(&n, p_bits.clone(), q_bits.clone(), bit_len, false, "bfs".to_string()));
    }
}
