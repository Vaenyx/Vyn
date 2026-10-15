use crate::tokenizing::{
    Token,
    token::{BlockCommentToken, IdentToken, LineCommentToken, PureToken, Span},
};

pub fn tokenize(vyn_code: &str) -> Vec<Token> {
    let mut it = vyn_code.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(tok) = next_token(&mut it) {
        tokens.push(tok);
    }

    tokens
}

fn next_token<I>(it: &mut std::iter::Peekable<I>) -> Option<Token>
where
    I: Iterator<Item = char>,
{
    skip_whitespaces(it);

    let ch = it.next()?;

    match ch {
        '/' if matches!(it.peek(), Some('/')) => {
            it.next();
            Some(Token {
                pure_token: PureToken::LineComment(LineCommentToken {
                    text: lex_until_newline(it),
                }),
                span: Span { start: 1, end: 2 },
            })
        }
        '/' if matches!(it.peek(), Some('*')) => {
            it.next();
            Some(Token {
                pure_token: PureToken::BlockComment(BlockCommentToken {
                    text: lex_block_comment(it),
                }),
                span: Span { start: 1, end: 2 },
            })
        }
        c if c.is_ascii_alphabetic() || c == '_' => {
            let name = lex_ident(it, c);

            let pure_token = match name.as_str() {
                "int" => PureToken::TypeInt(TypeIntToken {}),
                // wenn du später mehr willst:
                // "float" => PureToken::TypeFloat(TypeFloatToken {}),
                // "bool" => PureToken::TypeBool(TypeBoolToken {}),
                // "str" => PureToken::TypeStr(TypeStrToken {}),
                _ => PureToken::Ident(IdentToken { name }),
            };

            Some(Token {
                pure_token,
                span: Span { start: 1, end: 2 },
            })
        }
        _ => None,
    }
}

fn skip_whitespaces<I>(it: &mut std::iter::Peekable<I>)
where
    I: Iterator<Item = char>,
{
    while matches!(it.peek(), Some(c) if c.is_whitespace()) {
        it.next();
    }
}

fn lex_until_newline<I>(it: &mut I) -> String
where
    I: Iterator<Item = char>,
{
    let mut s = String::new();
    for c in it {
        if c == '\n' {
            break;
        }
        s.push(c);
    }
    s
}

fn lex_block_comment<I>(it: &mut std::iter::Peekable<I>) -> String
where
    I: Iterator<Item = char>,
{
    let mut s = String::new();
    while let Some(c) = it.next() {
        if c == '*' && matches!(it.peek(), Some('/')) {
            it.next(); // consume '/'
            break;
        }
        s.push(c);
    }
    s
}

fn lex_ident<I>(it: &mut std::iter::Peekable<I>, first: char) -> String
where
    I: Iterator<Item = char>,
{
    let mut ident = String::new();
    ident.push(first);
    while matches!(it.peek(), Some(c) if c.is_ascii_alphanumeric() || *c == '_') {
        ident.push(it.next().unwrap());
    }
    ident
}
