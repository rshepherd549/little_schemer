Learning Scheme and Rust

Scheme (and Lisp in general) seems to be a very pure way of investigating programming. There is less distraction with syntax, so more focus on the computation described.

The highly recommended 'The Little Schemer' book turned out to be a very Socratic tutorial, comprised entirely of questions, allowing the reader to progressively build up their understanding.

To force myself to read, consider and answer the questions, I decided to incrementally write a parser that could answer the questions for me.

Writing the parser in Rust would provide an opportunity to learn Rust from scratch. It would also give me an excuse to initially write very crude parsers in rudimentary Rust, and then rewrite more sophisticated parsers as my knowledge of Rust syntax and techniques increased.

## Create little_schemer project

- Start with command line
- Use Rust standard naming:
  - Use lower camel case names, apart from upper case for Types.
- VisualStudioCode
  - Configure building and running tests

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



