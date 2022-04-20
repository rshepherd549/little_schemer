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

#[test]
fn test_is_atom() {
    assert_eq!(is_atom(&to_tokens("atom")), true);
    assert_eq!(is_atom(&to_tokens("turkey")), true);
    assert_eq!(is_atom(&to_tokens("1492")), true);
    assert_eq!(is_atom(&to_tokens("u")), true);
    assert_eq!(is_atom(&to_tokens("*abc$")), true);
    assert_eq!(is_atom(&to_tokens("")), false);
    assert_eq!(is_atom(&to_tokens(" ")), false);
    assert_eq!(is_atom(&to_tokens("a")), true);
    assert_eq!(is_atom(&to_tokens(" a")), true); //allow whitespace
    assert_eq!(is_atom(&to_tokens("a ")), true); //allow whitespace
    assert_eq!(is_atom(&to_tokens(" a ")), true); //allow whitespace
    assert_eq!(is_atom(&to_tokens("(")), false);
    assert_eq!(is_atom(&to_tokens("(abc$")), false);
    assert_eq!(is_atom(&to_tokens("(abc$)")), false);
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

#[test]
fn test_is_list() {
    assert_eq!(is_list(&to_tokens("atom")), false);
    assert_eq!(is_list(&to_tokens("(atom)")), true);
    assert_eq!(is_list(&to_tokens("()")), true);
    assert_eq!(is_list(&to_tokens("(atom")), false);
    assert_eq!(is_list(&to_tokens("(atom turkey or)")), true);
    assert_eq!(is_list(&to_tokens("(atom (turkey (pitch black))or ())")), true);
    assert_eq!(is_list(&to_tokens("  (  atom    turkey  or )  ")), true);
    assert_eq!(is_list(&to_tokens("(atom turkey) or")), false);
    assert_eq!(is_list(&to_tokens("((atom turkey) or)")), true);
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
