# sir-peaks
A program to compute the peak values of the infected population of a solution of the SIR model.
The SIR model is a mathematical model that represents the spread of an infection.
It is defined as the following differential equation:
dS/dt = -lS(t)I(t),
dI/dt = lS(t)I(t) - gI(t),
dR/dt = gI(t).
The variables S, I, and R represent the susceptible population, the infectious population, and the recovered population.

Reference: [https://www.ism.ac.jp/editsec/toukei/pdf/54-2-461.pdf](https://www.ism.ac.jp/editsec/toukei/pdf/54-2-461.pdf)

## How to build

The program is written in the Rust programming language.
You install the installer [rustup](https://rustup.rs/) and then you run the following command to build it:
```
cargo build --release
```

## How to use

The program has two subcommands: `simulate` and `peaks`.
The `simulate` subcommand generates a solution of the SIR model.
The `peaks` subcommand computes the peak values of the infectious population of solutions.
