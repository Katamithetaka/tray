use std::{
    fmt::Display,
    iter::Peekable,
    str::{CharIndices, FromStr},
};

#[derive(Debug, Clone)]
pub enum Token {
    Plus,
    Minus,
    Multiply,
    Divide,
    I32(i32),
    I64(i64),
    I128(i128),
    F32(f32),
    F64(f64),
    String(String),
    Char(char),
    LParenthesis,
    RParenthesis,
}

pub type TokenList = Vec<Token>;

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Plus => f.write_str("Plus"),
            Token::Minus => f.write_str("Minus"),
            Token::Multiply => f.write_str("Multiply"),
            Token::Divide => f.write_str("Divide"),
            Token::I32(i) => write!(f, "{i}i32"),
            Token::I64(i) => write!(f, "{i}i64"),
            Token::I128(i) => write!(f, "{i}i128"),
            Token::F32(float) => write!(f, "{float}f32"),
            Token::F64(float) => write!(f, "{float}f64"),
            Token::String(string) => write!(f, "\"{string}\""),
            Token::Char(char) => write!(f, "\'{char}\'"),
            Token::LParenthesis => write!(f, "Left parenthesis"),
            Token::RParenthesis => write!(f, "Right parenthesis"),
        }
    }
}

#[derive(Debug)]
pub enum LexerError {
    IllegalCharacter {
        position: usize,
        message: String,
    },
    ParsingError {
        start_position: usize,
        end_position: usize,
        message: String,
    },
}

impl LexerError {
    pub fn arrow_error(&self, line: String) {
        let (begin_pos, end_pos, message) = match self {
            LexerError::IllegalCharacter { position, message } => {
                (*position, *position + 1, message)
            }
            LexerError::ParsingError {
                start_position,
                end_position,
                message,
            } => (*start_position, *end_position, message),
        };
        eprintln!(
            "{message}\n{line}\n{}",
            " ".repeat(begin_pos) + &"^".repeat(end_pos - begin_pos)
        );
    }

    // pub fn offset_by(&self, size: usize) -> Self {
    //     match self {
    //         LexerError::IllegalCharacter { position } => LexerError::IllegalCharacter {
    //             position: position + size,
    //         },
    //         LexerError::ParsingError {
    //             start_position,
    //             end_position,
    //         } => LexerError::ParsingError {
    //             start_position: start_position + size,
    //             end_position: end_position + size,
    //         },
    //     }
    // }
}

pub fn parse_number(iterator: &mut Peekable<CharIndices>) -> Result<Token, LexerError> {
    let mut string = String::new();
    let mut has_dot = false;
    let begin_index = iterator
        .peek()
        .expect("Expected iterator to still be valid.")
        .0;
    let mut end_index = begin_index.clone();
    loop {
        if let Some((index, char)) = iterator.peek() {
            match *char {
                c if c == '_' && string.ends_with('.') => {
                    return Err(LexerError::ParsingError {
						start_position: begin_index,
                        end_position: index + 1,
						message: String::from("Syntax Error: Cannot add a `_` in a number right after a floating point `.`"),
                    })
                }
                c if c == '.' && has_dot => {
                    return Err(LexerError::ParsingError {
                        start_position: begin_index,
                        end_position: index + 1,
                        message: String::from("Syntax Error: A floating point number cannot have multiple `.`"),
                    })
                }
                '.' => {
                    has_dot = true;
                    string.push('.');
                }
                '_' => {}
                c @ '0'..='9' => string.push(c),
                _ if char.is_whitespace() => {
                    break;
                }
                _ => break,
            }
            end_index = *index;
            iterator.next();
        } else {
            break;
        }
    }

    match string.len() {
        0 => Err(LexerError::ParsingError {
            start_position: begin_index,
            end_position: begin_index + 1,
            message: String::from("Unreachable."),
        }),
        1..=9 => {
            if has_dot {
                return Ok(Token::F32(f32::from_str(&string).map_err(|_| {
                    LexerError::ParsingError {
                        start_position: begin_index,
                        end_position: end_index + 1,
                        message: String::from("Parsing Error: Couldn't parse number to a Float32"),
                    }
                })?));
            } else {
                Ok(Token::I32(i32::from_str(&string).map_err(|_| {
                    LexerError::ParsingError {
                        start_position: begin_index,
                        end_position: end_index + 1,
                        message: String::from("Parsing Error: Couldn't parse number to a Int32"),
                    }
                })?))
            }
        }
        10..=18 => {
            if has_dot {
                return Ok(Token::F64(f64::from_str(&string).map_err(|_| {
                    LexerError::ParsingError {
                        start_position: begin_index,
                        end_position: end_index + 1,
                        message: String::from("Parsing Error: Couldn't parse number to a Float64"),
                    }
                })?));
            } else {
                Ok(Token::I64(i64::from_str(&string).map_err(|_| {
                    LexerError::ParsingError {
                        start_position: begin_index,
                        end_position: end_index + 1,
                        message: String::from("Parsing Error: Couldn't parse number to a Int64"),
                    }
                })?))
            }
        }
        _ => {
            if has_dot {
                return Ok(Token::F64(f64::from_str(&string).map_err(|_| {
                    LexerError::ParsingError {
                        start_position: begin_index,
                        end_position: end_index + 1,
                        message: String::from("Parsing Error: Couldn't parse number to a Float64"),
                    }
                })?));
            } else {
                Ok(Token::I128(i128::from_str(&string).map_err(|_| {
                    LexerError::ParsingError {
                        start_position: begin_index,
                        end_position: end_index + 1,
                        message: String::from("Parsing Error: Couldn't parse number to a Int128"),
                    }
                })?))
            }
        }
    }
}

