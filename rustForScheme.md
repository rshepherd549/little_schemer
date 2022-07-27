---
theme: default
paginate: true
marp: true
---


# Using Rust to understand the Little Schemer
## Richard Shepherd

---

# My industrial experience

- C++
- Java
- C
- Fortran
- Modula 2
- Actor

---

No formal computer science

Plenty of reading, discussing and exploring

- JSP
- OOP
- XP & TDD
- Lambda calculus
- Rust

---

<!-- Increasingly more other approaches to programming and design -->

# Structure and Interpretation of Computer Programs
## Abelson, Sussman and Sussman

![](./images/sicpCoverSmall.jpg)

---

# Covers a lot, using Lisp

- arithmetic
- recursion
- functions
- functions as arguments
- data abstraction
- benefits and costs of assignnment
- concurrency
- data as programs
- logic programming
- assemblers and compilers

---

# Build an Lisp interpreter

At the same time I heard about

# The Little Lisper
# The Little Schemer
## Friedman and Felleisen

![](./images/littleSchemerCoverSmall.jpg)

<!-- The highly recommended 'The Little Schemer' book turned out to be a very Socratic tutorial, comprised entirely of questions, allowing the reader to progressively build up their understanding -->

---

# Unusual Socratic teaching method

![](./images/littleSchemer1Small.jpg)

---

# Understand something by teaching it
## Write a Lisp interpreter

- incrementally by following the questions
- using Rust, to practice and experiment
- while learning parser and interpreter techniques

<!-- To force myself to read, consider and answer the questions, I decided to incrementally write a parser that could answer the questions for me -->
<!-- Writing the parser in Rust would provide an opportunity to learn Rust from scratch. It would also give me an excuse to initially write very crude parsers in rudimentary Rust, and then rewrite more sophisticated parsers as my knowledge of Rust syntax and techniques increased -->

![](./images/rustCoverSmall.jpg) | ![](./images/parserCoverSmall.jpg) | ![](./images/commonLispCoverSmall.jpg)
--- | --- | ---

---

# Using Rust to learn to write an Interpreter to learn Scheme to appreciate the Structure and Interpretation of Computer Programs

Avoid Analysis Paralysis:
- Look things up on all topics as I went along and felt I was slowing down
- Expect to rewrite as I learnt something better

<!-- Start with command line
- Use Rust standard naming:
  - Use lower camel case names, apart from upper case for Types.
- VisualStudioCode
  - Configure building and running tests
-->

---

# Chapter 1. Page 3. Question 1

### Is it true that this is an atom?
> `atom`

*yes, because `atom` is a string of characters beginning with the letter a*

The book has the answer, often with a bit of explanation, but leaves it to the reader to build a mental model of the rationale and doesn't worry about implementation

---

### Is it true that this is an atom?
> `turkey`

*yes, because `turkey` is a string of characters beginning with a letter*

### Is it true that this is an atom?
> `1492`

*yes, because `1492` is a string of digits*

---

```rust
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
```

<!-- Difference between String and str
- Iterate over a collection
- Need to dereference references to use as value (e.g. for comparisons) but not for calling methods (but can; syntactic sugar)
- Experimented with minimizing `return`. Doesn't seem to like the last line of a general block returning a value, or returning the value of a block as an expression e.g. to simplify `is_atom` to a functional expression
-->

---

```rust
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
```

---

### Is it true that this is an list?
> `(atom)`

*yes, because `(atom)` is an atom enclosed by parentheses*

---
Simple state machine:
```rust
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
```

<!-- Crude state machine: loop over chars with mutable flags -->

---

```rust
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
```

---

### Is it true that this is an S-expression?
> `xyz`

*yes, because all atoms are S-expressions*

### Is it true that this is an S-expression?
> `(x y z)`

*yes, because all lists are S-expressions*

---

Simple:
```rust
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
```

---

This works for simple examples:
- but not for nested lists
- is a very manual, procedural approach

What would it look like if we used more of the Rust language tp help enforce concepts?

- separate text parsing from the understanding
- use types to represent the different concepts

---

