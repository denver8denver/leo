use crate::errors::{BooleanError, FieldElementError, FunctionError, IntegerError, ValueError};

#[derive(Debug, Error)]
pub enum ExpressionError {
    // Variables
    #[error("Variable \"{}\" not found", _0)]
    UndefinedVariable(String),

    // Types
    #[error("{}", _0)]
    IncompatibleTypes(String),

    #[error("{}", _0)]
    ValueError(ValueError),

    #[error("{}", _0)]
    IntegerError(IntegerError),

    #[error("{}", _0)]
    FieldElementError(FieldElementError),

    #[error("{}", _0)]
    BooleanError(BooleanError),

    #[error("Exponent must be an integer, got field {}", _0)]
    InvalidExponent(String),

    // Arrays
    #[error(
        "Array {} must be declared before it is used in an inline expression",
        _0
    )]
    UndefinedArray(String),

    #[error("Cannot access array {}", _0)]
    InvalidArrayAccess(String),

    #[error("Spread should contain an array, got {}", _0)]
    InvalidSpread(String),

    #[error("Index must resolve to an integer, got {}", _0)]
    InvalidIndex(String),

    // Circuits
    #[error(
        "Circuit {} must be declared before it is used in an inline expression",
        _0
    )]
    UndefinedCircuit(String),

    #[error("Circuit object {} does not exist", _0)]
    UndefinedCircuitObject(String),

    #[error("Cannot assign to circuit functions")]
    InvalidCircuitValue,

    #[error("Expected circuit object {}, got {}", _0, _1)]
    InvalidCircuitObject(String, String),

    #[error("Cannot access circuit {}", _0)]
    InvalidCircuitAccess(String),

    // Functions
    #[error(
        "Function {} must be declared before it is used in an inline expression",
        _0
    )]
    UndefinedFunction(String),

    #[error("Cannot evaluate function call")]
    FunctionError(Box<FunctionError>),

    #[error("Inline function call to {} did not return", _0)]
    FunctionDidNotReturn(String),

    // Conditionals
    #[error("If, else conditional must resolve to a boolean, got {}", _0)]
    IfElseConditional(String),
}

impl From<ValueError> for ExpressionError {
    fn from(error: ValueError) -> Self {
        ExpressionError::ValueError(error)
    }
}

impl From<IntegerError> for ExpressionError {
    fn from(error: IntegerError) -> Self {
        ExpressionError::IntegerError(error)
    }
}

impl From<FieldElementError> for ExpressionError {
    fn from(error: FieldElementError) -> Self {
        ExpressionError::FieldElementError(error)
    }
}

impl From<BooleanError> for ExpressionError {
    fn from(error: BooleanError) -> Self {
        ExpressionError::BooleanError(error)
    }
}

impl From<Box<FunctionError>> for ExpressionError {
    fn from(error: Box<FunctionError>) -> Self {
        ExpressionError::FunctionError(error)
    }
}
