# Advent of Code Solutions 🎄✨

🎅 **IT'S TIIIIIME** — You know it’s December when Mariah Carey defrosts, your
code starts resembling spaghetti, and regex becomes both your best friend and
your worst enemy.

Welcome to my Advent of Code Solutions! This is the place where festive cheer
collides with way too much coffee and questionable coding practices. Every day,
I'll crack open a new puzzle and try to solve it without turning into the
Grinch.

## 🎁 What's This?

Advent of Code is an annual coding event where every day from December 1st to
25th, a new programming puzzle is released. It’s like an advent calendar, but
instead of chocolate, you get headaches (and maybe a little satisfaction).

This repo contains solutions written entirely in **Rust**, for those who think
debugging the borrow checker is the perfect holiday activity.

## 🎄 Structure

```
├── shared/                       # Shared library with helpers (utilities used across solutions)        
├── template/
│   └── src/
│       └── main.rs              # Template maker for starting new daily solutions
├── year-<year>/
│   ├── inputs/                  # Input files (not committed to avoid spoilers)
│   ├── src/
│   │   ├── main.rs              # Runner for executing all daily solutions for the year
│   │   └── bin/
│   │       └── day-<day>.rs     # Individual solutions for each day's problem
│   └── test-input/
│       └── day-<day>(-part<part>).txt  # Test inputs downloaded from the AoC site
```

## 🎉 How to Use

1. Clone the repository `git clone https://github.com/Mintia11/advent-of-code`
2. Run all the year's solutions or a single one `cargo run --bin year-<year>` or
   `cargo run --bin year-<year>_day-<day>`

> 🎁 Happy Coding! And remember, even if the leaderboard mocks you, the real win
> is finishing a single day without a runtime panic.