```rust
enum Token {
    OpenBracket,
    CloseBracket,
    Atom(String),
}
```

---

```rust
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
```

<!-- Replace with Lexer

- Better tool for analyzing the atoms and lists.
- Still using mutable state
-->

---

```rust
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
```
Discovered the need for `#[derive(Debug)]` and `#[derive(PartialEq)]`!

<!-- interesting working out a Rust-ish way to test the type and contents of vector of variants -->

---

```rust
#[derive(Debug)]
#[derive(PartialEq)]
enum Token {
    OpenBracket,
    CloseBracket,
    Atom(String),
}
```

---

## Previous tests all pass, and...

In addition, nested list questions now work:

```rust
fn test_is_list() {
    assert_eq!(is_list("(atom (turkey (pitch black))or ())"), true);
    assert_eq!(is_list("(atom turkey) or"), false);
    assert_eq!(is_list("((atom turkey) or)"), true);
}
fn test_is_sexp()
{
    assert_eq!(is_sexp(""), false);
    assert_eq!(is_sexp(" "), false);
    assert_eq!(is_sexp("xyz"), true);
    assert_eq!(is_sexp("(x y z)"), true);
    assert_eq!(is_sexp("(x y) z"), false);
    assert_eq!(is_sexp("atom atom"), false);
}
```

<!-- until now, everything had been built in a text editor and run from the command line.
- Now put the time in to get build and test run working in VSC
- Still haven't got around to trying the debugger
  - not seemed to need to
  - if it compiles, then it works, or it's clear why
-->
---

Decoupled Tokens from Strings
```rust
fn is_atom(tokens: &Vec<Token>) -> bool {
    tokens.len() == 1 &&
    match &tokens[0] {
        Token::Atom(_) => true,
        _ => false
    }
}
fn test_is_atom() {
    assert_eq!(is_atom(&to_tokens("atom")), true);
```

Allowing me to:
- compose functions that work with tokens
- reduce error checking with functions

---

# Move to test cases

Instead of:
```rust
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
```

---

Reduce duplication:
```rust
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
```

<!-- Annoying having to add trailing different name
- Replace with custom macro that implicitly uses line number?
-->
---

# Recursive parser
```rust
fn to_sexpression(tokens: &[Token]) -> Option<SExpression> {
    fn to_list(tokens: &[Token], begin_token: usize) -> Option<(SExpression, usize)> {
        ...
              Token::OpenBracket => match to_list(tokens, curr_token+1) {
                  Some(list) => list,
                  _ => break,
              },
        ...
    }
    let curr_token = 0;
    let (sexp, next_token) = match &tokens[curr_token] {
        Token::OpenBracket => match to_list(tokens, curr_token+1) {
            Some(sexp_next_token) => sexp_next_token,
            _ => return None,
        },
        Token::CloseBracket => return None,
        Token::Atom(s) => (SExpression::Atom(s.to_string()), curr_token+1),
    };
    if next_token != tokens.len() {
        return None; //More than one sexpression when either list or atom expected
    }
    Some(sexp)
}
```

---

```rust
    fn to_list(tokens: &[Token], begin_token: usize) -> Option<(SExpression, usize)> {
        ...
        while curr_token != tokens.len() {
            let (sexp, next_token) = match &tokens[curr_token] {
              Token::Atom(s) => (SExpression::Atom(s.to_string()), curr_token+1),
              Token::OpenBracket => match to_list(tokens, curr_token+1) {
                  Some(list) => list,
                  _ => break,
              },
              Token::CloseBracket => return Some((SExpression::List(list), curr_token+1)),
            };
            list.push(Box::new(sexp));
            curr_token = next_token;
        }
        return None; //Didn't find matching CloseBracket
```

---

# Questions

Enum: can't use an individual enum value as a type even though it can have varying state e.g.
```rust
enum SExpression {
    Atom(String),
    List(Vec<Box<SExpression>>)
}

fnc(SExpression) -> SExpression::List
```
*Realized since that you can just use the type e.g. `Vec<Box<SExpression>>`,
or create a new type and then use that in the enum*

---

