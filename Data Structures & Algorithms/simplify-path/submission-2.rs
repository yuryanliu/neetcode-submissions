impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = vec![];
        let path = path.as_bytes();
        let (mut i, mut j) = (0, 0);
        while j < path.len() {
            if path[j] == b'/' {
                if j > i {
                    if &path[i..j] == &[b'.'] {
                        //do nothing
                    } else if &path[i..j] == &[b'.', b'.'] {
                        if !stack.is_empty() {
                            stack.pop();
                        }
                    } else {
                        stack.push(&path[i..j]);
                    }
                }
                i = j + 1;
            }
            j += 1;
        }
        if j > i {
            if &path[i..j] == &[b'.'] {
                //do nothing
            } else if &path[i..j] == &[b'.', b'.'] {
                if !stack.is_empty() {
                    stack.pop();
                }
            } else {
                stack.push(&path[i..j]);
            }
        }
        let mut res = String::new();
        for name in stack {
            res += &("/".to_string() + String::from_utf8(name.to_vec()).unwrap_or_default().as_str());
        }
        if res.is_empty() {
            "/".to_string()
        } else {
            res
        }
    }
}
