Learning Scheme and Rust

Scheme (and Lisp in general) seems to be a very pure way of investigating programming. There is less distraction with syntax, so more focus on the computation described.

The highly recommended 'The Little Schemer' book turned out to be a very Socratic tutorial, comprised entirely of questions, allowing the reader to progressively build up their understanding.

To force myself to read, consider and answer the questions, I decided to incrementally write a parser that could answer the questions for me.

Writing the parser in Rust would provide an opportunity to learn Rust from scratch. It would also give me an excuse to initially write very crude parsers in rudimentary Rust, and then rewrite more sophisticated parsers as my knowledge of Rust syntax and techniques increased.

# Create little_schemer project

- Start with command line
- Use Rust standard naming:
  - Use lower camel case names, apart from upper case for Types.
- VisualStudioCode
  - Configure building and running tests. Sometimes outputs usefully to Terminal and sometimes doesn't.

## Is a string an atom

- Difference between String and str
- Iterate over a collection
- Need to dereference references to use as value (e.g. for comparisons) but not for calling methods (but can; syntactic sugar)
- Experimented with minimizing `return`. Doesn't seem to like the last line of a general block returning a value, or returning the value of a block as an expression e.g. to simplify `is_atom` to a functional expression e.g.

```rust
fn is_atom(text: &str) -> bool {
    !text.is_empty()
    &&
    {
        for c in text.chars() {
            if !is_character(&c) {
                false;
            }
        }

        true
    }
}
```

## Is string a list

Crude state machine: loop over chars with mutable flags

# Replace with Lexer

- Better tool for analyzing the atoms and lists.
- Still using mutable state

Enum: can't use an individual enum value as a type even though it can have varying state e.g. fnc(SExpression) -> SExpression::List

Consider making functions return nil or () instead of None - removing a layer of Option.
- General idea that an additional enum value could replace wrapping the enum in an option, but then uses are forced to test Nil everytime i.e. can't use types to enforce initialized.
- but if everything receives and returns Option then might as well remove the extra layer? But then can't take advantage of ?

Consider relaxing errors from excess parameters. Just ignore them.

Consider replacing Vec in list with a linked list (head+body).
This might then allow all strings from the original string to be used by reference.
Good test of Rust helping us manage reference safely?

Adding new functions is becoming a pattern of coding the implementation and coding the parsing redirection.
- If many more then then consider adding structure to enforce name, parameters, lookup.

Starting to think about defining new functions inside scheme rather than outside.
Ultimate aim: what is the most minimal rust implementation?
- could it be a macro implementation of a DSL version?

```rust
        if let SExpression::Atom(s_self) = self &&
            let SExpression::Atom(s_other) = other &&
            s_self == s_other {
                return SExpression::Atom(String::from("true"))
            }
```
`let` expressions in this position are experimental

```rust
        SExpression::Atom( ( {
            if let SExpression::Atom(s_self) = self {
                if let SExpression::Atom(s_other) = other {
                    s_self == s_other
                }
            }
            false
        }).to_string())
```
illegal

It would be nice to describe eq?(list list) within scheme.
But nice excuse to try iterate functions.

Adding cond(): passing the iterator in for parameters was nice.
- Would this be better for the other functions, rather than the eval layer extracting the parameters when the number is known?

It definitely feels like we should be able to define `else` equivalent to `true` inside scheme.

Would be nice if compiler errors in output showed the top.

No examples of multiple statements making use of definitions

What should define return?
- lisp and scheme references say its a control structure and not an expression
- but in recursive evaluation everything returns something or fails with None
  - maybe relax this so that None doesn't contribute to the list but continues with the next element?
- return name of thing defined for initial experiements

Interesting:
```
        SExpression::Atom(_) => match sexp.eval(env) {
            Some(SExpression::Atom(s_)) => s += &s_,
            Some(sexp) => s += &sexpression_to_string(&sexp, env),
            _ => (),
        },
```
Difficulty returning empty string

Difficulty accessing a hash map and then using it:
```
            SExpression::Atom(s) => match env.get(s) {
                Some(sexp) => sexp.eval(env),
                _ => Some(self.clone()),
            },
```
Rather than maintaining our own Environment, could we just use Rust's closures? Store them?

Numerous problems with our Environment:
- likely to blow up for any cyclic lookup
- not scoped (allowing overrides and overlong lifetimes)
- direct replacement: not taking account of symbols meaning different things in different scopes
  - maybe only relevant inside lambdas?
- significant cloning going on
- need to better handle define not returning a value

If we're starting to use longer scripts, would it be better to have the test functions inside scheme?
Could build them within scheme, if we had an output function to deliver info?
But we already do have that, so maybe overthinking?

Lambdas are the big remaining challenge
