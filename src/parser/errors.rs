#[derive(Debug, Clone)]
pub enum ParsingError {
    IntegerOverflow,
    // (a + !)
    NoOperandAfterPrefixOperator,
    // (a + ?)
    PostfixOperatorWithoutOperand,
    // (+ b)
    InfixOperationWithoutLeftOperand,
    // (a +)
    NoOperandAfterInfixOperation,
    // f(a, b, c ####)
    UnexpectedCallArgument,
}
