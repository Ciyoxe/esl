use token::{Token, TokenKind};

pub mod token;

/*************************************************
 *                    STRUCTS                    *
 *************************************************/
pub struct Tokenizer<'a> {
    pub pos: usize,
    pub src: &'a [u8],
}
pub struct TokensIterator<'a> {
    tokenizer: Tokenizer<'a>,
}

/*************************************************
 *               PUBLIC INTERFACE                *
 *************************************************/
impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a [u8]) -> Self {
        Tokenizer { pos: 0, src }
    }
    pub fn print_debug(self) {
        let src = self.src;
        self.into_iter().for_each(|token| {
            let content = &src[token.range.clone()];
            let con_str = String::from_utf8_lossy(content);
            println!("{:#?} -> {}", token.kind, con_str);
        });
    }
}
impl<'a> IntoIterator for Tokenizer<'a> {
    type Item = Token;
    type IntoIter = TokensIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TokensIterator { tokenizer: self }
    }
}
impl Iterator for TokensIterator<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokenizer.skip_ignored();
        self.tokenizer.next_token()
    }
}

/*************************************************
 *                    HELPERS                    *
 *************************************************/
impl<'a> Tokenizer<'a> {
    fn mov(&mut self) {
        self.pos += 1;
    }
    fn skip(&mut self, f: impl Fn(u8) -> bool) {
        while self.src.get(self.pos).copied().is_some_and(&f) {
            self.mov();
        }
    }
    fn skip_and_get(&mut self, f: impl Fn(u8) -> bool) -> &[u8] {
        let start = self.pos;
        self.skip(f);
        &self.src[start..self.pos]
    }

    fn next(&self) -> Option<u8> {
        self.src.get(self.pos).copied()
    }
    fn next_at(&self, pos: usize) -> Option<u8> {
        self.src.get(self.pos + pos).copied()
    }
    fn next_unwrap(&self) -> u8 {
        self.src[self.pos]
    }

    fn make_token(&mut self, f: impl FnOnce(&mut Self) -> Option<TokenKind>) -> Option<Token> {
        let pos = self.pos;
        let tok = f(self);

        if let Some(tok) = tok {
            return Some(Token {
                kind: tok,
                range: pos..self.pos,
            });
        }
        None
    }
}

/*************************************************
 *                TOKEN MATCHERS                 *
 *************************************************/