# Using Vec means we don't need Box
```rust
enum SExpression {
    Atom(String),
    List(Vec<Box<SExpression>>)
}
...
            list.push(Box::new(sexp));
```
```rust
enum SExpression {
    Atom(String),
    List(Vec<SExpression>)
}
'''
            list.push(sexp);

```

---
## Use iterators rather than indexes

```rust
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
```

---

# Chapter 1. Page 5...

<!-- back to the book.. -->

## What is the *car* of `l` where `l` is 
> `(a b c)`

*`a`, because `a` is the first atom of the list*

```rust
fn car(sexp: &SExpression) -> Option<&SExpression> {
    match sexp {
        SExpression::List(list) if !list.is_empty() => Some(&list[0]),
        _ => None,
    }
}
```

---
```rust
fn test_car() {
    let tokens = to_tokens("(a b c)");
    let sexp = to_sexpression(&tokens);
    match sexp {
        Some(sexp) =>
            match car(&sexp) {
                Some(SExpression::Atom(s)) => assert!(s == "a"),
                _ => assert!(false),
            },
        _ => assert!(false),
    }
```
---
# Evaluate `car` 
```rust
fn eval(sexp: &SExpression) -> Option<SExpression> {
    fn eval_list(list: &Vec<SExpression>) -> Option<Vec<SExpression>> {
        let mut new_list : Vec<SExpression> = Vec::new();
        let mut current = list.iter();
        while let Some(sexp) = current.next() {
          new_list.push(match sexp {
              SExpression::Atom(a) if a == "car" =>
                  match current.next() {
                      Some(sexp) => match car(&sexp) {
                          Some(sexp) => sexp.clone(),
                          _ => return None,
                      },
                      _ => return None,
                  },
              _ => sexp.clone(),
          })
        }
        Some(new_list)
    }
    match sexp {
        SExpression::List(list) => match eval_list(&list) {
            Some(list) => Some(SExpression::List(list)),
            _ => None,
        },
        SExpression::Atom(s) => Some(SExpression::Atom(s.to_string())),
    }
}
```

---
```rust
fn test_eval_car() {
    {
        let tokens = to_tokens("(car (a b c))");
        let sexp = to_sexpression(&tokens);
        match sexp {
            Some(sexp) => {
                let eval_sexp = eval(&sexp);
                match eval_sexp {
                    Some(SExpression::List(list)) => {
                        assert!(list.len() == 1);
                        match &list[0] {
                            SExpression::Atom(s) => assert!(s == "a"),
                            _ => assert!(false),
                        }
                    },
                    _ => assert!(false),
                }
            },
            _ => assert!(false),
        }
    }
}
```
---

# More questions

Imperative loop! Start thinking about replacing this with recursion

Consider making functions return nil or () instead of None - removing a layer of Option.
- General idea that an additional enum value could replace wrapping the enum in an option, but then uses are forced to test Nil everytime i.e. can't use types to enforce initialized.
- Empty list has the advantage that many functions that is expecting a list should already have suitable behavior for empty list
- but if everything receives and returns Option then might as well remove the extra layer? But then can't take advantage of `?`

Individual test cases are painful to write

---
# string input to string output

- we already have String -> SExpression
- add SExpression -> String
- more natural and succinct tests
```rust
#[test_case("", ""; "eval: empty")]
#[test_case("a", "a"; "eval: atom")]
#[test_case("()", "()"; "eval: empty list")]
#[test_case(" ( ( a  b )   c ) ", "((a b) c)"; "eval: list with whitespace")]
fn test_eval_scheme_to_string(s: &str, expected: &str) {
    assert_eq!(eval_scheme_to_string(&s), expected);
}
```
---
# Attack of the Clones
```rust
fn eval(sexp: &SExpression) -> Option<SExpression> {
    fn eval_list(list: &Vec<SExpression>) -> Option<SExpression> {
        let mut new_list : Vec<SExpression> = Vec::new();
        let mut current = list.iter();
        while let Some(sexp) = current.next() {
          match sexp {
              SExpression::Atom(a) if a == "car" =>
                  return match current.next() {
                      Some(sexp) => Some(car(&sexp)?.clone()),
                      _ => None,
                  },
              _ => new_list.push(sexp.clone()),
```
Fixed bugs
Starting to get wary
---
Fixes allow more interesting expressions, including nested evaluations:
```rust
#[test_case("(car ( ((hotdogs)) (and) (pickle) relish ) )", "((hotdogs))"; "eval: car nested list")]
#[test_case("(car (car ( ((hotdogs)) (and) (pickle) relish ) ) )", "(hotdogs)"; "eval: nested car")]
```
---

