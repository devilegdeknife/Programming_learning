pub fn is_number(s: String) -> bool {
    enum State {
        Start,
        Sign,
        Integer,
        Point,
        PointWithoutInt,
        Fraction,
        Exp,
        ExpSign,
        ExpNum,
        End,
    }
    use State::*;
    let mut state = Start;
    for c in s.chars() {
        match c {
            ' ' => match state {
                Start => state = Start,
                Integer => state = End,
                Point => state = End,
                Fraction => state = End,
                ExpNum => state = End,
                _ => return false,
            },
            '+' | '-' => match state {
                Start => state = Sign,
                Exp => state = ExpSign,
                _ => return false,
            },
            '.' => match state {
                Start | Sign => state = PointWithoutInt,
                Integer => state = Point,
                _ => return false,
            },
            'e' | 'E' => match state {
                Integer | Fraction | Point => state = Exp,
                _ => return false,
            },
            _ => match c.is_digit(10) {
                true => match state {
                    Start | Sign => state = Integer,
                    Integer => state = Integer,
                    Point | PointWithoutInt => state = Fraction,
                    Exp | ExpSign => state = ExpNum,
                    _ => return false,
                },
                false => return false,
            },
        }
    }
    match state {
        Integer | Fraction | Point | ExpNum | End => true,
        _ => false,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_number() {
        assert_eq!(is_number("2".to_string()), true);
        assert_eq!(is_number("0089".to_string()), true);
        assert_eq!(is_number("-0.1".to_string()), true);
        assert_eq!(is_number("+3.14".to_string()), true);
        assert_eq!(is_number("4.".to_string()), true);
        assert_eq!(is_number("-.9".to_string()), true);
        assert_eq!(is_number("2e10".to_string()), true);
        assert_eq!(is_number("-90E3".to_string()), true);
        assert_eq!(is_number("3e+7".to_string()), true);
        assert_eq!(is_number("+6e-1".to_string()), true);
        assert_eq!(is_number("53.5e93".to_string()), true);
        assert_eq!(is_number("-123.456e789".to_string()), true);
        assert_eq!(is_number("abc".to_string()), false);
        assert_eq!(is_number("1a".to_string()), false);
        assert_eq!(is_number("1e".to_string()), false);
        assert_eq!(is_number("e3".to_string()), false);
        assert_eq!(is_number("99e2.5".to_string()), false);
        assert_eq!(is_number("--6".to_string()), false);
        assert_eq!(is_number("-+3".to_string()), false);
        assert_eq!(is_number("95a54e53".to_string()), false);
    }
}
