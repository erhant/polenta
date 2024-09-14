# Polenta REPL

A simple command-line tool to play around with [Polenta](../polenta/).

## Installation

```sh
cargo install --git TODO:!!!
```

## Usage

Launch the CLI app:

```sh
polenta-cli
```

You will be greeted with a prompt screen, here you can write Polenta code! The last evaluated statement will be printed to the screen.

```sh
> let x = 6 + 7;
13
> let y = x ^ 3;
2197
> y;
2197
```

To exit, simply type `.exit`:

```sh
> .exit
bye!
```

### Errors

If there is an error, you will see it on screen.

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
