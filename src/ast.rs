enum Width {
    Scalar,
    Vector(Option<usize>),
}

enum IntegerPrimitive {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
}

enum FloatPrimitive {
    F32,
    F64,
}

enum Primitive {
    Float(FloatPrimitive),
    Integer(IntegerPrimitive),
}

struct UnsignedInteger(String);

struct Integer {
    value: String,
    primitive: Option<IntegerPrimitive>,
}

struct Float {
    value: String,
    primitive: Option<FloatPrimitive>,
}

struct Number {
    value: String,
    primitive: Option<Primitive>,
}

enum Value {
    Expression(Box<Expression>),
    Symbol(String),
    Number(Number),
}

enum Expression {
    Define(DefExpression),
    Function(FnExpression),
    Export(ExportExpression),
    Call(CallExpression),
}

struct DefExpression {
    symbol: String,
    value: Value,
}

struct FnArg {
    symbol: String,
    primitive: Option<Primitive>,
    width: Option<Width>,
}

struct FnExpression {
    args: Vec<FnArg>,
    body: Vec<Value>,
}

struct ExportArg {
    name: String,
    primitive: Primitive,
    width: Width,
}

struct ExportExpression {
    export_symbol: String,
    args: Vec<ExportArg>,
    value: Value,
}

struct CallExpression {
    values: Vec<Value>,
}

struct Program(Vec<Expression>);
