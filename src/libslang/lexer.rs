use logos::Logos;

/// The tokens that slang can have. The lexer is auto-generated by
/// `logos`. High priorities are assigned to keywords and symbols 
/// so they are matched before identifiers and numbers.
#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    // Single character symbols
    #[token(":", priority = 500)]
    Colon,

    #[token(",", priority = 501)]
    Comma,

    #[token(".", priority = 502)]
    Dot,

    #[token("{", priority = 503)]
    LBrace,

    #[token("}", priority = 504)]
    RBrace,

    #[token("(", priority = 505)]
    LParen,

    #[token(")", priority = 506)]
    RParen,

    #[token("[", priority = 507)]
    LBracket,

    #[token("]", priority = 508)]
    RBracket,

    #[token(";", priority = 509)]
    Semicolon,


    // Comparison and assignment operators
    #[token("!", priority = 510)]
    Bang,

    #[token("!=", priority = 511)]
    BangEq,

    #[token("=", priority = 512)]
    Eq,

    #[token("==", priority = 513)]
    EqEq,

    #[token(">", priority = 514)]
    Gt,

    #[token(">=", priority = 515)]
    Gte,

    #[token("<", priority = 516)]
    Lt,

    #[token("<=", priority = 517)]
    Lte,


    // Arithmetic
    #[token("+", priority = 518)]
    Plus,

    #[token("++", priority = 519)]
    PlusPlus,

    #[token("+=", priority = 520)]
    PlusEq,

    #[token("-", priority = 521)]
    Minus,

    #[token("--", priority = 522)]
    MinusMinus,

    #[token("-=", priority = 523)]
    MinusEq,

    #[token("/", priority = 526)]
    Slash,

    #[token("/=", priority = 527)]
    SlashEq,

    #[token("*", priority = 524)]
    Star,

    #[token("*=", priority = 525)]
    StarEq,


    // Keywords
    #[token("and", priority = 1000)]
    And,

    #[token("break", priority = 1001)]
    Break,

    #[token("clone", priority = 1002)]
    Clone,

    #[token("const", priority = 1003)]
    Const,

    #[token("continue", priority = 1004)]
    Continue,

    #[token("else", priority = 1005)]
    Else,

    #[token("none", priority = 1006)]
    None,

    #[token("false", priority = 1007)]
    False,

    #[token("fn", priority = 1008)]
    Fn,

    #[token("for", priority = 1009)]
    For,

    #[token("if", priority = 1010)]
    If,

    #[token("let", priority= 1011)]
    Let,

    #[token("or", priority = 1012)]
    Or,

    #[token("print", priority = 1013)]
    Print,

    #[token("ret", priority = 1014)]
    Ret,

    #[token("self", priority = 1015)]
    Slf,

    #[token("true", priority = 1016)]
    True,

    #[token("while", priority = 1017)]
    While,

    // Integers and identifiers
    #[regex("_?[a-zA-Z]+[a-zA-Z0-9]*", priority = 6)]
    Identifier,

    #[regex("[0-9]+\\.[0-9]+", priority = 3)]
    Float,
    
    #[regex("[0-9]+", priority = 4)]
    Integer,

    // Matches a double quote string or a single quote string
    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*"|'([^'\\]|\\t|\\u|\\n|\\")*'"#, priority = 5)]
    String,

    // Skip comments and whitespace
    #[regex(r"\\/\\/\\.*|[ \\t\\n\\f]+", logos::skip)]
    #[error]
    Error,
}

mod tests {
    use super::*;

    // Generates tests for checking individual tokens
    macro_rules! gen_test {
        ($name: ident, $input: expr, $expected_res: expr) => {
            #[test]
            fn $name() {
                let mut lex = Token::lexer($input);
                
                assert_eq!(lex.next(), Some($expected_res));
                assert_eq!(lex.slice(), $input);
            }
        }
    }

    // TODO: Add tests with multiple strings

    gen_test!(test_dot, ".", Token::Dot);
    gen_test!(test_comma, ",", Token::Comma);
    gen_test!(test_lbrace, "{", Token::LBrace);
    gen_test!(test_rbrace, "}", Token::RBrace);
    gen_test!(test_lparen, "(", Token::LParen);
    gen_test!(test_rparen, ")", Token::RParen);
    gen_test!(test_lbracket, "[", Token::LBracket);
    gen_test!(test_rbracket, "]", Token::RBracket);
    gen_test!(test_semicolon, ";", Token::Semicolon);

    gen_test!(test_bang, "!", Token::Bang);
    gen_test!(test_bangeq, "!=", Token::BangEq);
    gen_test!(test_eq, "=", Token::Eq);
    gen_test!(test_eqeq, "==", Token::EqEq);
    gen_test!(test_gt, ">", Token::Gt);
    gen_test!(test_gte, ">=", Token::Gte);
    gen_test!(test_lt, "<", Token::Lt);
    gen_test!(test_lte, "<=", Token::Lte);

    gen_test!(test_plus, "+", Token::Plus);
    gen_test!(test_plusplus, "++", Token::PlusPlus);
    gen_test!(test_pluseq, "+=", Token::PlusEq);
    gen_test!(test_minus, "-", Token::Minus);
    gen_test!(test_minusminus, "--", Token::MinusMinus);
    gen_test!(test_slash, "/", Token::Slash);
    gen_test!(test_slasheq, "/=", Token::SlashEq);
    gen_test!(test_star, "*", Token::Star);
    gen_test!(test_stareq, "*=", Token::StarEq);

    gen_test!(test_and, "and", Token::And);
    gen_test!(test_break, "break", Token::Break);
    gen_test!(test_clone, "clone", Token::Clone);
    gen_test!(test_const, "const", Token::Const);
    gen_test!(test_continue, "continue", Token::Continue);
    gen_test!(test_else, "else", Token::Else);
    gen_test!(test_none, "none", Token::None);
    gen_test!(test_false, "false", Token::False);
    gen_test!(test_fn, "fn", Token::Fn);
    gen_test!(test_for, "for", Token::For);
    gen_test!(test_if, "if", Token::If);
    gen_test!(test_let, "let", Token::Let);
    gen_test!(test_or, "or", Token::Or);
    gen_test!(test_print, "print", Token::Print);
    gen_test!(test_ret, "ret", Token::Ret);
    gen_test!(test_slf, "self", Token::Slf);
    gen_test!(test_true, "true", Token::True);
    gen_test!(test_while, "while", Token::While);
}