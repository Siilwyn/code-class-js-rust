# Code class from JS to Rust

## [Slides](https://siilwyn.github.io/code-class-js-rust/)

## Setup
```sh
git clone git@github.com:Siilwyn/code-class-js-rust.git
cd code-class-js-rust
```

## Run exercise
Use `cargo run -p` followed by the exercise directory name, for example to run the first exercise:
```sh
cargo run -p exercise-1
```

Tip, automatically run the exercise on changes with [cargo watch](https://github.com/passcod/cargo-watch):
```sh
cargo watch -x 'run -p exercise-1'
```

## Exercises

### One
Based on the compiler errors fix the code to print `2 + 1 = 3`.

### Two
Add matching quotes to certain users and filter out users with no matching quotes, so `{username} - {quote}` is printed for each quote.

## Solutions
To get the solution for an exercise apply the solution patch file, for example for the first exercise:
```sh
git apply exercise-1/solution.patch
```

[*<<* My previous code class, about GraphQL](https://github.com/voorhoede/code-class-graphql/)
