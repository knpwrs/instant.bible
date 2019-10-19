pub fn first_char(s: &str) -> Option<char> {
    s.chars().next()
}

pub fn shared_prefix(s1: &str, s2: &str) -> Option<usize> {
    let count = s1
        .chars()
        .zip(s2.chars())
        .take_while(|(c1, c2)| c1 == c2)
        .count();
    if count != 0 {
        Some(count)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{first_char, shared_prefix};
    
    #[test]
    fn test_first_char() {
        let res = first_char("Banana");
        assert_eq!(res, Some('B'));
        let res = first_char("");
        assert_eq!(res, None);
    }

    #[test]
    fn test_shared_prefix() {
        let res = shared_prefix("Fast", "Faster");
        assert_eq!(res, Some(4));
        let res = shared_prefix("Faster", "Toaster");
        assert_eq!(res, None);
    }
}