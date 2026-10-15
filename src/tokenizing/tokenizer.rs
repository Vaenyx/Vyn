use std::{iter::Peekable, str::CharIndices};

use crate::tokenizing::{
    Token,
    token::{
        ArrowToken, AssignToken, BitwiseAndToken, BitwiseOrToken, BlockCommentToken,
        BoolLiteralToken, ColonToken, CombinationOperator, CommaToken, Comment, ComparisonOperator,
        ControlFlow, Delimiter, DotToken, EqEqToken, FloatLiteralToken, GreaterEqToken,
        GreaterToken, IdentToken, IntLiteralToken, Keyword, KeywordConstToken, KeywordElifToken,
        KeywordElseToken, KeywordFnToken, KeywordForToken, KeywordGlobalToken, KeywordIfToken,
        KeywordInToken, KeywordLocalToken, KeywordMutToken, KeywordReturnToken, KeywordWhereToken,
        KeywordWhileToken, LBraceToken, LBracketToken, LParenToken, LessEqToken, LessToken,
        LineCommentToken, Literal, Locality, LogicalAndToken, LogicalOrToken, MinusToken,
        ModificationOperator, Mutability, NotEqToken, NotToken, Operator, PlusToken, PureToken,
        RBraceToken, RBracketToken, RParenToken, SlashToken, Span, StarToken, StringLiteralToken,
        Type, TypeBoolToken, TypeFloatToken, TypeIntToken, TypeStrToken, XorToken,
    },
};

type Lexer<'a> = Peekable<CharIndices<'a>>;

pub fn tokenize(vyn_code: &str) -> Vec<Token> {
    let mut it = vyn_code.char_indices().peekable();
    let mut tokens = Vec::new();

    while let Some(tok) = next_token(&mut it, vyn_code.len()) {
        tokens.push(tok);
    }

    println!("tokens: {tokens:#?}");
    tokens
}

fn next_token(it: &mut Lexer<'_>, source_len: usize) -> Option<Token> {
    skip_whitespaces(it);

    let (start, ch) = it.next()?;

    let pure_token = match ch {
        '/' if matches!(it.peek(), Some((_, '/'))) => {
            it.next();
            PureToken::Comment(Comment::LineComment(LineCommentToken {
                text: lex_until_newline(it),
            }))
        }
        '/' if matches!(it.peek(), Some((_, '*'))) => {
            it.next();
            PureToken::Comment(Comment::BlockComment(BlockCommentToken {
                text: lex_block_comment(it),
            }))
        }

        '"' | '\'' | '´' | '`' => PureToken::Literal(Literal::String(StringLiteralToken {
            value: lex_string(it, ch),
        })),

        ch @ '0'..='9' => lex_number(it, ch),

        '(' => PureToken::Delimiter(Delimiter::LParen(LParenToken {})),
        ')' => PureToken::Delimiter(Delimiter::RParen(RParenToken {})),
        '[' => PureToken::Delimiter(Delimiter::LBrace(LBraceToken {})),
        ']' => PureToken::Delimiter(Delimiter::RBrace(RBraceToken {})),
        '{' => PureToken::Delimiter(Delimiter::LBracket(LBracketToken {})),
        '}' => PureToken::Delimiter(Delimiter::RBracket(RBracketToken {})),

        ',' => PureToken::Delimiter(Delimiter::Comma(CommaToken {})),
        ':' => PureToken::Delimiter(Delimiter::Colon(ColonToken {})),
        '=' if matches!(it.peek(), Some((_, '>'))) => {
            it.next();

            PureToken::Delimiter(Delimiter::Arrow(ArrowToken {}))
        }
        '.' => PureToken::Delimiter(Delimiter::Dot(DotToken {})),

        '=' if matches!(it.peek(), Some((_, '='))) => {
            it.next();
            PureToken::Operator(Operator::Comparison(ComparisonOperator::EqEq(EqEqToken {})))
        }
        '!' if matches!(it.peek(), Some((_, '='))) => {
            it.next();
            PureToken::Operator(Operator::Comparison(ComparisonOperator::NotEq(
                NotEqToken {},
            )))
        }
        '>' if matches!(it.peek(), Some((_, '='))) => {
            it.next();

            PureToken::Operator(Operator::Comparison(ComparisonOperator::GreaterEq(
                GreaterEqToken {},
            )))
        }
        '>' => PureToken::Operator(Operator::Comparison(ComparisonOperator::Greater(
            GreaterToken {},
        ))),
        '<' if matches!(it.peek(), Some((_, '='))) => {
            it.next();

            PureToken::Operator(Operator::Comparison(ComparisonOperator::LessEq(
                LessEqToken {},
            )))
        }
        '<' => PureToken::Operator(Operator::Comparison(ComparisonOperator::Less(LessToken {}))),

        '=' => PureToken::Operator(Operator::Assign(AssignToken {})),

        '+' => PureToken::Operator(Operator::Modification(ModificationOperator::Plus(
            PlusToken {},
        ))),
        '-' => PureToken::Operator(Operator::Modification(ModificationOperator::Minus(
            MinusToken {},
        ))),
        '*' => PureToken::Operator(Operator::Modification(ModificationOperator::Star(
            StarToken {},
        ))),
        '/' => PureToken::Operator(Operator::Modification(ModificationOperator::Slash(
            SlashToken {},
        ))),

        '&' if matches!(it.peek(), Some((_, '&'))) => {
            it.next();
            PureToken::Operator(Operator::Combination(CombinationOperator::LogicalAnd(
                LogicalAndToken {},
            )))
        }
        '|' if matches!(it.peek(), Some((_, '|'))) => {
            it.next();
            PureToken::Operator(Operator::Combination(CombinationOperator::LogicalOr(
                LogicalOrToken {},
            )))
        }
        '&' => PureToken::Operator(Operator::Combination(CombinationOperator::BitwiseAnd(
            BitwiseAndToken {},
        ))),
        '|' => PureToken::Operator(Operator::Combination(CombinationOperator::BitwiseOr(
            BitwiseOrToken {},
        ))),
        '!' => PureToken::Operator(Operator::Combination(CombinationOperator::Not(NotToken {}))),
        '^' => PureToken::Operator(Operator::Combination(CombinationOperator::Xor(XorToken {}))),

        c if c.is_ascii_alphabetic() || c == '_' => {
            let name = lex_ident(it, c);

            match name.as_str() {
                "fn" => PureToken::Keyword(Keyword::Fn(KeywordFnToken {})),
                "return" => PureToken::Keyword(Keyword::Return(KeywordReturnToken {})),

                "if" => {
                    PureToken::Keyword(Keyword::ControlFlow(ControlFlow::If(KeywordIfToken {})))
                }
                "elif" => {
                    PureToken::Keyword(Keyword::ControlFlow(ControlFlow::Elif(KeywordElifToken {})))
                }
                "else" => {
                    PureToken::Keyword(Keyword::ControlFlow(ControlFlow::Else(KeywordElseToken {})))
                }

                "while" => PureToken::Keyword(Keyword::ControlFlow(ControlFlow::While(
                    KeywordWhileToken {},
                ))),
                "for" => {
                    PureToken::Keyword(Keyword::ControlFlow(ControlFlow::For(KeywordForToken {})))
                }
                "in" => {
                    PureToken::Keyword(Keyword::ControlFlow(ControlFlow::In(KeywordInToken {})))
                }
                "where" => PureToken::Keyword(Keyword::ControlFlow(ControlFlow::Where(
                    KeywordWhereToken {},
                ))),

                "local" => {
                    PureToken::Keyword(Keyword::Locality(Locality::Local(KeywordLocalToken {})))
                }
                "global" => {
                    PureToken::Keyword(Keyword::Locality(Locality::Global(KeywordGlobalToken {})))
                }

                "const" => {
                    PureToken::Keyword(Keyword::Mutability(Mutability::Const(KeywordConstToken {})))
                }
                "mut" => {
                    PureToken::Keyword(Keyword::Mutability(Mutability::Mut(KeywordMutToken {})))
                }

                "int" => PureToken::Type(Type::Int(TypeIntToken {})),
                "float" => PureToken::Type(Type::Float(TypeFloatToken {})),
                "bool" => PureToken::Type(Type::Bool(TypeBoolToken {})),
                "str" => PureToken::Type(Type::Str(TypeStrToken {})),
                "true" => PureToken::Literal(Literal::Bool(BoolLiteralToken { value: true })),
                "false" => PureToken::Literal(Literal::Bool(BoolLiteralToken { value: false })),

                _ => PureToken::Ident(IdentToken { name }),
            }
        }
        _ => panic!("(tmp err msg) Unexpected token '{ch}'"),
    };

    let end = it.peek().map(|(index, _)| *index).unwrap_or(source_len);

    Some(Token {
        pure_token,
        span: Span { start, end },
    })
}

