# `bioneer` - Bionic Reading Library

The `bioneer` crate is a Rust port of the JavaScript library [text-vide](https://github.com/Gumball12/text-vide), providing a bionic reading library for converting text into a bionic reading format. This library enhances the readability and comprehension of text by applying formatting techniques. It provides a trait for bionifying strings.

## Installation

Add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
bioneer = "0.1"
```

## Usage

Import the necessary items into your Rust code:

```rust
use bioneer::Bionify;
```

### Bionify Trait

The `Bionify` trait provides a method for bionifying strings.

#### Required Method

##### `fn bionify(&self) -> String`

This method converts the implementing string into a bionic reading format.

```rust
use bioneer::Bionify;

let text = "Hello, world!";
let bionified_text = text.bionify();
println!("{}", bionified_text);
// "<b>Hel</b>lo, <b>Wor</b>ld!" 
```
**Hel**lo, **Wor**ld!

## License

This crate is distributed under the terms of the MIT license. See the [LICENSE](https://opensource.org/licenses/MIT) file for details.