fn parse_escape(iterator: &mut Peekable<CharIndices>) -> Result<char, LexerError> {
    let begin_index = iterator.next().expect("Expected string to stay valid").0;

    if let Some((index, char)) = iterator.peek() {
        match *char {
            'x' => {
                let _ = iterator.next().expect("Expected iterator to stay valid");
                let (code_1, code_2) = (iterator.next(), iterator.next());
                if let (Some(code_1), Some(code_2)) = (code_1, code_2) {
                    let code = u8::from_str_radix(&format!("{}{}", code_1.1, code_2.1), 16);
                    if let Ok(code) = code {
                        if code > 0x7F {
                            return Err(LexerError::ParsingError {
                                start_position: begin_index,
                                end_position: code_2.0 + 1,
                                message: String::from("Syntax Error: Expected two digits hexadecimal number lower between 0x00 and 0x7F."),
                            });
                        } else {
                            return Ok(code as char);
                        }
                    } else {
                        return Err(LexerError::ParsingError {
                            start_position: begin_index,
                            end_position: code_2.0 + 1,
							message: String::from("Syntax Error: Couldn't parse escaped hexadecimal number. Make sure the digits are valid hexadecimal characters (0-9, A-F).")
                        });
                    }
                } else if let Some(code_1) = code_1 {
                    return Err(LexerError::ParsingError {
                        start_position: begin_index,
                        end_position: code_1.0 + 1,
                        message: String::from("Syntax Error: Expected two hexadecimal digits to be escaped, could only find one. Make sure to use two hexadecimal digits like `\\x7F`"),
                    });
                } else {
                    return Err(LexerError::ParsingError {
                        start_position: begin_index,
                        end_position: begin_index + 1,
                        message: String::from("Syntax Error: Tried to escape an hexadecimal number 7bit number with \\x but couldn't find the two required hexadecimal digits."),
                    });
                }
            }
            'u' => {
                let _ = iterator.next().expect("Iterator should still be valid");
                let lcurly = iterator.next();

                if let Some(lcurly) = lcurly {
                    let mut count = 0;
                    let str = iterator
                        .clone()
                        .take_while(|(_, value)| match *value {
                            'a'..='z' | 'A'..='Z' | '0'..='9' => {
                                count += 1;
                                count < 7
                            }
                            '}' => false,
                            _ => false,
                        })
                        .map(|(_, v)| v)
                        .collect::<String>();
                    for _ in 0..count {
                        iterator.next().expect("Cannot fail");
                    }

                    if count == 0 {
                        return Err(LexerError::ParsingError {
                            start_position: begin_index,
                            end_position: lcurly.0 + 2,
                            message: String::from("Syntax Error: Tried to escape a 24bit unicode character but no hexadecimal digits were found. Make sure to specify a hexadecimal number in range [0, 10FFFF]"),
                        });
                    } else if count == 7 {
                        return Err(LexerError::ParsingError {
                            start_position: begin_index,
                            end_position: lcurly.0 + 7,
							message: String::from("Syntax Error: Tried to escape a 24bit unicode character but found 7 or more characters. A 24bit unicode character can at most have 6 hexadecimal digits and has to be in range [0, 10FFFF].")
                        });
                    }

                    let rcurly = iterator.next();
                    if let Some((_, '}')) = rcurly {
                        if let Ok(number) = u32::from_str_radix(&str, 16) {
                            if let Some(char) = char::from_u32(number) {
                                return Ok(char);
                            } else {
                                return Err(LexerError::ParsingError {
                                    start_position: begin_index,
                                    end_position: begin_index + count + 4,
                                    message: String::from("Parsing error: Couldn't convert escaped unicode character back into a single character, make sure the number represented is between in range [0,10FFFF]"),
                                });
                            }
                        } else {
                            return Err(LexerError::ParsingError {
                                start_position: begin_index,
                                end_position: begin_index + count + 4,
                                message: String::from("Parsing error: Couldn't convert escaped 24bit unicode character into a number. Make sure to use valid hexadecimal digits from 0-9 and A-F"),
                            });
                        }
                    } else {
                        return Err(LexerError::ParsingError {
                            start_position: begin_index,
                            end_position: begin_index + count + 4,
                            message: String::from("Syntax error: Escaped unicode characters must follow the format: `\\u{07FFFF}` with from 1 to 6 digits. The range of numbers is [0, 10FFFF].")
							
                        });
                    }
                } else {
                    return Err(LexerError::ParsingError {
                        start_position: begin_index,
                        end_position: begin_index + 2,
						message: String::from("Syntax error: Escaped unicode characters must follow the format: `\\u{07FFFF}` with from 1 to 6 digits. The range of numbers is [0, 10FFFF].")
                    });
                }
            }

            'n' => {
                iterator.next();
                return Ok('\n');
            }
            'r' => {
                iterator.next();
                return Ok('\r');
            }
            't' => {
                iterator.next();
                return Ok('\t');
            }
            '\\' => {
                iterator.next();
                return Ok('\\');
            }
            '0' => {
                iterator.next();
                return Ok('\0');
            }
            '\'' => {
                iterator.next();
                return Ok('\'');
            }
            '"' => {
                iterator.next();
                return Ok('"');
            }
            c => {
                return Err(LexerError::ParsingError {
                    start_position: begin_index,
                    end_position: *index + 1,
                    message: format!("Syntax error: Character {c} preceeded by a `\\` cannot be escaped, make sure to escape the backslash like `\\\\{c}` if you meant to add a backslash to the string."),
                });
            }
        }
    } else {
        return Err(LexerError::ParsingError {
            start_position: begin_index,
            end_position: begin_index + 1,
            message: String::from("Syntax error: "),
        });
    }
}