fn skip_whitespaces(it: &mut Lexer<'_>) {
    while matches!(it.peek(), Some((_, c)) if c.is_whitespace()) {
        it.next();
    }
}

fn lex_until_newline(it: &mut Lexer<'_>) -> String {
    let mut s = String::new();

    while let Some((_, c)) = it.peek() {
        if *c == '\n' {
            break;
        }

        s.push(*c);
        it.next();
    }

    s
}

fn lex_block_comment(it: &mut Lexer<'_>) -> String {
    let mut s = String::new();

    while let Some((_, c)) = it.next() {
        if c == '*' && matches!(it.peek(), Some((_, '/'))) {
            it.next();
            break;
        }

        s.push(c);
    }

    s
}

fn lex_ident(it: &mut Lexer<'_>, first: char) -> String {
    let mut ident = String::new();
    ident.push(first);

    while matches!(it.peek(), Some((_, c)) if c.is_ascii_alphanumeric() || *c == '_') {
        ident.push(it.next().unwrap().1);
    }

    ident
}

fn lex_string(it: &mut Lexer<'_>, start_mark: char) -> String {
    let mut value = String::new();

    for (_, ch) in it {
        if ch == start_mark {
            break;
        }

        value.push(ch);
    }

    value
}

fn lex_number(it: &mut Lexer<'_>, first: char) -> PureToken {
    let mut value = String::from(first);

    while matches!(it.peek(), Some((_, '0'..='9'))) {
        value.push(it.next().unwrap().1);
    }

    let is_float =
        matches!(it.peek(), Some((_, '.'))) && matches!(it.clone().nth(1), Some((_, '0'..='9')));

    if is_float {
        value.push(it.next().unwrap().1);

        while matches!(it.peek(), Some((_, '0'..='9'))) {
            value.push(it.next().unwrap().1);
        }

        PureToken::Literal(Literal::Float(FloatLiteralToken { value }))
    } else {
        PureToken::Literal(Literal::Int(IntLiteralToken { value }))
    }
}
