import time

from pwn import remote

import factor


e = 0x10001

_r = remote("localhost", 1337)

bit_len = 4096

_r.sendlineafter(": ", "2")
_r.recvline()
n = int(_r.recvline().split()[2])

# guess
p_mask = {}
for c in "fedcba98":
    _r.sendlineafter(": ", "4")
    _r.sendlineafter(": ", c * (bit_len // 4))
    ret = _r.recvline().strip().decode()
    p_mask[c] = ret
q_mask = {}
for c in "fedcba98":
    _r.sendlineafter(": ", "4")
    _r.sendlineafter(": ", c * (bit_len // 4))
    ret = _r.recvline().strip().decode()
    q_mask[c] = ret


# generate bits string describing known bits
p_known = [False] * bit_len
q_known = [False] * bit_len
p_sim = 0
q_sim = 0
for i, c in enumerate("fedcba98"):
    for j, b in enumerate(p_mask[c][::-1]):
        if b == "1":
            for k in range(4):
                p_known[j * 4 + k] = True
            p_sim += 16 ** j * (15 - i)
for i, c in enumerate("fedcba98"):
    for j, b in enumerate(q_mask[c][::-1]):
        if b == "1":
            for k in range(4):
                q_known[j * 4 + k] = True
            q_sim += 16 ** j * (15 - i)
# 8未満なので
for i in range(3, bit_len, 4):
    p_known[i] = True
    q_known[i] = True
p_bits_str = "".join(
    map(lambda x: str(x[0]) if x[1] else "_", zip(bin(p_sim)[2:], p_known[::-1]))
)
q_bits_str = "".join(
    map(lambda x: str(x[0]) if x[1] else "_", zip(bin(q_sim)[2:], q_known[::-1]))
)

now = time.perf_counter()
p_q = factor.from_str(n, p_bits_str, q_bits_str, verbose=False)
print(time.perf_counter() - now)
assert p_q is not None
p, q = p_q
phi = (p - 1) * (q - 1)
d = pow(e, -1, phi)

_r.sendlineafter(": ", "3")
_r.sendlineafter(": ", str(d))
_r.recvline()
print(_r.recvline())

_r.close()
