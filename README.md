# Crest
<!-- TODO: Add a banner image, icons for build results, status on crates.io, etc -->

# Overview

Crest is a Rust library for working with CSS selectors and stylesheets. It provides tools to parse and validate
CSS syntax, match selectors against custom DOM-like structures, and apply styles programmatically.

## Status

Crest is nearing alpha, and is able to parse any css style rules but unable to parse at-rules. It can also only
validate a VERY limited subset of the standard style properties, and some [Peacock](#)-specific ones. It's already
fairly fast, with planned changes to validation that should make it <ins>***much***</ins> faster.

## Features

- **CSS Parsing**: Parse and validate CSS strings, including selectors and stylesheets.
- **Selector Matching**: Match parsed selectors against types implementing the `DomElement` trait.
- **Custom DOM Support**: Easily integrate with your own DOM-like structures by implementing the `DomElement` trait.

## Roadmap

- [X] Expand benchmark coverage.
- [ ] Document simple use cases.
- [ ] Define the `DomElement` trait.
- [ ] Add detailed examples for the `DomElement` trait.
- [ ] Document advanced use cases.

## Quickstart

### Installation

Crest is not yet available on [crates.io](https://crates.io/). To use it, include it as a dependency using a
git repository:

```toml
[dependencies]
peacock-crest = { git = "https://github.com/nucleus-labs/Crest/", rev = "<rev>" }
```

### Selector Parsing

Crest uses [Pest](https://pest.rs/) to generate parsers for CSS selectors and for the full CSS syntax. Here's
how you can parse a selector string:

```rust
use peacock_crest::{SourceInfo, SelectorNode};

let selector = "div > .example";
let source_info = SourceInfo::new(selector);
let parsed_selector = SelectorNode::from_source(source_info).expect("Failed to read css");

println!("Parsed selector: {}", parsed_selector);
```

### Stylesheet Parsing

You can also parse full CSS stylesheets:

```rust
use peacock_crest::{SourceInfo, Stylesheet};

let css = "div { color: red; } .example { font-size: 16px; }";
let source_info = SourceInfo::new(css);
let stylesheet = Stylesheet::from_source(source_info).expect("Failed to read css");

println!("Parsed stylesheet:\n{}", stylesheet);
```

### Selector Matching

To match a selector against a custom element, implement the `DomElement` trait for your type:

```rust
// todo
```

## Testing and Validation

Crest currently uses the following to validate Crest's functionality for historical reference and to
ensure compatibility with a range of CSS practices:
- [X] acid1
- [X] acid2
- [ ] ~~bootstrap 1~~ (bootstrap2 relies on non-compliance with the standard and as such has been skipped in tests for historical compliance)
- [ ] ~~bootstrap 2~~ (bootstrap2 relies on non-compliance with the standard and as such has been skipped in tests for historical compliance)
- [ ] ~~bootstrap 3~~ (bootstrap2 relies on non-compliance with the standard and as such has been skipped in tests for historical compliance)
- [ ] bootstrap 4
- [ ] bootstrap 5

To run them, use:

```bash
cargo test
```

## Benchmarks

Performance benchmarks are available in the `benches` directory. To run them, use:

```bash
cargo bench
```

Current Results:

<img src="assets/benchmarks.png" alt="performance benchmarks between crest and other css parsers" />
