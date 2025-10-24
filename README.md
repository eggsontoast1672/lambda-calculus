# Lambda Calculus Interpreter

This is an interpreter for a small, lambda calculus-like language.

## Compilation

I have tried my hardest to keep this interpreter dependency-free. In my
opinion, Rust's biggest problem is the NPM effect (see *npm left-pad incident*
on Wikipedia). Almost every project will take on dependencies to solve trivial
problems, often leading to very long compilation times. I'm proud to be part of
the solution!

Compiling and running this project is as easy (easier, in fact, see above) as
compiling any other Rust project. Just run the command `cargo run --release` in
your shell to run the code.

## Usage

### Quick Start Guide

This program is really not hard to use. When you execute it (see above), you
will be thrown into an interactive prompt. You can type out Lambda Calculus
expressions, they will be evaluated, and the result will be displayed
underneath. The syntax is explained further down this document. For now, here
are some examples to pique your interest:

```
> (\x.x identity)
identity
> ((\first.\second.first one) two)
one
> names-are-themselves
names-are-themselves
> \so-are.functions
\so-are.functions
```

### Language Syntax

This language is completely expression-based, and there are three syntactic
forms. Those are names, functions, and applications. A name is any string of
non-whitespace characters which does not contain the characters '(', ')', '\\',
or '.'. These characters have reserved roles in the syntax, so it would be
weird if they could occur in names. Here are some examples of valid names:

- `i_am_a_variable`
- `<me-too>`
- `23740921874`
- `8========D`
- `"thisLooksLikeAStringButIsAVariable"`

Beware that this language does not have support for string literals in the
normal sense, so `"Hello, world!"` is interpreted as two seperate names, the
first being `"Hello,` and the second being `world!"`.

The next type of expression is a function. This type of expression is also
known in Lambda Calculus literature as a *lambda abstraction*, but it is
referred to throughout the code as a function. Functions begin with a backslash
(simulating a lambda symbol) followed by a name, a period, and an expression.
Here are some examples:

- Identity function: `\\x.x`
- The K-combinator: `\\first.\\second.first`

Note that functions of multiple arguments can be constructed by chaining
together functions of a single argument, as above in the K-combinator.

The last type of expression is an application, or *bound pair* in the
literature. It looks like a pair of parentheses with two expressions inside.
Relative to the other expression types, this one is not as nuanced. Here are
some examples:

- `(x y)`
- `((\\first.\\second.first one) two)`
- `(\\p.q \\r.s)`

### Evaluation Syntax

To do.

## Goals

- Add support for assignments
- Modernize language syntax
- Create some kind of syntax highlighting plugin, maybe a treesitter parser

## License

This code is licensed under the GNU GPLv3. I typically license my code under
MIT, but I have grown to apprecate the GPL. I like that it keeps my code away
from the filthy hands of the writers of proprietary software. Anyway, do what
you want with the software (except for including it in a closed source
project).
