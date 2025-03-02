docs resources https://www.coursera.org/learn/rust-fundamentals https://learn.microsoft.com/en-us/training/paths/rust-first-steps/

# Idea:

- I saw on Hacker News that this guy would essentially start prototyping in Python in the context of developing computer vision problems, however, then there would be another step of trying to optimize on the embedded in Rust. That made me to think, what's the need of switching environments when you could directly start in Rust light?

- for live reloading (the difference between hot vs live reloading lies in it scope of rebuilding. live reloading rebuilds the entire application where hot reloading is aiming to just change the lines concerned)

- for live reloading very simply you could start: `cargo watch -x 
run`

- for REPL (Read-Eval-Print LOop) functionality use Evcxr
- Evcxr is a standalone REPL + Jupyter kernel for notebook-based development
- in Evcxr you can for instance add a dependency dynamically using `:dep` without the need to manually change lines in Cargo.toml files: `cargo install --locked evcxr_jupyter` and `evcxr_jupyter --install`
- this workflow combines the benefits of literate programming with Rust's performance and safety guarantee
- the Jupyter integration has custom output formatting --> create rich HTML or image-based representations of the data structures, for visualization tasks
- for instance, Plotter library

essentially two roots:
= EVCXR REPL with cargo-watch
= EVCXR with Jupyter
# cv-rust-playground
