use crate::compiler::CompileException;

enum Token {
    Plus, Minus, Multiply, Divide, Mod, And, Or, Not, Xor, StrictAnd, StrictOr,
    Assign, PlusAssign, MinusAssign, MultiplyAssign, DivideAssign, ModAssign, Increment, Decrement,
    Equals, NotEqual, Greater, Less, GreaterEqual, LessEqual, 
    Colon, Comma, OpenParen, CloseParen, OpenBrace, CloseBrace, OpenBracket, CloseBracket, Dot, Accesser,
    String(String), Annotation(String), Identifier(String), Type(String), Int(i64), Num(f32), True, False,
    FuncDef, ProcDef, StructDef, EnumDef, TraitDef, Implementation, GroupDef, Private, Const, Let,
    Select, Raise, Import,
    SelfIdent, SelfType,
    For, While, If, In, Else
}

// + - * / % & | ! ^ && ||
// = += -= *= /= %=
// == > < >= <=
// : , ( ) { } [ ] . ::
// "str" 'str' @Annotation identifier Type 23 23.4 true false 
// Self self
// fn pc struct enum trait impl group private const let
// select raise import
// for in while if else

// 23 23.4

fn tokenize(input: &str) -> Result<Vec<Token>, CompileException> {
    let mut tokens = vec![];
    let mut iter = input.chars().peekable();
    let mut line_number = 1usize;
    let mut line_index = 0usize;
    while let Some(next) = iter.next() {
        line_index+=1;
        match next {
            '+' => {
                match iter.peek() {
                    Some('=') => {
                        iter.next();
                        tokens.push(Token::PlusAssign)
                    },
                    Some('+') => {
                        iter.next();
                        tokens.push(Token::Increment)
                    },
                    _ => tokens.push(Token::Plus)
                }
            }
            '-' => {
                match iter.peek() {
                    Some('=') => {
                        iter.next();
                        tokens.push(Token::MinusAssign)
                    },
                    Some('-') => {
                        iter.next();
                        tokens.push(Token::Decrement)
                    },
                    _ => tokens.push(Token::Minus)
                }
            }
            '*' => {
                match iter.peek() {
                    Some('=') => {
                        iter.next();
                        tokens.push(Token::MultiplyAssign)
                    },
                    _ => tokens.push(Token::Multiply)
                }
            }
            '/' => {
                match iter.peek() {
                    Some('=') => {
                        iter.next();
                        tokens.push(Token::DivideAssign)
                    },
                    Some('/') => {
                        loop {
                            if let Some('\n') = iter.next() {
                                break;
                            }
                        }
                    },
                    Some('*') => {
                        loop {
                            if let Some('*') = iter.next() {
                                if let Some('/') = iter.next() {
                                    break;
                                }
                            }
                        }
                    },
                    _ => tokens.push(Token::Divide)
                }
            }
            '%' => {
                match iter.peek() {
                    Some('=') => {
                        iter.next();
                        tokens.push(Token::ModAssign)
                    },
                    _ => tokens.push(Token::Mod)
                }
            }
            '&' => {
                match iter.peek() {
                    Some('&') => {
                        iter.next();
                        tokens.push(Token::StrictAnd)
                    },
                    _ => tokens.push(Token::And)
                }
            }
            '|' => {
                match iter.peek() {
                    Some('|') => {
                        iter.next();
                        tokens.push(Token::StrictOr)
                    },
                    _ => tokens.push(Token::Or)
                }
            }
            '=' => {
                match iter.peek() {
                    Some('=') => {
                        iter.next();
                        tokens.push(Token::Equals)
                    },
                    Some('>') => {
                        iter.next();
                        tokens.push(Token::FatArrow)
                    },
                    _ => tokens.push(Token::Assign)
                }
            }
            '>' => {
                match iter.peek() {
                    Some('=') => {
                        iter.next();
                        tokens.push(Token::GreaterEqual)
                    },
                    _ => tokens.push(Token::Greater)
                }
            }
            '<' => {
                match iter.peek() {
                    Some('=') => {
                        iter.next();
                        tokens.push(Token::LessEqual)
                    },
                    _ => tokens.push(Token::Less)
                }
                
            }
            ':' => {
                match iter.peek() {
                    Some(':') => {
                        iter.next();
                        tokens.push(Token::Accesser)
                    },
                    _ => tokens.push(Token::Colon)
                }
            }
            '!' => {
                match iter.peek() {
                    Some('=') => {
                        iter.next();
                        tokens.push(Token::NotEqual)
                    },
                    _ => tokens.push(Token::Not)
                }
                
            }
            '^' => tokens.push(Token::Xor),
            ',' => tokens.push(Token::Comma),
            '.' => tokens.push(Token::Dot),
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            '[' => tokens.push(Token::OpenBracket),
            ']' => tokens.push(Token::CloseBracket),
            '{' => tokens.push(Token::OpenBrace),
            '}' => tokens.push(Token::CloseBrace),
            '\n' => {
                line_index = 0;
                line_number+=1;
            }
            c @ 'a'..='z' => {
                let mut ident = String::with_capacity(32);
                ident.push(c);
                while let Some(m @ 'a'..='z' | m @ 'A'..='Z' | m @ '0'..='9' | m @ '_') = iter.peek() {
                    iter.next();
                    ident.push(*m);
                };
                tokens.push(match ident.as_str() {
                    "fn" => Token::FuncDef,
                    "pc" => Token::ProcDef,
                    "struct" => Token::StructDef,
                    "trait" => Token::TraitDef,
                    "enum" => Token::EnumDef,
                    "group" => Token::GroupDef,
                    "private" => Token::Private,
                    "let" => Token::Let,
                    "const" => Token::Const,
                    "self" => Token::SelfIdent,
                    "import" => Token::Import,
                    "impl" => Token::Implementation,
                    "in" => Token::In,
                    _ => Token::Identifier(ident)

                });
            }
            c @ 'A'..='Z' => {
                let mut ident = String::with_capacity(32);
                ident.push(c);
                while let Some(m @ 'a'..='z' | m @ 'A'..='Z' | m @ '0'..='9' | m @ '_') = iter.peek() {
                    iter.next();
                    ident.push(*m);
                };
                tokens.push(match ident.as_str() {
                    "Self" => Token::SelfType,
                    _ => Token::Type(ident)
                });
            }
            '@' => {
                let mut ident = String::with_capacity(32);
                while let Some(n) = iter.peek() {
                    match n {
                        m @ 'a'..='z' | m @ 'A'..='Z' | m @ '0'..='9' | m @ '_' => {
                            iter.next();
                            ident.push(*m);
                        }
                        _ => break
                    };
                };
                tokens.push(Token::Annotation(ident));
            }
            c @ '"' | c @ '\'' => {
                let mut ident = String::with_capacity(128);
                while let un = iter.peek() {
                    if let Some(n) = un {
                        match n {
                            c => break,
                            '\\' => {
                                match iter.next() {
                                    Some('n') => ident.push('\n'),
                                    Some('t') => ident.push('\t'),
                                    Some('\\') => ident.push('\\'),
                                    Some(c) => ident.push(c),
                                    _ => return Err(CompileException::UnrecognizedEscape(line_number, line_index))
                                }
                            }
                            _ => ident.push(*n)
                        };
                    } else {
                        return Err(CompileException::UnfinishedString(line_number, line_index))
                    }
                };
                tokens.push(Token::String(ident));
            }
            c @ '1'..='9' => {
                let digits = Vec::with_capacity(16);
                while let Some(d @ '0'..='9') = iter.peek() {   // Get all main digits
                    digits.push(iter.next().unwrap())
                }
                let mut num = 0i64;
                let mut position = 1i64;
                for d in digits.into_iter().rev() { // Look through them backwards and add up as necessary
                    num += position * d.to_digit(10).unwrap() as i64;
                    position *= 10;
                }
                if let Some('.') = iter.peek() {    // If a . follows the number
                    iter.next();
                    let mut num = num as f32;
                    let mut position = 0.1f32;
                    while let Some(d @ '0'..='9') = iter.peek() {   // Loop over decimal digits (no reverse necessary)
                        num += position * iter.next().unwrap().to_digit(10).unwrap() as f32;
                        position /= 10f32;
                    }
                    tokens.push(Token::Num(num));   // Add num token
                } else {
                    if let Some('f') = iter.peek() {    // If int ends in f
                        iter.next();
                        tokens.push(Token::Num(num as f32));    // Add num token
                    } else {
                        tokens.push(Token::Int(num));   // Add int token
                    }
                }
            }
            '0' => {
                match iter.peek() {
                    Some('x') => {
                        iter.next();
                        let digits = Vec::with_capacity(16);
                        while let Some('0'..='9') | Some('a'..='f') | Some('A'..='F') = iter.peek() {
                            digits.push(iter.next().unwrap().to_ascii_lowercase())
                        }
                        let mut num = 0i64;
                        let mut position = 1i64;
                        for d in digits.into_iter().rev() {
                            num += position * d.to_digit(16).unwrap() as i64;
                            position *= 16;
                        }
                        tokens.push(Token::Int(num));
                    }
                    Some('b') => {
                        iter.next();
                        let digits = Vec::with_capacity(16);
                        while let Some(d @ '0'..='1') = iter.peek() {
                            digits.push(iter.next().unwrap())
                        }
                        let mut num = 0i64;
                        let mut position = 1i64;
                        for d in digits.into_iter().rev() {
                            num += position * d.to_digit(2).unwrap() as i64;
                            position *= 2;
                        }
                        tokens.push(Token::Int(num));
                    }
                    Some('.') => {
                        iter.next();
                        let mut num = 0f32;
                        let mut position = 0.1f32;
                        while let Some(d @ '0'..='9') = iter.peek() {
                            num += position * iter.next().unwrap().to_digit(10).unwrap() as f32;
                            position /= 10f32;
                        }
                        tokens.push(Token::Num(num));
                    }
                    Some('f') => {
                        iter.next();
                        tokens.push(Token::Num(0f32));
                    }
                    Some('0'..='9') => {
                        iter.next(); // Ignore something like 05325, this will just turn into 5325
                    }
                    _ => {
                        iter.next();
                        tokens.push(Token::Int(0));
                    }
                }
            }
            ' ' | '\n' => {
                iter.next();
            }
            c => return Err(CompileException::UnrecognizedToken(c.to_string(), line_number, line_index))
        };
    };
    Ok(tokens)
}