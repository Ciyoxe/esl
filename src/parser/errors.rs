#[derive(Debug, Clone)]
pub enum ParsingError {
    // int bigger than 2^64
    IntegerOverflow,
    // (a + !)
    NoOperandAfterPrefixOperator,
    // (a + ?)
    NoOperandBeforePostfixOperation,
    // (+ b)
    NoOperandBeforeInfixOperation,
    // (a +)
    NoOperandAfterInfixOperation,
    // f(a, b, c ####)
    UnexpectedCallArgument,
    // (a + b ####)
    UnexpectedToken,
}
