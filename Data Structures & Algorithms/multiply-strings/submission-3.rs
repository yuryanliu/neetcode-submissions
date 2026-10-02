impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }

        let mut res = vec![0i32; num1.len() + num2.len()];
        let n1: Vec<u32> = num1.bytes().rev().map(|b| (b - b'0') as u32).collect();
        let n2: Vec<u32> = num2.bytes().rev().map(|b| (b - b'0') as u32).collect();

        for i1 in 0..n1.len() {
            for i2 in 0..n2.len() {
                let digit = (n1[i1] * n2[i2]) as i32;
                res[i1 + i2] += digit;
                res[i1 + i2 + 1] += res[i1 + i2] / 10;
                res[i1 + i2] %= 10;
            }
        }

        let mut i = res.len() as i32 - 1;
        while i >= 0 && res[i as usize] == 0 {
            i -= 1;
        }

        let mut result = String::new();
        while i >= 0 {
            result.push((b'0' + res[i as usize] as u8) as char);
            i -= 1;
        }
        result
    }
}
