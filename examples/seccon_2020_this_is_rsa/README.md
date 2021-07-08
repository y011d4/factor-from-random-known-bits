```ruby
require 'openssl'

def get_prime
  i = OpenSSL::BN.rand(512).to_s.unpack1('H*').hex
  OpenSSL::BN.new(i).prime? ? i : get_prime
end

p = get_prime
q = get_prime
n = p * q
e = 65537
m = File.read('flag.txt').unpack1('H*').hex
c = m.pow(e, n)

puts "N = #{n}"
puts "c = #{c}"
```

`OpenSSL::BN.rand(512).to_s.unpack1('H*').hex` returns `0x3_3_3_...`.
So we know that `p` and `q` are `[..., 0, 0, 1, 1, ?, ?, ?, ?, 0, 0, 1, 1, ?, ?, ?, ?, ..., 0, 0, 1, 1, ?, ?, ?, ?]`
