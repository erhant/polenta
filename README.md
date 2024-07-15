# Polenta

Polenta is a polynomial arithmetic language, and **everything** is a polynomial. As such, it is more suited towards having fun instead of having performance.

## Usage

In this section we go over what can be done with Polenta. All code snippets below are valid Polenta code.

### Statements

You can use Polenta as a calculator-over-field.

```rs
10 - 2^3 * 2
```

> [!NOTE]
>
> The value of last evaluated expression statement is stored at `!!` symbol.

### Creating a Polynomial

A polynomial is created by specifying its terms.

```rs
let P(x) = 3*x^2 + 5;
```

Behind the scenes all terms are considered a monomial, and they are composed together. As such, you can repeat a term with the same degree and the result will have them added up:

```rs
let P(x) = 3*x^2 + 2*x^2 + 1*x^2; // 6*x^2
let Q(x) = 5*x - 5*x + 1; // 1
```

> [!TIP]
>
> Polenta supports single line comments, as shown above.

You can also multiply polynomials like:

```rs
let P(x) = (x + 1)*(x + 2)*(x + 4); // x^3 + 7*x^2 + 14*x + 8
```

You can create a polynomial from an existing one:

```rs
let P(x) = 3*x;
let Q(x) = P^2 + 2*P; // (3*x)^2 + 2*(3*x) = 9*x^2 + 6*x
```

Polenta allows shadowing to overwrite existing polynomials:

```rs
let P = 3*x;
let P = 3*P + 5; // 9*x + 5
```

### Evaluating A Polynomial

Evaluation is achieved using a binary operation `@`, so that `P@2` means "evaluate polynomial `P` at point `2`.

```rs
let P(x) = 3*x;
let z = P@3; // 9
```

We can later use these values if we would like:

```rs
let Q(x) = x^z - z*x; // x^9 - 9*x
```

Remember that everything is a polynomial in Polenta, so you could evaluate a number as well. Evaluation will not have an effect because a number is treated as a constant polynomial.

```rs
4@2 // 4
```

Since evaluation is a binary operation, you can chain them together:

```rs
let P(x) = 3*x + 1;
let Q(x) = x/2;
Q@P@Q@P@3; // Q(P(Q(P(3))))
```

## Testing

Run all tests with:

```sh
cargo test
```

## License

Polenta is MIT licensed.
