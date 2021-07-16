mod bfs;
mod dfs;

use rug::Integer;

use crate::factor::bfs::factor_bfs;
use crate::factor::dfs::factor_dfs;

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
