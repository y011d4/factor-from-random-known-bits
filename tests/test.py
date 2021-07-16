import unittest

import factor


class TestFactor(unittest.TestCase):
    def test_from_vector(self):
        p_q = factor.from_vector(91, [1, 1, 0, -1], [0, -1, -1, 1])
        assert p_q is not None
        p, q = p_q
        assert p == 13
        assert q == 7

        # obviously fail
        p_q = factor.from_vector(1337, [0, 0], [0, 0])
        assert p_q is None

    def test_from_str(self):
        p_q = factor.from_str(91, "110_", "0__1")
        assert p_q is not None
        p, q = p_q
        assert p == 13
        assert q == 7

        # obviously fail
        p_q = factor.from_str(1337, "00", "00")
        assert p_q is None


if __name__ == "__main__":
    unittest.main()
