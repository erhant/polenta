# Polenta

Polenta is a polynomial arithmetic language.

## Creating a Polynomial

A polynomial is created by specifying its terms, which is in order: (coefficient, term, exponent). It is assigned to an identifier, in the form `let <uppercase_id> = <polynomial>`.

The expression below stands for $p = 3x^2 + 5$.

```rs
let P = 3*x^2 + 5;
```

Behind the scenes all terms are considered a monomial, and they are added together. As such, you can repeat a term with the same degree and the result will have them added up:

```rs
let P = 3*x^2 + 2*x^2 + 1*x^2; // equals 6*x^2
let Q = 5*x - 5*x + 1; // equals 1
```

You can also multiply polynomials like:

```rs
let P = (x - 1)*(x - 2)*(x - 4);
```

You can create a polynomial from an existing one:

```rs
let P = 3*x;
let Q = P^2 + 2*P; // (3*x)^2 + 2*(3*x) = 9*x^2 + 6*x
```

Polenta allows shadowing to overwrite existing polynomials:

```rs
let P = 3*x;
let P = 3*P + 5; // 9*x + 5
```

## Evaluating A Polynomial

The notation of polynomial evaluation is done by simply "calling" the polynomial's identifier.

```rs
let P = 3*x;
let P_3 = P(3); // 9
```

We can later use these values if we would like:

```rs
let Q = x^P_3 - P_3*x; // x^9 - 9*x
```
