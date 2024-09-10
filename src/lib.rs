mod pattern;

use pattern::{Pattern, PatternErr};
use std::error::Error;
use std::io;

pub struct Config {
    patterns: Vec<Pattern>,
    input: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, Box<dyn Error>> {
        args.next();
        if args.next().unwrap() != "-E" {
            return Err("Expected first argument to be '-E'".into());
        }

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string".into()),
        };

        let mut input_line = String::new();

        io::stdin().read_line(&mut input_line).unwrap();

        let patterns = Self::parse_pattern(query)?;

        Ok(Config {
            patterns,
            input: input_line,
        })
    }

    fn parse_pattern(pattern: String) -> Result<Vec<Pattern>, PatternErr> {
        let mut pattern_chars = pattern.chars().peekable();
        let mut patterns: Vec<Pattern> = Vec::new();

        while let Some(&pat) = pattern_chars.peek() {
            match pat {
                '\\' => {
                    pattern_chars.next(); // Consume the '\'
                    if let Some(next_char) = pattern_chars.next() {
                        match next_char {
                            'd' => patterns.push(Pattern::Digit),
                            'w' => patterns.push(Pattern::AlphaNumeric),
                            _ => {
                                return Err(PatternErr::InvalidPattern(
                                    "Unsupported escape sequence".to_string(),
                                ))
                            }
                        }
                    }
                }
                '^' => {
                    if !patterns.is_empty() {
                        return Err(PatternErr::InvalidPattern(
                            "Start anchor should be the first character".to_string(),
                        ));
                    }
                    pattern_chars.next(); // Consume the '^'
                    patterns.push(Pattern::StartAnchor);
                }
                '$' => {
                    pattern_chars.next();
                    if let Some(_) = pattern_chars.peek() {
                        return Err(PatternErr::InvalidPattern(
                            "End anchor should be the last character".to_string(),
                        ));
                    } else {
                        patterns.push(Pattern::EndAnchor)
                    }
                }
                '[' => {
                    pattern_chars.next();
                    let mut char_group = String::new();
                    let mut is_negative = false;

                    if pattern_chars.peek() == Some(&'^') {
                        pattern_chars.next();
                        is_negative = true
                    }

                    while let Some(&ch) = pattern_chars.peek() {
                        match ch {
                            ']' => {
                                pattern_chars.next();
                                break;
                            }
                            _ => {
                                char_group.push(ch);
                                pattern_chars.next();
                            }
                        }
                    }
                    if is_negative {
                        patterns.push(Pattern::NegativeCharGroup(char_group));
                    } else {
                        patterns.push(Pattern::PositiveCharGroup(char_group));
                    }
                }
                '(' => {
                    pattern_chars.next();
                    let mut char_group1 = String::new();

                    while let Some(&ch) = pattern_chars.peek() {
                        match ch {
                            '|' => {
                                pattern_chars.next();
                                break;
                            }
                            _ => {
                                char_group1.push(ch);
                                pattern_chars.next();
                            }
                        }
                    }
                    let mut char_group2 = String::new();
                    while let Some(&ch) = pattern_chars.peek() {
                        match ch {
                            ')' => {
                                pattern_chars.next();
                                break;
                            }
                            _ => {
                                char_group2.push(ch);
                                pattern_chars.next();
                            }
                        }
                    }

                    patterns.push(Pattern::Alternation(char_group1, char_group2))
                }
                '.' => {
                    pattern_chars.next();
                    patterns.push(Pattern::Wildcard);
                }
                _ => {
                    pattern_chars.next();

                    if pattern_chars.next_if(|&c| c == '+').is_some() {
                        patterns.push(Pattern::OneOrMore(pat));
                    } else if pattern_chars.next_if(|&c| c == '?').is_some() {
                        patterns.push(Pattern::ZeroOrOne(pat));
                    } else {
                        patterns.push(Pattern::Char(pat));
                    }
                }
            }
        }

        Ok(patterns)
    }

    pub fn match_pattern(&self) -> bool {
        if self.patterns[0] == Pattern::StartAnchor {
            return Self::matchhere(
                &mut self.patterns[1..].iter(),
                &mut self.input.chars().peekable(),
            );
        }

        for start_pos in 0..self.input.len() {
            let mut pattern_iter = self.patterns.iter();
            let mut input_iter = self.input[start_pos..].chars().peekable();
            if Self::matchhere(&mut pattern_iter, &mut input_iter) {
                return true;
            }
        }

        false
    }

    fn matchhere(
        patterns: &mut std::slice::Iter<Pattern>,
        input: &mut std::iter::Peekable<std::str::Chars>,
    ) -> bool {
        match patterns.next() {
            Some(Pattern::Char(c)) => {
                input.next_if_eq(c).is_some() && Self::matchhere(patterns, input)
            }
            Some(Pattern::AlphaNumeric) => {
                input.next_if(|&c| c.is_ascii_alphanumeric()).is_some()
                    && Self::matchhere(patterns, input)
            }
            Some(Pattern::Digit) => {
                input.next_if(|&c| c.is_numeric()).is_some() && Self::matchhere(patterns, input)
            }
            Some(Pattern::PositiveCharGroup(group)) => {
                input.next_if(|&c| group.contains(c)).is_some() && Self::matchhere(patterns, input)
            }
            Some(Pattern::NegativeCharGroup(group)) => {
                input.next_if(|&c| !group.contains(c)).is_some() && Self::matchhere(patterns, input)
            }
            Some(Pattern::StartAnchor) => false,
            Some(Pattern::EndAnchor) => input.peek().map_or(true, |&c| c == '\n'),
            Some(Pattern::OneOrMore(repeat_c)) => {
                if input.next_if(|&c| c == *repeat_c).is_some() {
                    while input.next_if(|&c| c == *repeat_c).is_some() {}
                    Self::matchhere(patterns, input)
                } else {
                    false
                }
            }
            Some(Pattern::ZeroOrOne(c)) => {
                if input.peek().unwrap() == c {
                    input.next();
                    Self::matchhere(patterns, input)
                } else {
                    Self::matchhere(patterns, input)
                }
            }
            Some(Pattern::Wildcard) => {
                input.next();
                Self::matchhere(patterns, input)
            }
            Some(Pattern::Alternation(g1, g2)) => {
                let mut temp_input = input.clone();
                temp_input.eq(g1.chars()) || input.eq(g2.chars())
            }
            None => true,
        }
    }
}
