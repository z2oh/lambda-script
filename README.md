# λ-script
λ-script is an implementation of the [λ-calculus](https://en.wikipedia.org/wiki/Lambda_calculus) in Rust. This project is still in its very early stages. Running the program will invoke a small demo which parses the following input:

```
0 = λf.λx.x
succ = λn.λf.λx.f (n f x)

1 = succ 0
2 = succ 1

add = λm.λn.m succ n

add 2 2
```

and begins evaluating `add 2 2`. The evaluation advances by one operation every time stdin advances a line. The eventual output looks like the following:

```
add 2 2
(λm.λn.m succ n) 2 2
(λn.2 succ n) 2
2 succ 2
succ 1 succ 2
(λn.λf.λx.f (n f x)) 1 succ 2
(λf.λx.f (1 f x)) succ 2
(λx.succ (1 succ x)) 2
succ (1 succ 2)
(λn.λf.λx.f (n f x)) (1 succ 2)
λf.λx.f (1 succ 2 f x)
λf.λx.f (succ 0 succ 2 f x)
λf.λx.f ((λn.λf.λx.f (n f x)) 0 succ 2 f x)
λf.λx.f ((λf.λx.f (0 f x)) succ 2 f x)
λf.λx.f ((λx.succ (0 succ x)) 2 f x)
λf.λx.f (succ (0 succ 2) f x)
λf.λx.f ((λn.λf.λx.f (n f x)) (0 succ 2) f x)
λf.λx.f ((λf.λx.f (0 succ 2 f x)) f x)
λf.λx.f ((λx.f (0 succ 2 f x)) x)
λf.λx.f (f (0 succ 2 f x))
λf.λx.f (f ((λf.λx.x) succ 2 f x))
λf.λx.f (f ((λx.x) 2 f x))
λf.λx.f (f (2 f x))
λf.λx.f (f (succ 1 f x))
λf.λx.f (f ((λn.λf.λx.f (n f x)) 1 f x))
λf.λx.f (f ((λf.λx.f (1 f x)) f x))
λf.λx.f (f ((λx.f (1 f x)) x))
λf.λx.f (f (f (1 f x)))
λf.λx.f (f (f (succ 0 f x)))
λf.λx.f (f (f ((λn.λf.λx.f (n f x)) 0 f x)))
λf.λx.f (f (f ((λf.λx.f (0 f x)) f x)))
λf.λx.f (f (f ((λx.f (0 f x)) x)))
λf.λx.f (f (f (f (0 f x))))
λf.λx.f (f (f (f ((λf.λx.x) f x))))
λf.λx.f (f (f (f ((λx.x) x))))
λf.λx.f (f (f (f x)))
```

The final result, `λf.λx.f (f (f (f x)))` is the Church numeral 4.

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
