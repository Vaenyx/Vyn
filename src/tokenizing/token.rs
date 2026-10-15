#[derive(Debug, Clone)]
pub struct Token {
    pub pure_token: PureToken,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct Span {
    pub start: usize, // byte offset
    pub end: usize,   // byte offset (exclusive)
}

#[derive(Debug, Clone, PartialEq)]
pub enum PureToken {
    Ident(IdentToken),
    Literal(Literal),
    Keyword(Keyword),
    Type(Type),
    Comment(Comment),
    Operator(Operator),
    Delimiter(Delimiter),
    NewLine(NewLineToken),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(IntLiteralToken),
    Float(FloatLiteralToken),
    Bool(BoolLiteralToken),
    String(StringLiteralToken),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int(TypeIntToken),
    Float(TypeFloatToken),
    Bool(TypeBoolToken),
    Str(TypeStrToken),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Comment {
    LineComment(LineCommentToken),
    BlockComment(BlockCommentToken),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Fn(KeywordFnToken),
    Return(KeywordReturnToken),

    ControlFlow(ControlFlow),

    Locality(Locality),
    Mutability(Mutability),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ControlFlow {
    If(KeywordIfToken),
    Elif(KeywordElifToken),
    Else(KeywordElseToken),

    While(KeywordWhileToken),
    For(KeywordForToken),
    In(KeywordInToken),
    Where(KeywordWhereToken),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Locality {
    Local(KeywordLocalToken),
    Global(KeywordGlobalToken),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Mutability {
    Mut(KeywordMutToken),
    Const(KeywordConstToken),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CombinationOperator {
    LogicalAnd(LogicalAndToken),
    LogicalOr(LogicalOrToken),

    BitwiseAnd(BitwiseAndToken),
    BitwiseOr(BitwiseOrToken),

    Not(NotToken),
    Xor(XorToken),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonOperator {
    EqEq(EqEqToken),
    NotEq(NotEqToken),
    Greater(GreaterToken),
    GreaterEq(GreaterEqToken),
    Less(LessToken),
    LessEq(LessEqToken),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModificationOperator {
    Plus(PlusToken),
    Minus(MinusToken),
    Star(StarToken),
    Slash(SlashToken),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Assign(AssignToken),

    Modification(ModificationOperator),
    Comparison(ComparisonOperator),
    Combination(CombinationOperator),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Delimiter {
    LParen(LParenToken),
    RParen(RParenToken),
    LBrace(LBraceToken),
    RBrace(RBraceToken),
    LBracket(LBracketToken),
    RBracket(RBracketToken),
    Comma(CommaToken),
    Colon(ColonToken),
    Arrow(ArrowToken),
    Dot(DotToken),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentToken {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntLiteralToken {
    pub value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FloatLiteralToken {
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoolLiteralToken {
    pub value: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringLiteralToken {
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineCommentToken {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockCommentToken {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordFnToken {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordReturnToken {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordIfToken {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordElifToken {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordElseToken {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordWhileToken {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordForToken {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordInToken {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordWhereToken {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordLocalToken {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordGlobalToken {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordMutToken {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordConstToken {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeIntToken {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeFloatToken {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeBoolToken {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeStrToken {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LParenToken {} // (
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RParenToken {} // )
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LBraceToken {} // {
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RBraceToken {} // }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LBracketToken {} // [
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RBracketToken {} // ]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommaToken {} // ,
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColonToken {} // :
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrowToken {} // ->
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DotToken {} // .

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignToken {} // =

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlusToken {} // +
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MinusToken {} // -
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StarToken {} // *
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlashToken {} // /

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EqEqToken {} // ==
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotEqToken {} // !=
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GreaterToken {} // >
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GreaterEqToken {} // >=
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LessToken {} // <
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LessEqToken {} // <=

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalAndToken {} // &&
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalOrToken {} // ||

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitwiseAndToken {} // &
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitwiseOrToken {} // |
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotToken {} // !
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XorToken {} // ||

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewLineToken {} // end of line
