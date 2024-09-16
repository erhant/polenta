# Polenta

Polenta is a toy polynomial arithmetic language, and **everything** is a polynomial. As such, it is more suited towards having fun instead of having performance.

It features:

- [x] Polynomial arithmetic using [Lambdaworks](https://github.com/lambdaclass/lambdaworks)
- [x] REPL to play around with
- [x] `let` and `assert` expressions
- [ ] `prove` and `commit` expressions

> [!NOTE]
>
> The project mostly started to learn more about Pest, and it is a lovely project! See <https://pest.rs> for more!

## Installation

You can add the library with:

```sh
cargo add polenta
```

You can install the REPL with:

```sh
cargo install polenta --bin repl --features="repl"
```

## Usage

In this section we go over what can be done with Polenta.

> [!TIP]
>
> All code shown below are valid Polenta code. To follow along, you can use the REPL installed above!

### Statements

You can use Polenta as a calculator-over-field.

```rs
> 10 - 2^3 * 2;
18446744069414584315
```

However, you should keep in mind that all operations are defined over the field, so division might not always work as you expect:

```rs
> 1 / 2; // some number x such that 2*x = 1, not 0.5!
9223372034707292161
```

> [!TIP]
>
> Polenta supports single line comments using `//`, as shown above.

### Creating a Polynomial

A polynomial is created by specifying its terms.

```rs
> let P(x) = 3*x^2 + 5;
3*x^2 + 5
```

Behind the scenes all terms are considered a monomial, and they are composed together. As such, you can repeat a term with the same degree and the result will have them added up:

```rs
> let P(x) = 3*x^2 + 2*x^2 + 1*x^2;
6*x^2
> let Q(x) = 5*x - 5*x + 1;
1
```

You can also multiply polynomials:

```rs
> let P(x) = (x + 1)*(x + 2)*(x + 4);
x^3 + 7*x^2 + 14*x + 8
```

You can create a polynomial from an existing one:

```rs
> let P(x) = 3*x;
3*x
> let Q(x) = P^2 + 2*P;
9*x^2 + 6*x
```

Shadowing is allowed:

```rs
> let P(x) = 3*x;
3*x
> let P(x) = 3*P + 5;
9*x + 5
```

You can use an identifier within a polynomial, but if the identifier has the same name as the term, it will be ignored.

```rs
> let t = 2;
> let x = 5;
> let P(x) = x^t + 2*x;
x^2 + 2*x
```

### Equality

Polenta has `==` and `!=` operators that return either a 1 or 0 based on the equality.

```rs
> 2 == 3
0
> let P(x) = 2*x
2*x
> let Q(x) = 3*x - x
2*x
> P == Q
1
```

### Evaluating a Polynomial

Evaluation is achieved using a binary operation `@`, so that `P@2` means "evaluate polynomial `P` at point `2`.

```rs
> let P(x) = 3*x;
3*x
> let z = P@3;
9
> let Q(x) = x^z + z*x;
x^9 + 9*x
```

Remember that everything is a polynomial in Polenta, so you could evaluate a number as well. Evaluation will not have an effect because a number is treated as a constant polynomial.

```rs
> 4@2
4
```

Since evaluation is a binary operation, you can chain them together:

```rs
> let P(x) = 3*x + 1;
3*x + 1
> let Q(x) = x/2;
9223372034707292161*x
> Q@P@Q@P@3; // Q(P(Q(P(3))))
8
```

### Assertions

You can make assertions within Polenta for safety, where a failed assertion throws an `AssertionError`.
An assertion fails if the expression that they are given is a zero polynomial.

```rs
> assert 1
1
> assert 0
  × Assertion Failed

> assert 43
43
```

### Errors in REPL

While using REPL, if there is an error you will see it on screen with clear logs.

```sh
> let x = idontexist;
  × Unknown Identifier: idontexist
> 5/0;
  × Division by Zero
> let a = ++syntaxerror--;
  × Syntax Error
   ╭─[input:1:9]
 1 │ let a = ++syntaxerror--;
   ·         ─
   ╰────
  help: Expected one of [expr], got []
```

## Testing

Run all tests with:

```sh
cargo test
```

## License

Polenta is MIT licensed.
