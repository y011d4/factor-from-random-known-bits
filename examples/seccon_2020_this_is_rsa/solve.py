import time

import factor


e = 65537
N = 13234306273608973531555502334446720401597326792644624514228362685813698571322410829494757436628326246629203126562441757712029708148508660279739210512110734001019285095467352938553972438629039005820507697493315650840705745518918873979766056584458077636454673830866061550714002346318865318536544606580475852690351622415519854730947773248376978689711597597169469401661488756669849772658771813742926651925442468141895198767553183304485662688033274567173210826233405235701905642383704395846192587563843422713499468379304400363773291993404144432403315463931374682824546730098380872658106314368520370995385913965019067624762624652495458399359096083188938802975032297056646831904294336374652136926975731836556951432035301855715375295216481079863945383657
c = 9094564357254217771457579638296343398667095069849711922513911147179424647045593821415928967849073271368133854458732106409023539482401316282328817488781771665657515880026432487444729168909088425021111879152492812216384426360971681055941907554538267523250780508925995498013624610554177330113234686073838491261974164065812534037687990653834520243512128393881497418722817552604416319729143988970277812550536939775865310487081108925130229024749287074763499871216498398695877450736179371920963283041212502898938555288461797406895266037211533065670904218278235604002573401193114111627382958428536968266964975362791704067660270952933411608299947663325963289383426020609754934510085150774508301734516652467839087341415815719569669955613063226205647580528

bit_len = 8 * len(str(2 ** 512 - 1))
p_bits = [-1] * bit_len
q_bits = [-1] * bit_len

i = 0
while i < bit_len:
    if i % 8 == 4 or i % 8 == 5:
        tmp = 1
    elif i % 8 == 6 or i % 8 == 7:
        tmp = 0
    else:
        tmp = -1
    p_bits[bit_len - 1 - i] = tmp
    q_bits[bit_len - 1 - i] = tmp
    i += 1

now = time.perf_counter()
p_q = factor.from_vector(N, p_bits, q_bits, search="dfs")
print(time.perf_counter() - now)
now = time.perf_counter()
p_q = factor.from_vector(N, p_bits, q_bits, search="bfs")
print(time.perf_counter() - now)
assert p_q is not None
p, q = p_q

phi = (p - 1) * (q - 1)
d = pow(e, -1, phi)
m = pow(c, d, N)
print(bytes.fromhex(f"{m:x}"))
