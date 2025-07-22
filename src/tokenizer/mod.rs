pub mod token;

use token::{Token, TokenKind};

use crate::tokenizer::token::{ErrorToken, KeywordToken, OperationToken, ScopeToken};

pub struct Tokenizer<'a> {
    pub pos: usize,
    pub src: &'a [u8],
}

/*************************************************
 *               PUBLIC INTERFACE                *
 *************************************************/
impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a [u8]) -> Self {
        Tokenizer { src, pos: 0 }
    }
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            self.skip_ignored();
            match self.next_token() {
                None => break,
                Some(t) => tokens.push(t),
            }
        }
        tokens
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
    fn capture_bytes(&mut self, f: impl FnOnce(&mut Self) -> ()) -> &[u8] {
        let start = self.pos;
        f(self);
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
            // 0b0101
            if this.next_unwrap() == b'0' && this.next_at(1).is_some_and(|b| b == b'b') {
                let bytes = this.capture_bytes(|this| {
                    this.mov();
                    this.mov();
                    this.skip(|b| b == b'0' || b == b'1');
                });

                match std::str::from_utf8(bytes) {
                    Ok(str) => match u64::from_str_radix(str, 2) {
                        Ok(num) => Some(TokenKind::Integer(num)),
                        Err(_) => Some(TokenKind::Error(ErrorToken::InvalidNumber)),
                    },
                    Err(_) => Some(TokenKind::Error(ErrorToken::InvalidByteSequence)),
                }
            // 0x123ABC
            } else if this.next_unwrap() == b'0' && this.next_at(1).is_some_and(|b| b == b'x') {
                let bytes = this.capture_bytes(|this| {
                    this.mov();
                    this.mov();
                    this.skip(|b| b.is_ascii_hexdigit());
                });

                match std::str::from_utf8(bytes) {
                    Ok(str) => match u64::from_str_radix(str, 16) {
                        Ok(num) => Some(TokenKind::Integer(num)),
                        Err(_) => Some(TokenKind::Error(ErrorToken::InvalidNumber)),
                    },
                    Err(_) => Some(TokenKind::Error(ErrorToken::InvalidByteSequence)),
                }
            }
            // 12345.678
            else {
                let mut is_floating_number = false;

                let bytes = this.capture_bytes(|this| {
                    this.skip(|b| b.is_ascii_digit());

                    if this.next().is_some_and(|b| b == b'.')
                        && this.next_at(1).is_some_and(|b| b.is_ascii_digit())
                    {
                        this.mov();
                        this.mov();
                        this.skip(|b| b.is_ascii_digit());

                        is_floating_number = true;
                    };
                });

                match std::str::from_utf8(bytes) {
                    Ok(str) => {
                        if is_floating_number {
                            match str.parse::<f64>() {
                                Ok(num) => Some(TokenKind::Floating(num)),
                                Err(_) => Some(TokenKind::Error(ErrorToken::InvalidNumber)),
                            }
                        } else {
                            match str.parse::<u64>() {
                                Ok(num) => Some(TokenKind::Integer(num)),
                                Err(_) => Some(TokenKind::Error(ErrorToken::InvalidNumber)),
                            }
                        }
                    }
                    Err(_) => Some(TokenKind::Error(ErrorToken::InvalidByteSequence)),
                }
            }
        })
    }

    fn t_word(&mut self) -> Option<Token> {
        if !self.next_unwrap().is_ascii_alphabetic() && self.next_unwrap() != b'_' {
            return None;
        }

        self.make_token(|this| {
            match this.capture_bytes(|this| this.skip(|b| b.is_ascii_alphanumeric() || b == b'_')) {
                b"_" => Some(TokenKind::Ignore),

                b"as" => Some(TokenKind::Operation(OperationToken::As)),
                b"ref" => Some(TokenKind::Operation(OperationToken::Ref)),

                b"true" => Some(TokenKind::Boolean(true)),
                b"false" => Some(TokenKind::Boolean(false)),

                b"if" => Some(TokenKind::Keyword(KeywordToken::If)),
                b"or" => Some(TokenKind::Keyword(KeywordToken::Or)),
                b"in" => Some(TokenKind::Keyword(KeywordToken::In)),
                b"for" => Some(TokenKind::Keyword(KeywordToken::For)),
                b"let" => Some(TokenKind::Keyword(KeywordToken::Let)),
                b"var" => Some(TokenKind::Keyword(KeywordToken::Var)),
                b"fun" => Some(TokenKind::Keyword(KeywordToken::Fun)),
                b"pub" => Some(TokenKind::Keyword(KeywordToken::Pub)),
                b"use" => Some(TokenKind::Keyword(KeywordToken::Use)),
                b"enum" => Some(TokenKind::Keyword(KeywordToken::Enum)),
                b"type" => Some(TokenKind::Keyword(KeywordToken::Type)),
                b"loop" => Some(TokenKind::Keyword(KeywordToken::Loop)),
                b"trait" => Some(TokenKind::Keyword(KeywordToken::Trait)),
                b"while" => Some(TokenKind::Keyword(KeywordToken::While)),
                b"match" => Some(TokenKind::Keyword(KeywordToken::Match)),
                b"struct" => Some(TokenKind::Keyword(KeywordToken::Struct)),
                b"module" => Some(TokenKind::Keyword(KeywordToken::Module)),

                bytes => match std::str::from_utf8(bytes) {
                    Ok(str) => Some(TokenKind::Identifier(str.to_owned())),
                    Err(_) => Some(TokenKind::Error(ErrorToken::InvalidByteSequence)),
                },
            }
        })
    }
    fn t_op(&mut self) -> Option<Token> {
        self.make_token(|this| match this.next_unwrap() {
            b'+' => match this.next_at(1) {
                Some(b'=') => {
                    this.mov();
                    this.mov();
                    Some(TokenKind::Operation(OperationToken::AddAsg))
                }
                _ => {
                    this.mov();
                    Some(TokenKind::Operation(OperationToken::Add))
                }
            },
            b'-' => match this.next_at(1) {
                Some(b'>') => {
                    this.mov();
                    this.mov();
                    Some(TokenKind::Operation(OperationToken::Lam))
                }
                Some(b'=') => {
                    this.mov();
                    this.mov();
                    Some(TokenKind::Operation(OperationToken::SubAsg))
                }
                _ => {
                    this.mov();
                    Some(TokenKind::Operation(OperationToken::Sub))
                }
            },
            b'*' => match this.next_at(1) {
                Some(b'=') => {
                    this.mov();
                    this.mov();
                    Some(TokenKind::Operation(OperationToken::MulAsg))
                }
                _ => {
                    this.mov();
                    Some(TokenKind::Operation(OperationToken::Mul))
                }
            },
            b'/' => match this.next_at(1) {
                Some(b'=') => {
                    this.mov();
                    this.mov();
                    Some(TokenKind::Operation(OperationToken::DivAsg))
                }
                _ => {
                    this.mov();
                    Some(TokenKind::Operation(OperationToken::Div))
                }
            },
            b'%' => match this.next_at(1) {
                Some(b'=') => {
                    this.mov();
                    this.mov();
                    Some(TokenKind::Operation(OperationToken::ModAsg))
                }
                _ => {
                    this.mov();
                    Some(TokenKind::Operation(OperationToken::Mod))
                }
            },
            b'>' => match this.next_at(1) {
                Some(b'=') => {
                    this.mov();
                    this.mov();
                    Some(TokenKind::Operation(OperationToken::Ge))
                }
                _ => {
                    this.mov();
                    Some(TokenKind::Operation(OperationToken::Gt))
                }
            },
            b'<' => match this.next_at(1) {
                Some(b'=') => {
                    this.mov();
                    this.mov();
                    Some(TokenKind::Operation(OperationToken::Le))
                }
                _ => {
                    this.mov();
                    Some(TokenKind::Operation(OperationToken::Lt))
                }
            },
            b'!' => match this.next_at(1) {
                Some(b'=') => {
                    this.mov();
                    this.mov();
                    Some(TokenKind::Operation(OperationToken::Ne))
                }
                _ => {
                    this.mov();
                    Some(TokenKind::Operation(OperationToken::Not))
                }
            },
            b'=' => match this.next_at(1) {
                Some(b'=') => {
                    this.mov();
                    this.mov();
                    Some(TokenKind::Operation(OperationToken::Eq))
                }
                _ => {
                    this.mov();
                    Some(TokenKind::Operation(OperationToken::Asg))
                }
            },
            b'|' => match this.next_at(1) {
                Some(b'=') => {
                    this.mov();
                    this.mov();
                    Some(TokenKind::Operation(OperationToken::OrAsg))
                }
                _ => {
                    this.mov();
                    Some(TokenKind::Operation(OperationToken::Or))
                }
            },
            b'&' => match this.next_at(1) {
                Some(b'=') => {
                    this.mov();
                    this.mov();
                    Some(TokenKind::Operation(OperationToken::AndAsg))
                }
                _ => {
                    this.mov();
                    Some(TokenKind::Operation(OperationToken::And))
                }
            },
            b'.' => match this.next_at(1) {
                Some(b'.') => {
                    this.mov();
                    this.mov();
                    Some(TokenKind::Operation(OperationToken::Rng))
                }
                _ => {
                    this.mov();
                    Some(TokenKind::Operation(OperationToken::Dot))
                }
            },
            b':' => {
                this.mov();
                Some(TokenKind::Operation(OperationToken::Typedef))
            }
            b'?' => {
                this.mov();
                Some(TokenKind::Operation(OperationToken::Catch))
            }
            b',' => {
                this.mov();
                Some(TokenKind::Operation(OperationToken::Comma))
            }
            _ => None,
        })
    }
    fn t_scope(&mut self) -> Option<Token> {
        self.make_token(|this| match this.next_unwrap() {
            b'(' => {
                this.mov();
                let children = this.tokenize();

                if this.next().is_none_or(|b| b != b')') {
                    Some(TokenKind::Error(ErrorToken::MissingClosingRound))
                } else {
                    Some(TokenKind::Scope(ScopeToken::RoundBraces(children)))
                }
            }
            b'[' => {
                this.mov();
                let children = this.tokenize();

                if this.next().is_none_or(|b| b != b']') {
                    Some(TokenKind::Error(ErrorToken::MissingClosingSquare))
                } else {
                    Some(TokenKind::Scope(ScopeToken::SquareBraces(children)))
                }
            }
            b'{' => {
                this.mov();
                let children = this.tokenize();

                if this.next().is_none_or(|b| b != b'}') {
                    Some(TokenKind::Error(ErrorToken::MissingClosingCurly))
                } else {
                    Some(TokenKind::Scope(ScopeToken::CurlyBraces(children)))
                }
            }
            b')' => {
                this.mov();
                Some(TokenKind::Error(ErrorToken::RedundantClosingRound))
            }
            b']' => {
                this.mov();
                Some(TokenKind::Error(ErrorToken::RedundantClosingSquare))
            }
            b'}' => {
                this.mov();
                Some(TokenKind::Error(ErrorToken::RedundantClosingCurly))
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
            Some(TokenKind::ErrUnterminatedString)
        })
    }
    fn t_attribute(&mut self) -> Option<Token> {
        if self.next_unwrap() != b'@' {
            return None;
        }

        self.make_token(|this| {
            this.mov();
            if this
                .next()
                .is_none_or(|b| !b.is_ascii_alphabetic() && b != b'_')
            {
                return Some(TokenKind::ErrAttributeName);
            }

            this.mov();
            this.skip(|b| b.is_ascii_alphanumeric() || b == b'_');
            Some(TokenKind::Attribute)
        })
    }
    fn t_doc(&mut self) -> Option<Token> {
        if self.next_unwrap() == b'/'
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

            Some(TokenKind::ErrUnexpectedChar)
        })
    }
    fn skip_ignored(&mut self) {
        while let Some(b) = self.next() {
            if b.is_ascii_whitespace() {
                self.mov();
                continue;
            }
            // Ensure that we don't skip a doc comment
            if b == b'/'
                && self.next_at(1).is_some_and(|b| b == b'/')
                && self.next_at(2).is_none_or(|b| b != b'/')
            {
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
        self.t_scope()
            .or_else(|| self.t_word())
            .or_else(|| self.t_number())
            .or_else(|| self.t_doc()) // WARN: doc should go before op, to not match /// as three divisions
            .or_else(|| self.t_op())
            .or_else(|| self.t_string())
            .or_else(|| self.t_attribute())
            .or_else(|| self.skip_error())
    }
}
