# rmenu

`rmenu` is a simple Rust command-line menu framework.
It's lightweight and easily configurable.

---

## Overview

The core idea of `rmenu` is to:
- Register commands with short and long names.
- Display an interactive textual menu.
- Parse user input into typed arguments (`Int`, `Double`, `String`).
- Invoke callback functions linked to commands.

The included commands are:
- `g` / `go` — Runs a sample `go` function, printing parsed arguments.
- `q` / `quit` — Exits the program.

---

## Example Run

When executed, you’ll see something like:

```cpp
Menu
g go
q quit
<select>:

Press Enter to continue back to the menu.

Entering a command—for example:

g 42 hello 3.14

Produces output:

go int: 42 string: hello double: 3.14

```

---

## Features

- Simple CLI menu system.
- Dynamic callback registration.
- Argument parsing into integers, floats, or strings.
- Lightweight — no dependencies outside the Rust standard library.

---

## Building and Running

### Prerequisites
- **Rust** (1.70+ recommended)

### Steps
```bash
git clone https://github.com/kipm808/rmenu.git
cd rmenu
cargo run
```

---

## Code Structure

- **`Arg` enum:** Represents argument types (Int, Double, String).
- **`CallbackFunc` type alias:** Defines function signatures for callbacks.
- **`Callbacks` struct:** Manages command registration and lookup.
- **`Menu` struct:** Handles UI loop, rendering, and input parsing.
- **`main()` function:** Instantiates the menu and runs the loop.

---

## Example Callback Definition

Here’s a simplified example showing how to define a new callback:

```rust
fn hello(args: Vec<Arg>) {
    println!("Hello command executed with {:?}!", args);
}
```

To add this command to the menu:
```rust
cb.add_callback("h", "hello", hello);
```

---

## Adding Boolean Support

Extend `rmenu` to handle **boolean arguments** (`true`/`false`, `1`/`0`):

### 1. Update `Arg` enum
```rust
#[derive(Debug)]
enum Arg {
    Int(i32),
    Double(f64),
    String(String),
    Bool(bool),  // Add this variant
}
```

### 2. Extend `parse_args()`
```rust
fn parse_args<'a>(&self, tokens: impl Iterator<Item = &'a str>) -> Vec<Arg> {
    tokens.map(|token| {
        if let Ok(i) = token.parse::<i32>() {
            Arg::Int(i)
        } else if let Ok(d) = token.parse::<f64>() {
            Arg::Double(d)
        } else if token.eq_ignore_ascii_case("true") || token == "1" {
            Arg::Bool(true)
        } else if token.eq_ignore_ascii_case("false") || token == "0" {
            Arg::Bool(false)
        } else {
            Arg::String(token.to_string())
        }
    }).collect()
}
```

### 3. Create boolean callback
```rust
fn status(args: Vec<Arg>) {
    print!("status ");
    for arg in args {
        match arg {
            Arg::Bool(b) => print!("bool:{} ", b),
            Arg::Int(i) => print!("int:{} ", i),
            Arg::Double(d) => print!("double:{:.1} ", d),
            Arg::String(s) => print!("str:'{}' ", s),
        }
    }
    println!();
}
```

### 4. Register callback
```rust
cb.add_callback("s", "status", status);
```

---

## License

This project is released under the MIT License.  

---

## Author

Developed by Kip McAtee.  

