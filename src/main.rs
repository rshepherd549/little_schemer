use std::collections::HashMap;

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

#[derive(Clone)]
enum SExpression {
    Atom(String),
    List(Vec<SExpression>)
}

fn to_sexpression(tokens: &[Token]) -> Option<SExpression> {

    fn to_list(begin: std::slice::Iter<Token>) -> Option<(SExpression, std::slice::Iter<Token>)> {
        let mut list: Vec<SExpression> = Vec::new();
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
            list.push(sexp);
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
                assert_eq!(list.len(), 2);
                match &list[0] {
                    SExpression::List(list2) => {
                        assert_eq!(list2.len(), 3);
                        assert!(match &list2[0] {
                            SExpression::Atom(s) => (s == "atom"),
                            _ => false,
                        });
                        assert!(match &list2[1] {
                            SExpression::Atom(s) => (s == "turkey"),
                            _ => false,
                        });
                        assert!(match &list2[2] {
                            SExpression::Atom(s) => (s == "third"),
                            _ => false,
                        });
                    },
                    _ => assert!(false),
                }
                assert!(match &list[1] {
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

type Environment = HashMap<String, SExpression>;

impl SExpression {
    fn car(&self) -> Option<SExpression> {
        match &*self {
            SExpression::List(list) if !list.is_empty() => Some(list[0].clone()),
            _ => None,
        }
    }

    fn cdr(&self) -> Option<SExpression> {
        match &*self {
            SExpression::List(list) if !list.is_empty() => Some(SExpression::List(list[1..].to_vec())),
            _ => None,
        }
    }
    
    fn cons(&self, list: &SExpression) -> Option<SExpression> {
        match list {
            SExpression::List(list) => {
                let mut list = list.clone();
                list.insert(0, self.clone());
                Some(SExpression::List(list))
            },
            _ => None,
        }
    }
    
    fn is_null(&self) -> SExpression {
        SExpression::Atom(match &*self {
            SExpression::List(list) => list.is_empty().to_string(),
            _ => String::from("false"),
        })
    }
    
    //`quote` returns the following parameter without evaluation
    fn quote(&self) -> Option<SExpression> {
        Some(self.clone())
    }

    fn is_atom_(&self) -> bool {
        match &*self {
            SExpression::Atom(_) => true,
            _ => false,
        }
    }
    
    fn is_atom(&self) -> SExpression {
        SExpression::Atom(self.is_atom_().to_string())
    }
    
    fn is_eq(&self, other: &SExpression) -> SExpression {
        fn is_eq_(lhs: &SExpression, rhs: &SExpression) -> bool {
            match lhs {
                SExpression::Atom(lhs) => match rhs {
                    SExpression::Atom(rhs) => (lhs == rhs),
                    _ => false,
                },
                SExpression::List(lhs) => match rhs {
                    SExpression::List(rhs) =>
                         (lhs.len() == rhs.len()) &&
                         lhs.iter().zip(rhs).any(|(lhs,rhs)|is_eq_(&lhs,&rhs)),
                    _ => false,
                },
            }
        }

        SExpression::Atom(is_eq_(self, other).to_string())
    }

    fn is_lat(&self) -> SExpression {
        SExpression::Atom((match &*self {
            SExpression::Atom(_) => false,
            SExpression::List(list) => list.iter().all(|s|s.is_atom_()),
        }).to_string())
    }

    fn is_true(&self) -> bool {
        match &*self {
            SExpression::Atom(s) => (s == "true"),
            _ => false,
        }
    }
    
    fn cond(&self, conditions: &mut std::slice::Iter<SExpression>, env: &mut Environment) -> Option<SExpression> {
        let applicable_condition = conditions.find(|&condition| {
            match condition {
                SExpression::List(condition) if condition.len() > 1 => match condition[0].eval(env) {
                    Some(condition) => condition.is_true(),
                    _ => false,
                },
                _ => false,
            }
        });
        match applicable_condition {
            Some(SExpression::List(condition)) => condition[1].eval(env),
            _ => None,
        }
    }

    fn define(&self, other: &SExpression, env: &mut Environment) {
        match *&self {
            SExpression::Atom(s) => {env.insert(s.to_string(), other.clone()); ()},
            _ => (),
        }
    }

    fn eval(&self, env: &mut Environment) -> Option<SExpression> {
        fn eval_list(list: &Vec<SExpression>, env: &mut Environment) -> Option<SExpression> {
            let mut new_list : Vec<SExpression> = Vec::new();
            let mut current = list.iter();
            while let Some(sexp) = current.next() {
              match sexp {
                  SExpression::Atom(a) if a == "car" => return current.next()?.eval(env)?.car(),
                  SExpression::Atom(a) if a == "cdr" => return current.next()?.eval(env)?.cdr(),
                  SExpression::Atom(a) if a == "cons" => return current.next()?.eval(env)?.cons(&current.next()?.eval(env)?),
                  SExpression::Atom(a) if a == "null?" => return Some(current.next()?.eval(env)?.is_null()),
                  SExpression::Atom(a) if a == "quote" || a == "'" => return current.next()?.quote(),
                  SExpression::Atom(a) if a == "atom?" => return Some(current.next()?.eval(env)?.is_atom()),
                  SExpression::Atom(a) if a == "eq?" => return Some(current.next()?.eval(env)?.is_eq(&current.next()?.eval(env)?)),
                  SExpression::Atom(a) if a == "lat?" => return Some(current.next()?.eval(env)?.is_lat()),
                  SExpression::Atom(a) if a == "cond" => return sexp.cond(&mut current, env),
                  SExpression::Atom(a) if a == "define" => current.next()?.eval(env)?.define(&current.next()?.eval(env)?, env),
                  _ => match sexp.eval(env) {
                      Some(sexp) => new_list.push(sexp),
                      _ => (),
                  },
              }
            }
            Some(SExpression::List(new_list))
        }
        match &*self {
            SExpression::List(list) => eval_list(&list, env),
            SExpression::Atom(s) => match env.get(s) {
                Some(sexp) => sexp.clone().eval(env),
                _ => Some(self.clone()),
            },
        }
    }
}

#[test]
fn test_car() {
    {
        let tokens = to_tokens("hotdog");
        let sexp = to_sexpression(&tokens);
        match sexp {
            Some(sexp) =>
                assert!(match sexp.car() {
                    None => true,
                    _ => false,
                }),
            _ => assert!(false),
        }
    }
    {
        let tokens = to_tokens("()");
        let sexp = to_sexpression(&tokens);
        match sexp {
            Some(sexp) =>
                assert!(match sexp.car() {
                    None => true,
                    _ => false,
                }),
            _ => assert!(false),
        }
    }
    {
        let tokens = to_tokens("(a b c)");
        let sexp = to_sexpression(&tokens);
        match sexp {
            Some(sexp) =>
                match sexp.car() {
                    Some(SExpression::Atom(s)) => assert_eq!(s,"a"),
                    _ => assert!(false),
                },
            _ => assert!(false),
        }
    }
    {
        let tokens = to_tokens("((a b c) x y z)");
        let sexp = to_sexpression(&tokens);
        match sexp {
            Some(sexp) =>
                match sexp.car() {
                    Some(SExpression::List(list)) => {
                        assert_eq!(list.len(), 3);
                        match &list[2] {
                            SExpression::Atom(s) => assert_eq!(s, "c"),
                            _ => assert!(false),
                        }
                    },
                    _ => assert!(false),
                },
            _ => assert!(false),
        }
    }
}

#[test]
fn test_eval_car() {
    let mut env = Environment::new();
    {
        let tokens = to_tokens("(car (a b c))");
        let sexp = to_sexpression(&tokens);
        match sexp {
            Some(sexp) => match sexp.eval(&mut env) {
                Some(SExpression::Atom(s)) => assert_eq!(s, "a"),
                _ => assert!(false),
            },
            _ => assert!(false),
        }
    }
    {
        let tokens = to_tokens("(car a)");
        let sexp = to_sexpression(&tokens);
        match sexp {
            Some(sexp) => assert!(match sexp.eval(&mut env) {
                None => true,
                _ => false,
            }),
            _ => assert!(false),
        }
    }
}

fn sexpression_to_string(sexp: &SExpression, env: &mut Environment) -> String {
    let mut s = String::new();
    match sexp {
        SExpression::Atom(_) => match sexp.eval(env) {
            Some(SExpression::Atom(s_)) => s += &s_,
            Some(sexp) => s += &sexpression_to_string(&sexp, env),
            _ => (),
        },
        SExpression::List(list) => {
            s += "(";
            let mut current = list.iter();
            if let Some(sexp) = current.next() {
                if let Some(sexp) = sexp.eval(env) {
                    s += &sexpression_to_string(&sexp, env);
                    while let Some(sexp) = current.next() {
                        if let Some(sexp) = sexp.eval(env) {
                            s += " ";
                            s += &sexpression_to_string(&sexp, env);
                        }
                    }
                }
            }
            s += ")";
        }
    }
    s
}

fn eval_scheme_to_string(s: &str) -> String {
    let tokens = to_tokens(s);
    let mut env = Environment::new();
    match to_sexpression(&tokens) {
        Some(sexp) => match sexp.eval(&mut env) {
            Some(sexp) => sexpression_to_string(&sexp, &mut env),
            _ => String::from("Bad eval!"),
        },
        _ if s == "" => String::new(),
        _ => String::from("Bad scheme!"),
    }
}

#[test_case("", ""; "eval: empty")]
#[test_case("a", "a"; "eval: atom")]
#[test_case("(", "Bad scheme!"; "eval: bad input")]
#[test_case("()", "()"; "eval: empty list")]
#[test_case(" ( ( a  b )   c ) ", "((a b) c)"; "eval: list with whitespace")]
#[test_case("(car (hotdogs))", "hotdogs"; "eval: car")]
#[test_case("(car ((hotdogs)))", "(hotdogs)"; "eval: car hotdogs nested")]
#[test_case("(car (((hotdogs))))", "((hotdogs))"; "eval: car hotdogs more nested")]
#[test_case("(car ( ((hotdogs)) (and) (pickle) relish ) )", "((hotdogs))"; "eval: car nested list")]
#[test_case("(car (car ( ((hotdogs)) (and) (pickle) relish ) ) )", "(hotdogs)"; "eval: nested car")]
#[test_case("(car a)", "Bad eval!"; "eval: car of atom")]
#[test_case("(cdr (a b c) )", "(b c)"; "eval: cdr")]
#[test_case("(cdr ((a b c) x y z) )", "(x y z)"; "eval: cdr nested list")]
#[test_case("(cdr (hamburger) )", "()"; "eval: cdr 1-list")]
#[test_case("(cdr a)", "Bad eval!"; "eval: cdr of atom")]
#[test_case("(cdr ())", "Bad eval!"; "eval: cdr of empty list")]
#[test_case("(car (cdr ((b) (x y) ((c))) ))", "(x y)"; "eval: car cdr")]
#[test_case("(cdr (cdr ((b) (x y) ((c))) ))", "(((c)))"; "eval: cdr cdr")]
#[test_case("(cdr (car ((b) (x y) ((c))) ))", "()"; "eval: cdr car")]
#[test_case("(cons peanut ())", "(peanut)"; "eval: cons into empty list")]
#[test_case("(cons () ())", "(())"; "eval: cons empty list into empty list")]
#[test_case("(cons peanut (butter and jelly))", "(peanut butter and jelly)"; "eval: cons")]
#[test_case("(null? spaghetti)", "false"; "eval: null? atom")]
#[test_case("(null? ())", "true"; "eval: null? empty list")]
#[test_case("(null? (()))", "false"; "eval: null? non-empty list")]
#[test_case("(null? (car (())))", "true"; "eval: null? car non-empty list")]
#[test_case("(quote ())", "()"; "eval: quote")]
#[test_case("('())", "()"; "eval: quote apostrophe")]
#[test_case("(null? (a b c))", "false"; "eval: null? list")]
#[test_case("(atom? Harry)", "true"; "eval: atom? atom")]
#[test_case("(atom? (Harry had a heap of apples))", "false"; "eval: atom? list")]
#[test_case("(atom? ())", "false"; "eval: atom? empty list")]
#[test_case("(atom? (car (Harry had a heap of apples)))", "true"; "eval: atom? car list")]
#[test_case("(atom? (cdr (Harry had a heap of apples)))", "false"; "eval: atom? cdr list")]
#[test_case("(atom? (cdr (Harry)))", "false"; "eval: atom? cdr 1-list")]
#[test_case("(atom? (car (cdr (swing low sweet cherry oat))))", "true"; "eval: atom? car cdr list")]
#[test_case("(atom? (car (cdr (swing (low sweet) cherry oat))))", "false"; "eval: atom? car cdr list of list")]
#[test_case("(eq? Harry Harry)", "true"; "eval: eq? same atoms")]
#[test_case("(eq? margarine butter)", "false"; "eval: eq? different atoms")]
#[test_case("(eq? () (strawberry))", "false"; "eval: eq? different lists")]
#[test_case("(eq? (strawberry tea) (strawberry tea))", "true"; "eval: eq? same lists")]
#[test_case("(eq? 6 7)", "false"; "eval: eq? different numbers")]
#[test_case("(eq? 7 7)", "true"; "eval: eq? same numbers")]
#[test_case("(eq? (car (Mary had a little lamb)) Mary)", "true"; "eval: eq? car")]
#[test_case("(eq? (cdr (soured milk)) milk)", "false"; "eval: eq? cdr list and atom")]
#[test_case("(eq? (cdr (soured milk)) (milk))", "true"; "eval: eq? cdr list and list")]
#[test_case("(eq? (car (beans beans we need jelly beans)) (car (cdr (beans beans we need jelly beans))) )", "true"; "eval: eq? 1st 2nd")]
#[test_case("(lat? (Jack Sprat could eat no chicken fat) )", "true"; "eval: lat? list of atoms")]
#[test_case("(lat? ((Jack) Sprat could eat no chicken fat) )", "false"; "eval: lat? list including list")]
#[test_case("(lat? (Jack (Sprat could) eat no chicken fat) )", "false"; "eval: lat? another list including list")]
#[test_case("(lat? () )", "true"; "eval: lat? empty list")]
#[test_case("(cond (true a) )", "a"; "eval: cond true")]
#[test_case("(cond (false a) (true b) )", "b"; "eval: cond false true")]
#[test_case("(cond (true a) (true b) )", "a"; "eval: cond first result")]
#[test_case("(cond (false a) )", "Bad eval!"; "eval: cond no result")]
#[test_case("(cond ((eq? a a) equal) (true not-equal) )", "equal"; "eval: cond apply eq? to same")]
#[test_case("(cond ((eq? a b) equal) (true not-equal) )", "not-equal"; "eval: cond apply eq? to different")]
#[test_case("((define else true) (cond ((eq? a b) equal) (else not-equal)) )", "(() not-equal)"; "eval: cond use else")]
#[test_case("(define a b)", "()"; "eval: define isolated")]
#[test_case("( (eq? a b) (eq? c c))", "(false true)"; "eval: multiple expressions")]
#[test_case("( (define a b) a)", "(() b)"; "eval: define substitute definition")]
fn test_eval_scheme_to_string(s: &str, expected: &str) {
    assert_eq!(eval_scheme_to_string(&s), expected);
}

fn main() {
    println!("little_schemer");
}
