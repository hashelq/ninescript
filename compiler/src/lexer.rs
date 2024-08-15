use regex::Regex;

use crate::{error::{LexicalError, LexicalErrorType}, location::Location, token::Tok, types::RGBA};

pub type Spanned = (Location, Tok, Location);
pub type LexResult = Result<Spanned, LexicalError>;

pub struct Lexer {
    chars: Vec<char>,
    position: usize,
    location: Location,
    indention_level: usize,
    indention_now: usize,
    dedent_required: isize,
    prev_new_line: bool,
    new_line: bool
}

impl Lexer {
    pub fn new(src: &str, indention_level: usize) -> Self {
        let src = Regex::new(r"^([_a-zA-Z]+[_a-zA-Z0-9]*)(<.*>)?\(.*\) =>").unwrap().replace_all(src, "\0function $0");
        let src = Regex::new(r"\[([_a-zA-Z]+[_a-zA-Z0-9]*)(,\s*[_a-zA-Z]+[_a-zA-Z0-9]*)*\]\s*=").unwrap().replace_all(&src, "\0unpack_tuple $0");
        let src = src.replace("\t", &String::from(" ").repeat(indention_level));
        let chars = src.chars().collect();
        Self { chars, position: 0, location: Location::new(1, 1), indention_level, new_line: true, indention_now: 0, dedent_required: 0, prev_new_line: true }
    }

    fn has_more_tokens(&self) -> bool {
        return self.position < self.chars.len();
    }

    fn set_next_char(&mut self) -> Option<char> {
        let char = self.chars.get(self.position).map(Clone::clone); 
        self.go_right();
        return char;
    }

    fn go_right(&mut self) {
        self.position += 1;
        self.location.go_right();
    }

    fn go_left(&mut self) {
        self.position -= 1;
        self.location.go_left();
    }

    fn get_marker(&self, id: &str) -> Option<Tok> {
        Some(match id {
            "function" => Tok::FunctionMarker,
            "unpack_tuple" => Tok::UnpackTupleMarker,
            _ => return None
        })
    }

    fn get_keyword(&self, id: &str) -> Option<Tok> {
        Some(match id {
            "if" => Tok::If,
            "else" => Tok::Else,
            "series" => Tok::Series,
            "const" => Tok::Const,
            "type" => Tok::Type,
            "enum" => Tok::Enum,
            "var" => Tok::Var,
            "for" => Tok::For,
            "to" => Tok::To,
            "in" => Tok::In,
            "by" => Tok::By,
            "while" => Tok::While,
            "switch" => Tok::Switch,
            "import" => Tok::Import,
            "not" => Tok::Not,
            "and" => Tok::And,
            "or" => Tok::Or,
            "true" => Tok::True,
            "false" => Tok::False,
            _ => return None
        })
    }

    fn make_eof(&mut self) -> Spanned { 
        let t = if self.indention_now > 0 {
            self.indention_now -= 1;
            Tok::Dedent
        } else {
            Tok::EndOfFile
        }; 

        (self.location, t, self.location)
    }

