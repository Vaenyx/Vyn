#[derive(Debug, Clone)]
pub struct Token {
    pub pure_token: PureToken,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct Span {
    pub start: u64, // byte offset
    pub end: u64,   // byte offset (exclusive)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    IntLiteral(IntLiteralToken),
    FloatLiteral(FloatLiteralToken),
    BoolLiteral(BoolLiteralToken),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int(TypeIntToken),
    Float(TypeFloatToken),
    Bool(TypeBoolToken),
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

    If(KeywordIfToken),
    Elif(KeywordElifToken),
    Else(KeywordElseToken),

    While(KeywordWhileToken),
    For(KeywordForToken),
    In(KeywordInToken),
    Where(KeywordWhereToken),

    Local(KeywordLocalToken),
    Global(KeywordGlobalToken),

    Mut(KeywordMutToken),
    Const(KeywordConstToken),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PureToken {
    Ident(IdentToken),
    Literal(Literal),
    Keyword(Keyword),
    Type(Type),
    Comment(Comment),

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

    Assign(AssignToken),

    Plus(PlusToken),
    Minus(MinusToken),
    Star(StarToken),
    Slash(SlashToken),

    EqEq(EqEqToken),
    NotEq(NotEqToken),
    Greater(GreaterToken),
    GreaterEq(GreaterEqToken),
    Less(LessToken),
    LessEq(LessEqToken),

    And(AndToken),
    Or(OrToken),
    Xor(XorToken),

    NewLine(NewLineToken),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentToken {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntLiteralToken {
    pub value: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FloatLiteralToken {
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoolLiteralToken {
    pub value: bool,
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
pub struct AndToken {} // &
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrToken {} // |
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XorToken {} // ||

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewLineToken {} // end of line
