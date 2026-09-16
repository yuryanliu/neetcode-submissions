impl Solution {
    pub fn reverse_bits(n: u32) -> u32 {
        let n = (n >> 16) | (n << 16);
        let n = ((n & 0xff00ff00) >> 8) | ((n & 0x00ff00ff) << 8);
        let n = ((n & 0xf0f0f0f0) >> 4) | ((n & 0x0f0f0f0f) << 4);
        let n = ((n & 0xcccccccc) >> 2) | ((n & 0x33333333) << 2);
        let n = ((n & 0xaaaaaaaa) >> 1) | ((n & 0x55555555) << 1);
        n
    }
}
