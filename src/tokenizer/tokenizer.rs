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

    pub fn mov(&mut self) {
        self.pos += 1;
    }
    pub fn skip(&mut self, f: impl Fn(u8) -> bool) {
        while self.src.get(self.pos as usize).copied().is_some_and(&f) {
            self.mov();
        }
    }
    pub fn skip_and_get(&mut self, f: impl Fn(u8) -> bool) -> &[u8] {
        let start = self.pos;
        self.skip(f);
        &self.src[start as usize..self.pos as usize]
    }

    pub fn next(&self) -> Option<u8> {
        self.src.get(self.pos as usize).copied()
    }
    pub fn test(&self, f: impl FnOnce(u8) -> bool) -> bool {
        self.src.get(self.pos as usize).copied().is_some_and(f)
    }
    pub fn next_at(&self, pos: u32) -> Option<u8> {
        self.src.get(self.pos as usize + pos as usize).copied()
    }
    pub fn next_unchecked(&self) -> u8 {
        unsafe { self.src.get_unchecked(self.pos as usize).clone() }
    }

    pub fn make_token(&mut self, f: impl FnOnce(&mut Self) -> Option<TokenKind>) -> Option<Token> {
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
