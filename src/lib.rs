// use ctrlc;
pub mod factor;
pub mod util;

use num_bigint::BigInt;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rug::Integer;

use factor::factor_core;
use util::str_to_vec;

/// from_vector(n, p_bits, q_bits, verbose=False, search="bfs")
/// --
///
/// factor `n` from `p` (`list` like `[-1, 1, 0, -1, 0, 1, -1]`) and `q` whose bits are known around more than 50%.
///
/// Args:
///     n (int): `n` to be factored. `str` of decimal number.
///     p_bits (list[int]): `list[int]` of `p`'s bits like `[-1, 1, 0, -1, 0, 1, -1]` (big endian).
///     q_bits (list[int]): the same as `p_bits`
/// Returns:
///     (int, int) or None: (p, q) such that n == p * q.
/// Examples:
///     >>> import factor
///     >>> factor.from_vector(35, [1, 1, -1], [-1, 0, 1])
///         (7, 5)
///     >>> factor.from_vector(35, [0, 1, -1], [-1, 0, 1]) is None
///         True
#[pyfunction]
fn from_vector(
    n: BigInt,
    p_bits: Vec<i8>,
    q_bits: Vec<i8>,
    verbose: Option<bool>,
    search: Option<String>,
) -> PyResult<Option<(BigInt, BigInt)>> {
    // ctrlc::set_handler(|| std::process::exit(2));
    let n: Integer = n.to_str_radix(10).parse().unwrap();
    let bit_len = p_bits.len().max(q_bits.len());
    let mut p_bits = p_bits.clone();
    let mut q_bits = q_bits.clone();
    p_bits.reverse();
    q_bits.reverse();
    match factor_core(
        &n,
        p_bits,
        q_bits,
        bit_len,
        verbose.unwrap_or(false),
        search.unwrap_or("bfs".to_string()),
    ) {
        Some((p, q)) => Ok(Some((
            p.to_string_radix(10).parse().unwrap(),
            q.to_string_radix(10).parse().unwrap(),
        ))),
        None => Ok(None),
    }
}

/// from_str(n, p_bits_str, q_bits_str, verbose=False, search="bfs")
/// --
///
/// factor `n` from `p` (str like "_10_01__1") and `q` whose bits are known around more than 50%.
///
/// Args:
///     n (int): `n` to be factored.
///     p_bits_str (str): string of `p`'s bits like "?10?01??1" (big endian).
///     q_bits_str (str): the same as `p_bits_str`
/// Returns:
///     (int, int) or None: (p, q) such that n == p * q.
/// Examples:
///     >>> import factor
///     >>> factor.from_str(35, "11_", "_01")
///         (7, 5)
///     >>> factor.from_str(35, "11?", "?01")
///         (7, 5)
///     >>> factor.from_str(35, "01_", "_01") is None
///         True
#[pyfunction]
fn from_str(
    n: BigInt,
    p_bits_str: String,
    q_bits_str: String,
    verbose: Option<bool>,
    search: Option<String>,
) -> PyResult<Option<(BigInt, BigInt)>> {
    // ctrlc::set_handler(|| std::process::exit(2));
    let n: Integer = n.to_str_radix(10).parse().unwrap();
    let bit_len = p_bits_str.len().max(q_bits_str.len());
    let p_bits = str_to_vec(p_bits_str.as_bytes(), bit_len);
    let q_bits = str_to_vec(q_bits_str.as_bytes(), bit_len);
    match factor_core(
        &n,
        p_bits,
        q_bits,
        bit_len,
        verbose.unwrap_or(false),
        search.unwrap_or("bfs".to_string()),
    ) {
        Some((p, q)) => Ok(Some((
            p.to_string_radix(10).parse().unwrap(),
            q.to_string_radix(10).parse().unwrap(),
        ))),
        None => Ok(None),
    }
}

#[pymodule]
fn factor(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(from_vector, m)?)?;
    m.add_function(wrap_pyfunction!(from_str, m)?)?;

    Ok(())
}
