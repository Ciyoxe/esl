pub mod combinators;

struct ParserScope<ScopeT> {
    pub begin_index: usize,
    pub data: ScopeT,
}

pub struct Parser<T, ScopeT = ()> {
    scopes: Vec<ParserScope<ScopeT>>,
    tokens: Vec<T>,
    index: usize,
}

impl<T, ScopeT> Parser<T, ScopeT> {
    pub fn new(tokens: Vec<T>) -> Self {
        Self {
            tokens,
            index: 0,
            scopes: Vec::new(),
        }
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

    pub fn test(&self, f: impl FnOnce(&T) -> bool) -> bool {
        self.peek().map_or(false, |token| f(token))
    }
    pub fn test_and_take(&mut self, f: impl FnOnce(&T) -> bool) -> Option<&T> {
        self.peek().filter(|token| f(token))
    }
    pub fn test_and_skip(&mut self, f: impl FnOnce(&T) -> bool) -> Option<&T> {
        if self.peek().filter(|token| f(token)).is_some() {
            // rust moment
            return self.next();
        }
        None
    }

    pub fn skip_while(&mut self, mut f: impl FnMut(&T) -> bool) {
        while let Some(token) = self.peek() {
            if f(token) {
                self.index += 1;
            } else {
                break;
            }
        }
    }

    pub fn enter_scope(&mut self, data: ScopeT) {
        self.scopes.push(ParserScope {
            data,
            begin_index: self.index,
        });
    }
    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn get_range(&self) -> std::ops::Range<usize> {
        self.scopes.last().unwrap().begin_index..self.index
    }
    pub fn take_range(&self) -> &[T] {
        &self.tokens[self.get_range()]
    }
    pub fn get_scope(&self) -> &ScopeT {
        &self.scopes.last().unwrap().data
    }
}
