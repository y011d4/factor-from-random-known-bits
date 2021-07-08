# Factor from random known bits

This is a Python's library written in Rust to quickly factor `n = pq` when around >50% bits of `p` and `q` are known which are distributed at random.
(Used mainly for CTF challenges, especially about RSA)

```python
>>> import factor
>>> factor.from_str("35", "11?", "?01")
    ('7', '5')
>>> factor.from_vector("35", [1, 1, -1], [-1, 0, 1])
    ('7', '5')
>>> n_str = "104158954646372695568095796310479805403678314919693272509836778997179683485437763692891984254171869987446475357518587344178264028334102088429629785065036660148146855007349113784322098795994839040721664806905084554147298456659074384855277678993200563966327086005547016327991986225930798076081014377904788085807"
>>> p_bits_str = "1010101111000___11000___11100___11010___0___0___0___100110000___0___0___0___11000___0___0___110110010___11001100100111010___100011000___0___0___0___11111000111111100___1101110010000___0___0___0___10110___0___0___0___0___0___1100101111000___0___1001111011110___0___10000___0___0___11010___1010101110110___0___0___0___0___10010___1011101011100___110111010___0___0___0___101010110___0___10000___1000101011000___0___0___0___101010000___11010___111010000___0___11110___0___10010___111010010___0___0___10100___0___0___"
>>> q_bits_str = "110111010___111011110___0___1000100110001110100111100___0___10110___11000___0___10110___11100___10000___0___0___11111100110010100___10000___11100___0___110010110___101110010___10010___11110___11110___0___1101111011000___101010110___10100___0___10100___1010101011010___0___0___100110110___0___10000___0___0___1000101110010___1111110010110___0___0___0___101110100___0___1100101111000___10100___0___0___0___0___0___0___10010___0___0___10100___10010___0___0___0___101011110___0___111110000___0___11110___0___10100___"
>>> ans = factor.from_str(n_str, p_bits_str, q_bits_str)
>>> assert ans is not None
>>> p, q = map(int, ans)
>>> assert p * q == int(n_str)
```

In addition, `cargo run` works.
```bash
$ cargo run --release 3233 1_0__1 1___01
53, 61
```

## Install

```bash
pip install -r requirements.txt
python setup.py install
```

## Examples in CTF

There are some examples in `examples` directory.
- "This is RSA" in SECCON CTF 2020
- "regulus-calendula" in HSCTF 2021

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
