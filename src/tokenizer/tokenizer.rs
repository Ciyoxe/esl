use super::token::{Token, TokenKind};

pub struct Tokenizer<'a> {
    // assert that files will be less than 4gb
    pub pos: u32,
    pub src: &'a [u8],
}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a [u8]) -> Self {
        Tokenizer { pos: 0, src }
    }

    fn mov(&mut self) {
        self.pos += 1;
    }
    fn skip(&mut self, f: impl Fn(u8) -> bool) {
        while self.src.get(self.pos as usize).copied().is_some_and(&f) {
            self.mov();
        }
    }
    fn skip_and_get(&mut self, f: impl Fn(u8) -> bool) -> &[u8] {
        let start = self.pos;
        self.skip(f);
        &self.src[start as usize..self.pos as usize]
    }

    fn next(&self) -> Option<u8> {
        self.src.get(self.pos as usize).copied()
    }
    fn test(&self, f: impl FnOnce(u8) -> bool) -> bool {
        self.src.get(self.pos as usize).copied().is_some_and(f)
    }
    fn next_at(&self, pos: u32) -> Option<u8> {
        self.src.get(self.pos as usize + pos as usize).copied()
    }
    fn next_unchecked(&self) -> u8 {
        unsafe { self.src.get_unchecked(self.pos as usize).clone() }
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


impl<'a> Tokenizer<'a> {
    fn t_number(&mut self) -> Option<Token> {
        if !self.test(|b| b.is_ascii_digit()) {
            return None;
        }

        self.make_token(|this| {
            if this.next_unchecked() == b'0' && this.next_at(1).is_some_and(|b| b == b'b') {
                this.mov();
                this.mov();
                this.skip(|b| b == b'0' || b == b'1');

                Some(TokenKind::NumBinInt)
            } else if this.next_unchecked() == b'0' && this.next_at(1).is_some_and(|b| b == b'x') {
                this.mov();
                this.mov();
                this.skip(|b| b.is_ascii_hexdigit());

                Some(TokenKind::NumHexInt)
            } else {
                this.skip(|b| b.is_ascii_digit());

                if this.test(|b| b == b'.') && this.next_at(1).is_some_and(|b| b.is_ascii_digit()) {
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
        if !self.test(|b| b.is_ascii_alphabetic() || b == b'_') {
            return None;
        }

        self.make_token(|this| {
            match this.skip_and_get(|b| b.is_ascii_alphanumeric() || b == b'_') {
                b"as" => Some(TokenKind::OpAs),
                b"if" => Some(TokenKind::KwIf),
                b"el" => Some(TokenKind::KwEl),
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
        if self.next().is_none() {
            return None;
        }

        self.make_token(|this| match this.next() {
            Some(b'+') => match this.next_at(1) {
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
            Some(b'-') => match this.next_at(1) {
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
            Some(b'*') => match this.next_at(1) {
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
            Some(b'/') => match this.next_at(1) {
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
            Some(b'%') => match this.next_at(1) {
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
            Some(b'>') => match this.next_at(1) {
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
            Some(b'<') => match this.next_at(1) {
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
            Some(b'!') => match this.next_at(1) {
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
            Some(b'=') => match this.next_at(1) {
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
            Some(b'|') => match this.next_at(1) {
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
            Some(b'&') => match this.next_at(1) {
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
            Some(b'.') => match this.next_at(1) {
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
            Some(b'?') => {
                this.mov();
                Some(TokenKind::OpErr)
            }
            _ => None,
        })
    }
    fn t_delim(&mut self) -> Option<Token> {
        if self.next().is_none() {
            return None;
        }

        self.make_token(|this| match this.next() {
            Some(b',') => {
                this.mov();
                Some(TokenKind::Comma)
            }
            Some(b':') => {
                this.mov();
                Some(TokenKind::Colon)
            }
            Some(b';') => {
                this.mov();
                Some(TokenKind::Semicolon)
            }
            Some(b'(') => {
                this.mov();
                Some(TokenKind::RoundL)
            }
            Some(b')') => {
                this.mov();
                Some(TokenKind::RoundR)
            }
            Some(b'[') => {
                this.mov();
                Some(TokenKind::SquareL)
            }
            Some(b']') => {
                this.mov();
                Some(TokenKind::SquareR)
            }
            Some(b'{') => {
                this.mov();
                Some(TokenKind::CurvedL)
            }
            Some(b'}') => {
                this.mov();
                Some(TokenKind::CurvedR)
            }
            _ => None,
        })
    }
    fn t_string(&mut self) -> Option<Token> {
        if !self.test(|b| b == b'"') {
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
        if !self.test(|b| b == b'@') {
            return None;
        }

        self.make_token(|this| {
            this.mov();
            if !this.test(|b| b.is_ascii_alphabetic() || b == b'_') {
                return Some(TokenKind::ErrAttr);
            }
            this.mov();
            this.skip(|b| b.is_ascii_alphanumeric() || b == b'_');

            Some(TokenKind::Attribute)
        })
    }
    fn t_doc(&mut self) -> Option<Token> {
        if self.next().is_some_and(|b| b == b'/')
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
        if self.next().is_none() {
            return None;
        }
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
            if b == b'/' && self.next_at(1).is_some_and(|b| b == b'/') {
                self.mov();
                self.mov();
                self.skip(|b| b != b'\n');
                continue;
            }
            break;
        }
    }
    fn next_token(&mut self) -> Option<Token> {
                        self.t_op()
            .or_else(|| self.t_word())
            .or_else(|| self.t_number())
            .or_else(|| self.t_delim())
            .or_else(|| self.t_string())
            .or_else(|| self.t_attribute())
            .or_else(|| self.t_doc())
            .or_else(|| self.skip_error())
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            self.skip_ignored();
            if let Some(tok) = self.next_token() {
                tokens.push(tok);
            } else {
                break tokens;
            }
        }
    }
}
