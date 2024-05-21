pub fn is_match(s: String, p: String) -> bool {
    let s_chars: Vec<char> = s.chars().collect();
    let p_chars: Vec<char> = p.chars().collect();
    let mut dp = vec![vec![false; p.len() + 1]; 2];

    dp[0][0] = true;
    for j in 2..=p.len() {
        dp[0][j] = dp[0][j - 2] && p_chars[j - 1] == '*';
    }

    let mut curr = 0;
    for i in 1..=s.len() {
        let prev = curr;
        curr = 1 - curr;
        dp[curr][0] = false;
        for j in 1..=p.len() {
            if p_chars[j - 1] == '*' {
                dp[curr][j] = dp[curr][j - 2] || (dp[prev][j] && (s_chars[i - 1] == p_chars[j - 2] || p_chars[j - 2] == '.'));
            } else {
                dp[curr][j] = dp[prev][j - 1] && (s_chars[i - 1] == p_chars[j - 1] || p_chars[j - 1] == '.');
            }
        }
    }

    dp[curr][p.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_match() {
        assert_eq!(is_match("aa".to_string(), "a".to_string()), false);
        assert_eq!(is_match("aa".to_string(), "a*".to_string()), true);
        assert_eq!(is_match("ab".to_string(), ".*".to_string()), true);
        assert_eq!(is_match("aab".to_string(), "c*a*b".to_string()), true);
        assert_eq!(is_match("mississippi".to_string(), "mis*is*p*.".to_string()), false);
    }
}