    fn inner_next(&mut self) -> Result<Spanned, LexicalError> {
        if !self.has_more_tokens() {
            return Ok(self.make_eof());
        };

        let loc_left = self.location.clone();
        let mut char = self.set_next_char().unwrap();

        // comment
        if char == '/' {
            if let Some('/') = self.set_next_char() {
                loop {
                    char = match self.set_next_char() {
                        Some(v) => v,
                        None => return Ok(self.make_eof())
                    };

                    if char == '\n' {
                        self.go_left();
                        break;
                    }
                };
            } else {
                self.go_left();
            }
        }
        
        let count_for_indent = (self.indention_now + 1) * self.indention_level;
        let count_for_indent2 = (self.indention_now + 2) * self.indention_level;

        // new line:
        if char == '\n' {
            /* It is not a new line if the next line indention is higher than previous, but not
             * exact as it must be */
            self.location.newline();

            let count = self.chars[self.position .. self.chars.len()].into_iter().position(|x| *x != ' ').unwrap_or(self.chars.len());
            if count > count_for_indent && count < count_for_indent2 {
                return self.inner_next();
            }

            self.new_line = true;
            if self.prev_new_line {
                return self.inner_next();
            }

            self.prev_new_line = true;
            return Ok((self.location, Tok::NewLine, self.location));
        }
        self.prev_new_line = false;

        // indention:
        if self.dedent_required > 0 {
            self.dedent_required -= 1;
            self.indention_now -= 1;
            self.go_left();
            return Ok((loc_left, Tok::Dedent, self.location));
        }
        if self.new_line { 
            let count = self.chars[self.position - 1 .. self.chars.len()].into_iter().position(|x| *x != ' ').unwrap_or(self.chars.len());
            if count > 0 {
                self.position += count - 1;
            }

            if count_for_indent == count {
                self.indention_now += 1;
                self.new_line = false;
                return Ok((loc_left, Tok::Indent, self.location));
            }

            let indention_level = if count == 0 { 0 } else { count / self.indention_level };
            let diff = (indention_level as isize) - (self.indention_now as isize);

            if diff < 0 {
                self.dedent_required = (diff * -1) - 1;
                self.new_line = false;
                self.go_left();
                self.indention_now -= 1;

                return Ok((loc_left, Tok::Dedent, self.location));
            }
            self.new_line = false;
        }

        // ints and floats:
        if char.is_numeric() || char == '.' {
            let mut is_float = false;
            let mut stack = String::new();
            if char == '.' {
                is_float = true;
            }
            stack.push(char);
            
            loop {
                char = match self.set_next_char() {
                    Some(v) => v,
                    None => break
                };

                if char == '.' {
                    if is_float {
                        return Err(LexicalError { error: LexicalErrorType::NumberError, location: loc_left })
                    }

                    is_float = true;
                } else if !char.is_numeric() {
                    self.go_left();
                    break;
                }

                stack.push(char);
            };

            // FIXME: bounds
            let t = if is_float {
                Tok::Float { value: stack.parse::<f64>().unwrap() }
            } else {
                Tok::Int { value: stack.parse::<i64>().unwrap() }
            };
            return Ok((loc_left, t, self.location));
        }

        // strings ("):
        if char == '"' {
            let mut stack = String::new();
            let mut prev_was_backslash = false;
            
            loop {
                char = match self.set_next_char() {
                    Some(v) => v,
                    None => break
                };

                if prev_was_backslash {
                    prev_was_backslash = false;
                } else if char == '\\' {
                    prev_was_backslash = true;
                    continue;
                } else if char == '"' {
                    break;
                }

                stack.push(char);
            };

            // FIXME: bounds
            return Ok((loc_left, Tok::String { value: stack }, self.location));
        }

        // strings ('):
        if char == '\'' {
            let mut stack = String::new();
            let mut prev_was_backslash = false;
            
            loop {
                char = match self.set_next_char() {
                    Some(v) => v,
                    None => break
                };

                if prev_was_backslash {
                    prev_was_backslash = false;
                } else if char == '\\' {
                    prev_was_backslash = true;
                    continue;
                } else if char == '\'' {
                    break;
                }

                stack.push(char);
            };

            // FIXME: bounds
            return Ok((loc_left, Tok::String { value: stack }, self.location));
        }

        // hash colors:
        if char == '#' {
            let mut stack = String::new();
            
            loop {
                char = match self.set_next_char() {
                    Some(v) => v,
                    None => break
                };

                let range_af = char >= 'a' && char <= 'f';
                let range_af_up = char >= 'A' && char <= 'F';

                if !char.is_numeric() && !range_af && !range_af_up {
                    if char.is_alphabetic() {
                        return Err(LexicalError { location: self.location, error: LexicalErrorType::HashColorError });
                    }
                    self.go_left();
                    break;
                }

                stack.push(char);
            };

            let l = stack.len();
            if l != 6 && l != 8 {
                return Err(LexicalError { location: self.location, error: LexicalErrorType::HashColorError });
            }

            let red = u8::from_str_radix(&stack[0..2], 16).unwrap();
            let green = u8::from_str_radix(&stack[2..4], 16).unwrap();
            let blue = u8::from_str_radix(&stack[4..6], 16).unwrap();
            let alpha = if l == 8 { u8::from_str_radix(&stack[6..8], 16).unwrap() } else { 255 };

            return Ok((loc_left, Tok::HashColor(RGBA(red, green, blue, alpha)), self.location));
        }

        // idents:
        if char.is_alphabetic() || char == '_' || char == '\0' {
            let s = char;
            let mut stack = String::new();
            stack.push(char);

            loop {
                char = match self.set_next_char() {
                    Some(v) => v,
                    None => break
                };

                if !char.is_alphanumeric() && char != '_' {
                    self.go_left();
                    break;
                }

                stack.push(char);
            }

            if s == '\0' {
                stack.remove(0);
                let marker = self.get_marker(&stack);
                if let Some(v) = marker {
                    return Ok((loc_left, v, self.location));
                }
            }

            if let Some(v) = self.get_keyword(&stack) {
                return Ok((loc_left, v, self.location))
            }

            return Ok((loc_left, Tok::Identifier { name: stack }, self.location));
        }

        // symbols
        let symbol = match char {
            '+' => {
                let mut t = Tok::Plus;
                    if let Some('=') = self.set_next_char() {
                        t = Tok::EqualAdd;
                    } else {
                        self.go_left();
                    }
                t
            },
            '-' => {
                let mut t = Tok::Minus;
                    if let Some('=') = self.set_next_char() {
                        t = Tok::EqualSub;
                    } else {
                        self.go_left();
                    }
                t
            },
            '/' => {
                let mut t = Tok::Slash;
                    if let Some('=') = self.set_next_char() {
                        t = Tok::EqualDiv;
                    } else {
                        self.go_left();
                    }
                t
            },
            '*' => {
                let mut t = Tok::Asterisk;
                    if let Some('=') = self.set_next_char() {
                        t = Tok::EqualMul;
                    } else {
                        self.go_left();
                    }
                t
            },
            '\\' => Tok::Backslash,
            '?' => Tok::QuestionMark,
            '!' => {
                let c = self.set_next_char();
                if let Some('=') = c {
                    Tok::NotEqual
                } else {
                    self.go_left();
                    Tok::ExclamationMark
                }
            },
            '=' => {
                let mut t = Tok::Equal;
                let c = self.set_next_char();
                if let Some('=') = c {
                    t = Tok::DoubleEqual;
                } else if let Some('>') = c {
                    t = Tok::Follow;
                } else {
                    self.go_left();
                };
                t
            },
            '<' => {
                let c = self.set_next_char();
                if let Some('=') = c {
                    Tok::Lte
                } else {
                    self.go_left();
                    Tok::Less
                }
            },
            '>' => {
                let c = self.set_next_char();
                if let Some('=') = c {
                    Tok::Gte
                } else {
                    self.go_left();
                    Tok::Greater
                }
            },
            '(' => Tok::OpenParenthesis,
            ')' => Tok::CloseParenthesis,
            '[' => Tok::OpenBrackets,
            ']' => Tok::CloseBrackets,
            '@' => Tok::At,
            '#' => Tok::Hash,
            '.' => Tok::Dot,
            ',' => Tok::Comma,
            '%' => Tok::Mod,
            ':' => {
                let mut t = Tok::Colon;
                    if let Some('=') = self.set_next_char() {
                        t = Tok::Set;
                    } else {
                        self.go_left();
                    }
                t
            },
            ';' => Tok::Semicolon,
            _ => return self.inner_next()
        };

        return Ok((loc_left, symbol, self.location));
    }
}

impl Iterator for Lexer {
    type Item = LexResult;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.inner_next();

        match token {
            Ok((_, Tok::EndOfFile, _)) => None,
            r => Some(r)
        }
    }
}