pub fn parse_char(iterator: &mut Peekable<CharIndices>) -> Result<Token, LexerError> {
    let content: char;

    let begin_index = iterator.next().expect("Expected string to stay valid").0;
    if let Some((index, char)) = iterator.peek() {
        match *char {
            '\'' => {
                return Err(LexerError::ParsingError {
                    start_position: begin_index,
                    end_position: index + 1,
                    message: String::from("Character literal cannot be empty."),
                })
            }
            '\\' => {
                content = parse_escape(iterator).map_err(|err| err)?;
            }
            c => {
                content = c;
                iterator.next().expect("Expected iterator to stay valid");
            }
        };

        if let Some((_, '\'')) = iterator.next() {
            return Ok(Token::Char(content));
        } else {
            return Err(LexerError::ParsingError {
                start_position: begin_index,
                end_position: begin_index + 2,
                message: String::from("Syntax Error: Expected `'` to end the character literal."),
            });
        }
    } else {
        return Err(LexerError::ParsingError {
            start_position: begin_index,
            end_position: begin_index + 1,
			message: String::from("Syntax Error: Found end of file while trying to parse a character literal. Try removing the trailing `'`.")
        });
    }
}

pub fn parse_string(iterator: &mut Peekable<CharIndices>) -> Result<Token, LexerError> {
    let mut content = String::new();

    let begin_index = iterator.next().expect("Expected string to stay valid").0;
    let mut end_index = begin_index;
    loop {
        if let Some((index, char)) = iterator.peek() {
            match *char {
                '"' => {
                    iterator.next();
                    return Ok(Token::String(content));
                }
                '\\' => {
                    content.push(parse_escape(iterator).map_err(|err| err)?);
                    continue;
                }
                c => {
                    content.push(c);
                    end_index = *index;
                }
            }
            iterator.next();
        } else {
            return Err(LexerError::ParsingError {
                start_position: begin_index,
                end_position: end_index + 1,
                message: String::from("Syntax Error: Found end of file while trying to parse string literal. Make sure to close the quotes or remove the trailing `\"`"),
            });
        }
    }
}

pub fn parse_tokens(content: String) -> Result<TokenList, LexerError> {
    let mut return_val = vec![];
    let mut iterator = content.char_indices().peekable();
    loop {
        if let Some((index, char)) = iterator.peek() {
            match char {
                '+' => return_val.push(Token::Plus),
                '-' => return_val.push(Token::Minus),
                '*' => return_val.push(Token::Multiply),
                '/' => return_val.push(Token::Divide),
                '(' => return_val.push(Token::LParenthesis),
                ')' => return_val.push(Token::RParenthesis),
                '"' => {
                    return_val.push(parse_string(&mut iterator)?);
                    continue;
                }
                '\'' => {
                    return_val.push(parse_char(&mut iterator)?);
                }
                '0'..='9' => {
                    return_val.push(parse_number(&mut iterator)?);
                    continue;
                }
                _ if char.is_whitespace() => {}
                c => return Err(LexerError::IllegalCharacter { position: *index, message: format!("Unrecognized character {c}") }),
            }
            iterator.next();
        } else {
            break;
        }
    }

    Ok(return_val)
}

pub fn parse_one(string: String) -> Result<Vec<Token>, LexerError> {
    parse_tokens(string)
}
