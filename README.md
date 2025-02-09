# Chalk

[Video Demo](https://www.youtube.com/watch?v=zLOluBnUr-o)

A crate for terminal colors and styles

```rust
use lealang_chalk_rs::Chalk;
let mut chalk = Chalk::new();
chalk.red().println(&"This text is red");
chalk.bold().println(&"Now it's red AND bold");
```

That's an example of basic color. There are three types of color in chalk:
BasicChalk, AnsiChalk, and RgbChalk.

```rust
use lealang_chalk_rs::Chalk;
let mut chalk = Chalk::new();
chalk.ansi(56).println(&"Purple-ish");
chalk.rgb(25, 125, 63).println(&"This color is ugly");
```

Chalk can aldo do _styling_! Here's an example:

```rust
use lealang_chalk_rs::Chalk;
let mut chalk = Chalk::new();
chalk.bold().println(&"Bold!");
```
