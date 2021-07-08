# Run challenge server
```bash
socat tcp4-listen:1337,reuseaddr,fork "system:python3 server.py"
```
This `server.py` is modified to use fixed primes generated in advance because `getPrime(4096)` takes too much time...

# Writeup
The payload `88...88`, `99...99`, ..., `ff...ff` reveals about the half of the bits of `p` and `q`. We also know that unknown positions' MSB is 0. So over 50% bits are known. This is enough to factor.

# Run solver
```bash
python solver.py
```
