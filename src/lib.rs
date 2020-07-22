use regex::Regex;

pub trait MatcherTrait {
    fn execute(&self, line: &str) -> bool;
}

pub struct ExtendedRegexpMatcher {
    pattern: Regex,
}
impl ExtendedRegexpMatcher {
    pub fn new(pattern: String) -> ExtendedRegexpMatcher {
        ExtendedRegexpMatcher {
            pattern: Regex::new(&pattern).unwrap(),
        }
    }
}

impl MatcherTrait for ExtendedRegexpMatcher {
    fn execute(&self, line: &str) -> bool {
        self.pattern.is_match(line)
    }
}

pub struct FixedStringsMatcher {
    pattern: String,
}

impl FixedStringsMatcher {
    pub fn new(pattern: String) -> FixedStringsMatcher {
        FixedStringsMatcher { pattern: pattern }
    }
}

impl MatcherTrait for FixedStringsMatcher {
    fn execute(&self, line: &str) -> bool {
        line.contains(&self.pattern)
    }
}

pub enum Matcher {
    ExtendedRegexp(ExtendedRegexpMatcher),
    FixedStrings(FixedStringsMatcher),
}
impl Matcher {
    pub fn new(pattern: String, is_fixed_strings_mode: bool) -> Matcher {
        if is_fixed_strings_mode {
            Matcher::FixedStrings(FixedStringsMatcher::new(pattern.to_string()))
        } else {
            Matcher::ExtendedRegexp(ExtendedRegexpMatcher::new(pattern.to_string()))
        }
    }
    pub fn execute(&self, line: &str) -> bool {
        match self {
            Matcher::FixedStrings(m) => m.execute(line),
            Matcher::ExtendedRegexp(m) => m.execute(line),
        }
    }
}

#[cfg(test)]
mod tests {
    /*
        #[test]
        fn it_works() {
            assert_eq!(2 + 2, 4);
        }
    */
    use super::*;

    #[test]
    fn test_extended_regexp_matcher() {
        let matcher = Matcher::new("c".to_string(), false);
        assert_eq!(true, matcher.execute("abcdefg"));
        let matcher = Matcher::new("fg".to_string(), false);
        assert_eq!(true, matcher.execute("abcdefg"));
        let matcher = Matcher::new("Z".to_string(), false);
        assert_eq!(false, matcher.execute("abcdefg"));
        let matcher = Matcher::new("a.c".to_string(), false);
        assert_eq!(true, matcher.execute("abcdefg"));
        let matcher = Matcher::new("a+.b+".to_string(), false);
        assert_eq!(true, matcher.execute("aaa bbb"));
        let matcher = Matcher::new("[aA][bB][cC]".to_string(), false);
        assert_eq!(true, matcher.execute("aBc"));
        assert_eq!(true, matcher.execute("Abc"));
        assert_eq!(false, matcher.execute("efg"));

    }
    #[test]
    fn test_match_fix_string() {
        let matcher = Matcher::new("c".to_string(), true);
        assert_eq!(true, matcher.execute("abcdefg"));
        assert_eq!(true, matcher.execute("cccc"));
        let matcher = Matcher::new("fg".to_string(), true);
        assert_eq!(true, matcher.execute("abcdefg"));
        let matcher = Matcher::new("Z".to_string(), true);
        assert_eq!(false, matcher.execute("abcdefg"));
        let matcher = Matcher::new("a.c".to_string(), true);
        assert_eq!(false, matcher.execute("abcdefg"));
        let matcher = Matcher::new("a+.b+".to_string(), true);
        assert_eq!(false, matcher.execute("aaa bbb"));
        let matcher = Matcher::new("[aA][bB][cC]".to_string(), true);
        assert_eq!(false, matcher.execute("aBc"));
    }
}
