# Why Maud ?

## The good

- It's **fast** (we'll see it more in depth later)
- It has an **elegant syntax**. Even tho this is a subjective point, for the vast majority of expessions, Maud has substantially less characters than HTML and incorporate CSS convetions (`#` <=> id, `.` <=> class) that are easily recognizable.
- Support for **Rust statements** (`if`, `match`, `for`, ...)
- **Auto-completion**

## The bad
- Doesn't have a **formatter**
- Doesn't have an amazing syntax **highlighting**. (Syntax highlighting of `.my-class` is bugged because of "-", excepted that, its normal!)
- It's a **procedural macro** (=> might increase compile time)
- Not based on any **standard**

## How it compares to other ?
### Minijinja
I made an entire [repository](https://github.com/tkr-sh/maud_vs_minijinja) just to compare [minijinja](https://github.com/mitsuhiko/minijinja) to maud.
The results are self explicits: Maud is **significantly** faster.

#### Recursive example:
- Maud: 1.823µs
- MiniJinja: 100.628µs

_Maud was faster by 98.805µs!!!_

#### Names example:
- Maud: 420ns
- MiniJinja: 70.488µs

_Maud was faster by 70.068µs!!!_

#### Raw html example
- Maud: 118ns
- MiniJinja: 69.433µs

_Maud was faster by 69.315µs!!!_

More than that:
- Maud's HTML is automatically compressed (not minijinja)
- You don't have to write that with the HTML-syntax
- You have support for the native rust statements
- You have auto-completion
