#[derive(Debug)]
#[derive(PartialEq)]
enum Token {
    OpenBracket,
    CloseBracket,
    Atom(String),
}

fn to_tokens(text: &str) -> Vec<Token> {

    let mut tokens = Vec::<Token>::new();
    let mut atom = String::new();

    for c in text.chars() {
        if !atom.is_empty()
          && ( c == '(' 
            || c == ')'
            || !c.is_ascii_graphic()) {
            tokens.push(Token::Atom(atom.clone()));
            atom.clear();
        }

        if c == '(' {
            tokens.push(Token::OpenBracket);
        }
        else if c == ')' {
            tokens.push(Token::CloseBracket);
        }
        else if c.is_ascii_graphic() {
            atom.push(c);
        }
    }

    if !atom.is_empty() {
        tokens.push(Token::Atom(atom.clone()));
    }

    tokens
}

#[test]
fn test_to_tokens() {
    {
        let tokens = to_tokens("");
        assert_eq!(tokens.len(), 0);
    }

    {
        let tokens = to_tokens("a");
        assert_eq!(tokens.len(), 1);

        assert!(match &tokens[0] {
            Token::Atom(text) => text == "a",
            _ => false
          });

        assert_eq!(tokens, vec!(Token::Atom("a".to_string())));
    }
}

fn is_atom(tokens: &Vec<Token>) -> bool {
    tokens.len() == 1 &&
    match &tokens[0] {
        Token::Atom(_) => true,
        _ => false
    }
}

use test_case::test_case;

#[test_case("atom", true; "is_atom: simple word")]
#[test_case("turkey", true; "is_atom: simple word 2")]
#[test_case("1492", true; "is_atom: number")]
#[test_case("u", true; "is_atom: single letter")]
#[test_case("*abc$", true; "is_atom: include $")]
#[test_case("", false; "is_atom: empty string")]
#[test_case(" ", false; "is_atom: whitespace")]
#[test_case(" a", true; "is_atom: whitespace before")]
#[test_case("a ", true; "is_atom: whitespace after")]
#[test_case(" a ", true; "is_atom: whitespace before and after")]
#[test_case("(", false; "is_atom: left bracket")]
#[test_case("(abc$", false; "is_atom: left bracket and atom")]
#[test_case("(abc$)", false; "is_atom: bracketed atom")]
fn test_is_atom(s: &str, expected: bool) {
    let tokens = to_tokens(s);
    assert_eq!(is_atom(&tokens), expected);
}

fn is_list(tokens: &Vec<Token>) -> bool {

    let mut depth = 0;
    let mut max_depth = 0;
    
    for token in tokens {
        match token {
            Token::OpenBracket => {
                depth += 1;
                if depth > max_depth {
                    max_depth = depth;
                }
            },
            Token::CloseBracket => {
                depth -= 1;
                if depth < 0 {
                    return false;
                }
            },
            Token::Atom(_) => {
                //Check that atom isn't found outside outermost list
                if depth <= 0 {
                    return false;
                }
            }
        }
    }

    depth == 0 && max_depth > 0
}

#[test_case("atom", false; "is_list: atom")]
#[test_case("(atom)", true; "is_list: one atom list")]
#[test_case("()", true; "is_list: empty list")]
#[test_case("(atom", false; "is_list: unclosed list")]
#[test_case("(atom turkey or)", true; "is_list: list of 3 atoms")]
#[test_case("(atom (turkey (pitch black))or ())", true; "is_list: nested list")]
#[test_case("  (  atom    turkey  or )  ", true; "is_list: spaced out list")]
#[test_case("(atom turkey) or", false; "is_list: list and atom")]
#[test_case("((atom turkey) or)", true; "is_list: list of list and atom")]
fn test_is_list(s: &str, expected: bool) {
    assert_eq!(is_list(&to_tokens(s)), expected);
}

/// s_expression
fn is_s_exp(tokens: &Vec<Token>) -> bool {
    is_atom(tokens) || is_list(tokens)
}

#[test]
fn test_is_s_exp()
{
    assert_eq!(is_s_exp(&to_tokens("")), false);
    assert_eq!(is_s_exp(&to_tokens(" ")), false);
    assert_eq!(is_s_exp(&to_tokens("xyz")), true);
    assert_eq!(is_s_exp(&to_tokens("(x y z)")), true);
    assert_eq!(is_s_exp(&to_tokens("(x y) z")), false);
    assert_eq!(is_s_exp(&to_tokens("atom atom")), false);
}

fn main() {
    println!("little_schemer");
}