# Chapter 1. Page 6...
## What is the *cdr* of `l` where `l` is 
> `(a b c)`

*`(b c)`, because `(b c)` is the list without `(car l)`*

---

```rust
fn cdr(sexp: &SExpression) -> Option<SExpression> {
    match sexp {
        SExpression::List(list) if !list.is_empty() => Some(SExpression::List(list[1..].to_vec())),
        _ => None,
    }
}
```
---
<!-- and to add to evaluation... -->

```rust
fn eval(sexp: &SExpression) -> Option<SExpression> {
    fn eval_list(list: &Vec<SExpression>) -> Option<SExpression> {
        while let Some(sexp) = current.next() {
          match sexp {
              SExpression::Atom(a) if a == "car" =>
                  ...,
              SExpression::Atom(a) if a == "cdr" =>
                  return match current.next() {
                      Some(sexp) => match eval(sexp) {
                          Some(sexp) => Some(cdr(&sexp)?), 
                          _ => None,
                      },
                      _ => None,
                  },
```

---
```rust
#[test_case("(cdr (hamburger) )", "()"; "eval: cdr 1-list")]
#[test_case("(cdr a)", "Bad eval!"; "eval: cdr of atom")]
#[test_case("(cdr ())", "Bad eval!"; "eval: cdr of empty list")]
#[test_case("(car (cdr ((b) (x y) ((c))) ))", "(x y)"; "eval: car cdr")]
```
---
# Add cons
## What is the `cons` of atom `a` and list `l` where
> `a` is `peanut`
> `l` is `(butter and jelly)`

*`(peanut butter and jelly)`, because `cons` adds an atom to the front of a list*

---
```rust
fn cons(atom: &SExpression, list: &SExpression) -> Option<SExpression> {
    match list {
        SExpression::List(list) => {
            let mut list = list.clone();
            list.insert(0, atom.clone());
            Some(SExpression::List(list))
        },
        _ => None,
    }
}
```

<!-- simple addition of boilerplate to `eval` but can we reduce duplication -->

---

# Use member functions to allow `?`
<!-- although wary, in c++, of member functions if free functions can do the same
- shrinks from 2 pages to 20 lines
-->

```rust
impl SExpression {
    fn eval(&self) -> Option<SExpression> {
        fn eval_list(list: &Vec<SExpression>) -> Option<SExpression> {
            let mut new_list : Vec<SExpression> = Vec::new();
            let mut current = list.iter();
            while let Some(sexp) = current.next() {
              match sexp {
                  SExpression::Atom(a) if a == "car" => return car(&current.next()?.eval()?),
                  SExpression::Atom(a) if a == "cdr" => return cdr(&current.next()?.eval()?),
                  SExpression::Atom(a) if a == "cons" => return cons(&current.next()?.eval()?, &current.next()?.eval()?),
                  _ => new_list.push(sexp.clone()),
              }
            }
            Some(SExpression::List(new_list))
        }
        match &*self {
            SExpression::List(list) => eval_list(&list),
            SExpression::Atom(s) => Some(SExpression::Atom(s.to_string())),
```
---

```rust
#[test_case("(cons peanut ())", "(peanut)"; "eval: cons into empty list")]
#[test_case("(cons () ())", "(())"; "eval: cons empty list into empty list")]
#[test_case("(cons peanut (butter and jelly))", "(peanut butter and jelly)"; "eval: cons")]
```

---

## Is it true that list `l` is the `null` list where
> `l` is `()`

*Yes, because it is the list composed of zero S-expressions*

