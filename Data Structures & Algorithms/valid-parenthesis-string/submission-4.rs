impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut cmin = 0;
        let mut cmax = 0;
        for c in s.chars() {
            match c {
                '(' => {
                    cmin += 1;
                    cmax += 1;
                }
                ')' => {
                    cmin -= 1;
                    cmax -= 1;
                }
                '*' => {
                    cmin -= 1;
                    cmax += 1;
                }
                _ => {}
            }
            if cmax < 0 {
                return false;
            }
            if cmin < 0 {
                cmin = 0;
            }
        }
        cmin == 0
    }
}