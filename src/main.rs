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

fn is_atom(text: &str) -> bool {
    let tokens = to_tokens(text);

    tokens.len() == 1 &&
    match &tokens[0] {
        Token::Atom(_) => true,
        _ => false
    }
}

#[test]
fn test_is_atom() {
    assert_eq!(is_atom("atom"), true);
    assert_eq!(is_atom("turkey"), true);
    assert_eq!(is_atom("1492"), true);
    assert_eq!(is_atom("u"), true);
    assert_eq!(is_atom("*abc$"), true);
    assert_eq!(is_atom(""), false);
    assert_eq!(is_atom(" "), false);
    assert_eq!(is_atom("a"), true);
    assert_eq!(is_atom(" a"), true); //allow whitespace
    assert_eq!(is_atom("a "), true); //allow whitespace
    assert_eq!(is_atom(" a "), true); //allow whitespace
    assert_eq!(is_atom("("), false);
    assert_eq!(is_atom("(abc$"), false);
    assert_eq!(is_atom("(abc$)"), false);
}

fn is_list(text: &str) -> bool {

    let tokens = to_tokens(text);

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

#[test]
fn test_is_list() {
    assert_eq!(is_list("atom"), false);
    assert_eq!(is_list("(atom)"), true);
    assert_eq!(is_list("()"), true);
    assert_eq!(is_list("(atom"), false);
    assert_eq!(is_list("(atom turkey or)"), true);
    assert_eq!(is_list("(atom (turkey (pitch black))or ())"), true);
    assert_eq!(is_list("  (  atom    turkey  or )  "), true);
    assert_eq!(is_list("(atom turkey) or"), false);
    assert_eq!(is_list("((atom turkey) or)"), true);
}

/// sexpression
fn is_sexp(text: &str) -> bool {
    is_atom(text) || is_list(text)
}

#[test]
fn test_is_sexp()
{
    assert_eq!(is_sexp(""), false);
    assert_eq!(is_sexp(" "), false);
    assert_eq!(is_sexp("xyz"), true);
    assert_eq!(is_sexp("(x y z)"), true);
    assert_eq!(is_sexp("(x y) z"), false);
    assert_eq!(is_sexp("atom atom"), false);
}

fn main() {
    println!("little_schemer");
}
