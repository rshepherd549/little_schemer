fn is_character(c: &char) -> bool {
    c.is_ascii_graphic()
 && *c != '('
}


fn is_atom(text: &str) -> bool {
    if text.is_empty() {
      return false;
    }

    let chars = text.chars();

    for c in chars {
        if !is_character(&c) {
            return false;
        }
    }

    true
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
    assert_eq!(is_atom(" a"), false);
    assert_eq!(is_atom("a "), false);
    assert_eq!(is_atom(" a "), false);
    assert_eq!(is_atom("("), false);
    assert_eq!(is_atom("(abc$"), false);
    assert_eq!(is_atom("(abc$)"), false);
}

fn is_list(text: &str) -> bool {

    let mut found_open_bracket = false;
    let mut found_close_bracket = false;
    
    for c in text.chars() {
        if !found_open_bracket {
            if c == '(' {
                found_open_bracket = true;
            }
            else if c.is_ascii_graphic() {
                return false; //atom before open_bracket
            }
        }
        else if !found_close_bracket {
            if c == ')' {
                found_close_bracket = true;
            }
        }
        else {
            if c.is_ascii_graphic() {
                return false; //atom after close_bracket
            }
        }
    }

    found_close_bracket
}

#[test]
fn test_is_list() {
    assert_eq!(is_list("atom"), false);
    assert_eq!(is_list("(atom)"), true);
    assert_eq!(is_list("()"), true);
    assert_eq!(is_list("(atom"), false);
    assert_eq!(is_list("(atom turkey or)"), true);
    assert_eq!(is_list("  (  atom    turkey  or )  "), true);
    assert_eq!(is_list("(atom turkey) or"), false);
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
    assert_eq!(is_sexp("atom"), true);
    assert_eq!(is_sexp("(atom)"), true);
    assert_eq!(is_sexp("(atom) atom"), false);
    assert_eq!(is_sexp("atom atom"), false);
}

fn main() {
    println!("little_schemer");
}
