pub trait Ranged {
    fn set_range(&mut self, range: std::ops::Range<usize>);
}

pub struct ParserScope {
    pub begin_index: usize,
    pub end_index: usize,
}

pub struct Parser<T> {
    scopes: Vec<ParserScope>,
    tokens: Vec<T>,
    index: usize,
}

impl<T> Parser<T> {
    pub fn new(tokens: Vec<T>) -> Self {
        Self { tokens, index: 0, scopes: Vec::new() }
    }
    pub fn end(&self) -> bool {
        self.index >= self.tokens.len()
    }
    pub fn next(&mut self) -> Option<&T> {
        self.index += 1;
        self.tokens.get(self.index - 1)
    }
    pub fn peek(&self) -> Option<&T> {
        self.tokens.get(self.index)
    }
    pub fn peek_at(&self, index: usize) -> Option<&T> {
        self.tokens.get(self.index + index)
    }

    pub fn enter_scope(&mut self) {
        self.index = 0;
    }
    pub fn exit_scope(&mut self) {
        self.index = 0;
    }
}

pub fn repeat<T, O, E>(parser: &mut Parser<T>, mut f: impl FnMut(&mut Parser<T>) -> Result<O, E>) -> Result<Vec<O>, E> {
    let mut result = Vec::new();
    parser.parse(|parser| {
        let mut result = Vec::new();
        f(parser)?;
        f(parser)?;
        Ok(result)
    })?;
    Ok(result)
}
