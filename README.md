# λ-script
λ-script is an implementation of the [λ-calculus](https://en.wikipedia.org/wiki/Lambda_calculus) in Rust. This project is still in its very early stages. Running the program will invoke a small demo which parses the following input:

```
true  = λx.λy.x
false = λx.λy.y

if = λb.λt.λf.b t f
not = if false true

not not true
```

and begins evaluating `not not true`. The evaluation advances by one operation every time stdin advances a line. The eventual output looks like the following:

```
not not true
if false true not true
(λb.λt.λf.b t f) false true not true
(λt.λf.false t f) true not true
(λf.false true f) not true
false true not true
(λx.λy.y) true not true
(λy.y) not true
not true
if false true true
(λb.λt.λf.b t f) false true true
(λt.λf.false t f) true true
(λf.false true f) true
false true true
(λx.λy.y) true true
(λy.y) true
true
```

##### _N.B._, the "λ" symbol

This program and its source code make heavy use of the lambda character, `λ`. It is useful to be able to insert this symbol conveniently. The author of this software uses a [compose key](https://en.wikipedia.org/wiki/Compose_key) to accomplish this. For Linux installations utilizing the X Window System, this is as simple as including the following line in your `~/.XCompose` file:
```
<Multi_key> <backslash>: "λ"
```
The compose key can be mapped to right alt by executing the following command: `setxkbmap -option compose:ralt`, which can be included in an `.xinitrc` or `.xprofile` file to enable the compose key on startup. The combination of these two commands allows one to insert a λ by pressing `Right Alt + \`.

Similar solutions exist for macOS; see [gnarf/osx-compose-key](https://github.com/gnarf/osx-compose-key).

## Installation

Rust and Cargo must be installed. I recommend using [rustup](https://rustup.rs/).

```
git clone https://github.com/z2oh/lambda-script
cd lambda-script
cargo build --release
```

The binary is then located at `./target/release/λ-script`.