impl<'a> Tokenizer<'a> {
    fn t_number(&mut self) -> Option<Token> {
        if !self.next_unwrap().is_ascii_digit() {
            return None;
        }

        self.make_token(|this| {
            if this.next_unwrap() == b'0' && this.next_at(1).is_some_and(|b| b == b'b') {
                this.mov();
                this.mov();
                this.skip(|b| b == b'0' || b == b'1');

                Some(TokenKind::NumBinInt)
            } else if this.next_unwrap() == b'0' && this.next_at(1).is_some_and(|b| b == b'x') {
                this.mov();
                this.mov();
                this.skip(|b| b.is_ascii_hexdigit());

                Some(TokenKind::NumHexInt)
            } else {
                this.skip(|b| b.is_ascii_digit());

                if this.next().is_some_and(|b| b == b'.') && this.next_at(1).is_some_and(|b| b.is_ascii_digit()) {
                    this.mov();
                    this.mov();
                    this.skip(|b| b.is_ascii_digit());

                    Some(TokenKind::NumDecFloat)
                } else {
                    Some(TokenKind::NumDecInt)
                }
            }
        })
    }
    fn t_word(&mut self) -> Option<Token> {
        if !self.next_unwrap().is_ascii_alphabetic() || self.next_unwrap() != b'_' {
            return None;
        }

        self.make_token(|this| {
            match this.skip_and_get(|b| b.is_ascii_alphanumeric() || b == b'_') {
                b"as" => Some(TokenKind::OpAs),
                b"if" => Some(TokenKind::KwIf),
                b"or" => Some(TokenKind::KwOr),
                b"in" => Some(TokenKind::KwIn),
                b"for" => Some(TokenKind::KwFor),
                b"let" => Some(TokenKind::KwLet),
                b"var" => Some(TokenKind::KwVar),
                b"fun" => Some(TokenKind::KwFun),
                b"pub" => Some(TokenKind::KwPub),
                b"use" => Some(TokenKind::KwUse),
                b"type" => Some(TokenKind::KwType),
                b"enum" => Some(TokenKind::KwEnum),
                b"void" => Some(TokenKind::KwVoid),
                b"true" => Some(TokenKind::KwTrue),
                b"loop" => Some(TokenKind::KwLoop),
                b"while" => Some(TokenKind::KwWhile),
                b"trait" => Some(TokenKind::KwTrait),
                b"match" => Some(TokenKind::KwMatch),
                b"false" => Some(TokenKind::KwFalse),
                b"module" => Some(TokenKind::KwModule),
                b"struct" => Some(TokenKind::KwStruct),

                _ => Some(TokenKind::Identifier),
            }
        })
    }
    fn t_op(&mut self) -> Option<Token> {
        self.make_token(|this| match this.next_unwrap() {
            b'+' => match this.next_at(1) {
                Some(b'=') => {
                    this.mov();
                    this.mov();
                    Some(TokenKind::OpAddAsg)
                }
                _ => {
                    this.mov();
                    Some(TokenKind::OpAdd)
                }
            },
            b'-' => match this.next_at(1) {
                Some(b'>') => {
                    this.mov();
                    this.mov();
                    Some(TokenKind::OpLam)
                }
                Some(b'=') => {
                    this.mov();
                    this.mov();
                    Some(TokenKind::OpSubAsg)
                }
                _ => {
                    this.mov();
                    Some(TokenKind::OpSub)
                }
            },
            b'*' => match this.next_at(1) {
                Some(b'=') => {
                    this.mov();
                    this.mov();
                    Some(TokenKind::OpMulAsg)
                }
                _ => {
                    this.mov();
                    Some(TokenKind::OpMul)
                }
            },
            b'/' => match this.next_at(1) {
                Some(b'=') => {
                    this.mov();
                    this.mov();
                    Some(TokenKind::OpDivAsg)
                }
                _ => {
                    this.mov();
                    Some(TokenKind::OpDiv)
                }
            },
            b'%' => match this.next_at(1) {
                Some(b'=') => {
                    this.mov();
                    this.mov();
                    Some(TokenKind::OpModAsg)
                }
                _ => {
                    this.mov();
                    Some(TokenKind::OpMod)
                }
            },
            b'>' => match this.next_at(1) {
                Some(b'=') => {
                    this.mov();
                    this.mov();
                    Some(TokenKind::OpGe)
                }
                _ => {
                    this.mov();
                    Some(TokenKind::OpGt)
                }
            },
            b'<' => match this.next_at(1) {
                Some(b'=') => {
                    this.mov();
                    this.mov();
                    Some(TokenKind::OpLe)
                }
                _ => {
                    this.mov();
                    Some(TokenKind::OpLt)
                }
            },
            b'!' => match this.next_at(1) {
                Some(b'=') => {
                    this.mov();
                    this.mov();
                    Some(TokenKind::OpNe)
                }
                _ => {
                    this.mov();
                    Some(TokenKind::OpNot)
                }
            },
            b'=' => match this.next_at(1) {
                Some(b'=') => {
                    this.mov();
                    this.mov();
                    Some(TokenKind::OpEq)
                }
                _ => {
                    this.mov();
                    Some(TokenKind::OpAsg)
                }
            },
            b'|' => match this.next_at(1) {
                Some(b'=') => {
                    this.mov();
                    this.mov();
                    Some(TokenKind::OpOrAsg)
                }
                _ => {
                    this.mov();
                    Some(TokenKind::OpOr)
                }
            },
            b'&' => match this.next_at(1) {
                Some(b'=') => {
                    this.mov();
                    this.mov();
                    Some(TokenKind::OpAndAsg)
                }
                _ => {
                    this.mov();
                    Some(TokenKind::OpAnd)
                }
            },
            b'.' => match this.next_at(1) {
                Some(b'.') => {
                    this.mov();
                    this.mov();
                    Some(TokenKind::OpRng)
                }
                _ => {
                    this.mov();
                    Some(TokenKind::OpDot)
                }
            },
            b'?' => {
                this.mov();
                Some(TokenKind::OpErr)
            }
            _ => None,
        })
    }
    fn t_delim(&mut self) -> Option<Token> {
        self.make_token(|this| match this.next_unwrap() {
            b',' => {
                this.mov();
                Some(TokenKind::Comma)
            }
            b':' => {
                this.mov();
                Some(TokenKind::Colon)
            }
            b';' => {
                this.mov();
                Some(TokenKind::Semicolon)
            }
            b'(' => {
                this.mov();
                Some(TokenKind::RoundL)
            }
            b')' => {
                this.mov();
                Some(TokenKind::RoundR)
            }
            b'[' => {
                this.mov();
                Some(TokenKind::SquareL)
            }
            b']' => {
                this.mov();
                Some(TokenKind::SquareR)
            }
            b'{' => {
                this.mov();
                Some(TokenKind::CurvedL)
            }
            b'}' => {
                this.mov();
                Some(TokenKind::CurvedR)
            }
            _ => None,
        })
    }
    fn t_string(&mut self) -> Option<Token> {
        if self.next_unwrap() != b'"' {
            return None;
        }

        self.make_token(|this| {
            let mut escaped = false;
            this.mov();

            while let Some(b) = this.next() {
                if escaped {
                    escaped = false;
                } else if b == b'"' {
                    this.mov();
                    return Some(TokenKind::String);
                } else if b == b'\\' {
                    escaped = true;
                }
                this.mov();
            }
            Some(TokenKind::ErrStr)
        })
    }
    fn t_attribute(&mut self) -> Option<Token> {
        if self.next_unwrap() != b'@' {
            return None;
        }

        self.make_token(|this| {
            this.mov();
            if this.next().is_none_or(|b| !b.is_ascii_alphabetic() && b != b'_') {
                return Some(TokenKind::ErrAttr);
            }

            this.mov();
            this.skip(|b| b.is_ascii_alphanumeric() || b == b'_');
            Some(TokenKind::Attribute)
        })
    }
    fn t_doc(&mut self) -> Option<Token> {
        if     self.next_unwrap() == b'/'
            && self.next_at(1).is_some_and(|b| b == b'/')
            && self.next_at(2).is_some_and(|b| b == b'/')
        {
            self.make_token(|this| {
                this.mov();
                this.mov();
                this.mov();
                this.skip(|b| b != b'\n');
                Some(TokenKind::DocComment)
            })
        } else {
            None
        }
    }

    fn skip_error(&mut self) -> Option<Token> {
        self.make_token(|this| {
            this.mov();
            this.skip(|b| !b.is_ascii_alphanumeric() && b != b'_' && b != b'@' && b != b';');

            Some(TokenKind::ErrChar)
        })
    }
    fn skip_ignored(&mut self) {
        while let Some(b) = self.next() {
            if b.is_ascii_whitespace() {
                self.mov();
                continue;
            }
            // Ensure that we don't skip a doc comment
            if b == b'/' && self.next_at(1).is_some_and(|b| b == b'/') && self.next_at(2).is_none_or(|b| b != b'/') {
                self.mov();
                self.mov();
                self.skip(|b| b != b'\n');
                continue;
            }
            break;
        }
    }
    fn next_token(&mut self) -> Option<Token> {
        if self.next().is_none() {
            return None;
        }

                        self.t_word()
            .or_else(|| self.t_delim())
            .or_else(|| self.t_number())
            .or_else(|| self.t_doc()) // WARN: doc should go before op, to not match /// as three divisions
            .or_else(|| self.t_op())
            .or_else(|| self.t_string())
            .or_else(|| self.t_attribute())
            .or_else(|| self.skip_error())
    }
}
