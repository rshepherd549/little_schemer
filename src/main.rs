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

enum SExpression {
    Atom(String),
    List(Vec<Box<SExpression>>)
}

fn to_sexpression(tokens: &[Token]) -> Option<SExpression> {

    fn to_list(begin: std::slice::Iter<Token>) -> Option<(SExpression, std::slice::Iter<Token>)> {
        let mut list: Vec<Box<SExpression>> = Vec::new();
        let mut current = begin;
        loop {
            let (sexp, next) = match current.next() {
              Some(Token::Atom(s)) => (SExpression::Atom(s.to_string()), current),
              Some(Token::OpenBracket) => match to_list(current) {
                  Some(list) => list,
                  _ => break, //Bad inner list
              },
              Some(Token::CloseBracket) => return Some((SExpression::List(list), current)),
              None => break, //Ran out of tokens before finding matching CloseBracket
            };
            list.push(Box::new(sexp));
            current = next;
        }
        return None;
      }
      
    if tokens.is_empty() {
        return None
    }

    let mut current = tokens.iter();
    let (sexp, mut next) = match current.next() {
        Some(Token::OpenBracket) => match to_list(current) {
            Some(sexp_next) => sexp_next,
            _ => return None,
        },
        Some(Token::CloseBracket) => return None,
        Some(Token::Atom(s)) => (SExpression::Atom(s.to_string()), current),
        None => return None,
    };
    match next.next() {
        Some(_) => return None, //More than one sexpression when either list or atom expected
        _ => Some(sexp),
    }
}

#[test]
fn test_to_sexpression() {
    {
        let tokens = to_tokens("");
        let sexp = to_sexpression(&tokens);
        assert!(match sexp {
            None => true,
            _ => false
          });
    }
    {
        let tokens = to_tokens("()");
        let sexp = to_sexpression(&tokens);
        assert!(match sexp {
            Some(SExpression::List(list)) => (list.len() == 0),
            _ => false
          });
    }
    {
        let tokens = to_tokens("a");
        let sexp = to_sexpression(&tokens);
        assert!(match sexp {
            Some(SExpression::Atom(s)) => s == "a",
            _ => false
          });
    }
    {
        let tokens = to_tokens("(atom turkey) or");
        let sexp = to_sexpression(&tokens);
        assert!(match sexp {
            None => true,
            _ => false
          });
    }
    {
        let tokens = to_tokens("((atom turkey third) or)");
        let sexp = to_sexpression(&tokens);

        match sexp {
            Some(SExpression::List(list)) => {
                assert!(list.len() == 2);
                match &*list[0] {
                    SExpression::List(list2) => {
                        assert!(list2.len() == 3);
                        assert!(match &*list2[0] {
                            SExpression::Atom(s) => (s == "atom"),
                            _ => false,
                        });
                        assert!(match &*list2[1] {
                            SExpression::Atom(s) => (s == "turkey"),
                            _ => false,
                        });
                        assert!(match &*list2[2] {
                            SExpression::Atom(s) => (s == "third"),
                            _ => false,
                        });
                    },
                    _ => assert!(false),
                }
                assert!(match &*list[1] {
                    SExpression::Atom(s) => (s == "or"),
                    _ => false,
                });
            },
            _ => assert!(false),
          }
    }
    {
        let tokens = to_tokens("(how are you doing so far)");
        let sexp = to_sexpression(&tokens);

        assert!(match sexp {
            Some(SExpression::List(list)) => (list.len() == 6),
            _ => false,
        });
    }
    {
        let tokens = to_tokens("(((how) are)((you)(doing so))far)");
        let sexp = to_sexpression(&tokens);

        assert!(match sexp {
            Some(SExpression::List(_)) => true,
            _ => false,
        });

        assert!(if let Some(SExpression::List(_)) = sexp {
                true
            } else {
                false
        });

        assert!(match sexp {
            Some(SExpression::List(list)) => (list.len() == 3),
            _ => false,
        });
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