```rust
#[test_case("(null? spaghetti)", "Bad eval!"; "eval: null? atom")]
#[test_case("(null? ())", "true"; "eval: null? empty list")]
#[test_case("(null? (()))", "false"; "eval: null? non-empty list")]
#[test_case("(null? (car (())))", "true"; "eval: null? car non-empty list")]
#[test_case("(quote ())", "()"; "eval: quote")]
```

---
# Move Scheme functions to members
```rust
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
```
---

```rust
    fn eval(&self) -> Option<SExpression> {
        fn eval_list(list: &Vec<SExpression>) -> Option<SExpression> {
            while let Some(sexp) = current.next() {
              match sexp {
                  SExpression::Atom(a) if a == "car" => return current.next()?.eval()?.car(),
                  SExpression::Atom(a) if a == "cdr" => return current.next()?.eval()?.cdr(),
                  SExpression::Atom(a) if a == "cons" => return current.next()?.eval()?.cons(&current.next()?.eval()?),
                  SExpression::Atom(a) if a == "null?" => return current.next()?.eval()?.is_null(),
                  SExpression::Atom(a) if a == "quote" || a == "'" => return current.next()?.quote(),
```

<!-- Consider storing a map of function names to function objects
- extendable for user functions
-->
---
# Simplify errors
If more input is legal then there's less error checking needed
<!-- This is an example of a different dialect of Lisp or Scheme
-->

Instead of
```rust
fn is_null(&self) -> Option<SExpression>
...
#[test_case("(null? spaghetti)", "Bad eval!"; "eval: null? atom")]
```
Allow any parameter:
```rust
fn is_null(&self) -> SExpression
...
#[test_case("(null? spaghetti)", "false"; "eval: null? atom")]
```
---

# Add more functions
## `atom?`, `eq?`, `lat?`

```rust
#[test_case("(atom? Harry)", "true"; "eval: atom? atom")]
#[test_case("(atom? (Harry had a heap of apples))", "false"; "eval: atom? list")]

#[test_case("(eq? 7 7)", "true"; "eval: eq? same numbers")]
#[test_case("(eq? (car (Mary had a little lamb)) Mary)", "true"; "eval: eq? car")]
#[test_case("(eq? (cdr (soured milk)) milk)", "false"; "eval: eq? cdr list and atom")]

#[test_case("(lat? (Jack Sprat could eat no chicken fat) )", "true"; "eval: lat? list of atoms")]
#[test_case("(lat? ((Jack) Sprat could eat no chicken fat) )", "false"; "eval: lat? list including list")]
```

<!--
Weird to limit `eq?` to atoms!
It would be nice to describe eq?(list list) within scheme.
But nice excuse to try iterate functions.
-->

<!-- list of atoms -->

---
# Define `lat?` within lisp
## Write `lat?` using any of `car`, `cdr`, `cons`, `null?`, `atom?`, `eq?`

Tempting: Is the item, `l`, not an atom and is `car l` an atom and is `lat? (cdr l)` 

*Not possible, but introduces defining things, lambda functions, recursion and the `cond` function*

---
# Chapter 2: Do It, Do It Again, and Again, and Again...

```lisp
(define lat?
  (lambda (l)
    (cond
      ((null) #t)
      ((atom? (car l)) (lat? (cdr l)))
      (else #f))))
```

---

# Add conditional evaluation

to be able to write tests such as:
```rust
#[test_case("(cond (true a) )", "a"; "eval: cond true")]
#[test_case("(cond (false a) (true b) )", "b"; "eval: cond false true")]
#[test_case("(cond (true a) (true b) )", "a"; "eval: cond first result")]
#[test_case("(cond (false a) )", "Bad eval!"; "eval: cond no result")]
#[test_case("(cond ((eq? a a) equal) (true not-equal) )", "equal"; "eval: cond apply eq? to same")]
#[test_case("(cond ((eq? a b) equal) (true not-equal) )", "not-equal"; "eval: cond apply eq? to different")]
#[test_case("(cond ((eq? a b) equal) (else not-equal) )", "not-equal"; "eval: cond use else")]
```

---
```rust
    fn cond(&self, conditions: & mut std::slice::Iter<SExpression>) -> Option<SExpression> {
        let applicable_condition = conditions.find(|&condition| {
            match condition {
                SExpression::List(condition) if condition.len() > 1 => match condition[0].eval() {
                    Some(condition) => condition.is_true(),
                    _ => false,
                },
                _ => false,
            }
        });
        match applicable_condition {
            Some(SExpression::List(condition)) => condition[1].eval(),
            _ => None,
```
*Using rust iterators and algorithms!*

<!--
Adding cond(): passing the iterator in for parameters was nice.
- Would this be better for the other functions, rather than the eval layer extracting the parameters when the number is known?
-->

<!-- I did cheat by internally defining the evaluation of `else` to be true, so I could use it as a syntactically pleasing last clause -->

---
# Work towards define
## Start tracking the environment
<!-- in which the evaluation should take place -->
```rust
type Environment = HashMap<String, SExpression>;
..
    fn eval(&self, env: &mut Environment) -> Option<SExpression> {
        fn eval_list(list: &Vec<SExpression>, env: &mut Environment) -> Option<SExpression> {
            while let Some(sexp) = current.next() {
              match sexp {
                  SExpression::Atom(a) if a == "car" => return current.next()?.eval(env)?.car(),
                  SExpression::Atom(a) if a == "cdr" => return current.next()?.eval(env)?.cdr(),
                  ..
                  SExpression::Atom(a) if a == "cond" => return sexp.cond(&mut current, env),
```
---
# Add define
```rust
    fn define(&self, other: &SExpression, env: &mut Environment) -> Option<SExpression> {
        match *&self {
            SExpression::Atom(s) => {
                env.insert(s.to_string(), other.clone());
                Some(SExpression::Atom(s.clone()))
            },
            _ => None,
```
---
# What should `define` return?

- lisp and scheme references say its a control structure and not an expression
- but in recursive evaluation everything returns something or fails with None
  - maybe relax this so that None doesn't contribute to the list but continues with the next element?
- return name of thing defined for initial experiments
  - later changed to return `()`

---
# Defining `else` and using it
```rust
#[test_case("(define a b)", "b"; "eval: define isolated")]
#[test_case("( (define a b) a)", "(b b)"; "eval: define substitute definition")]
#[test_case("((define else true) (cond ((eq? a b) equal) (else not-equal)) )", "true not-equal"; "eval: cond use else")]
```

*Are sequences legitmate lisp?*
<!-- No examples of multiple statements making use of definitions -->

---
# Next steps - lambdas

- Maybe initially lambdas that don't have a closure
- but the structure is starting to feel a bit artificial and hacky
- and a growing number of questions make me want to start over.
- Numerous problems with our Environment:
 - likely to blow up for any cyclic lookup
  - not scoped (allowing overrides and overlong lifetimes)
  - not taking account of symbols meaning different things in different scopes
    - maybe only relevant inside lambdas?
  - significant cloning going on
  - need to better handle `define` not returning a value

---
# ReWrite Ideas
- not changing the inputs, so should be able to just reference, and use rust to enforce rather than `clone()`
- store function implementations as function pointers or objects that can be looked up or invoked
  - and allow user functions to be equally used 
- thinking about the environment as an immutable list, with the head branching, makes closures much more straight forward
- which implies using a list=head+tail structure much more in the implementation of the interpreter, rather than list and map
- maybe change the process to 3 passes:
  - parse text into tokens
  - reform tokens into list of lists of lists ...
  - eval is just working on the head of each list as the function and the tail as the parameters
- consider relaxing errors from excess parameters. Just ignore them.

---
# Ambitions
- Defining new functions inside scheme rather than outside
- Ultimate aim: what is the most minimal rust implementation?
- If we're starting to use longer scripts, would it be better to have the test functions inside scheme?
  - Could build them within scheme, if we had an output function to deliver info?

# Boring but useful stuff like:
- Would be nice if compiler errors in output hinted the problem
- Include or Import mechanism for definitions

---

# Questions and Ideas?
