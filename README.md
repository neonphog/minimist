# minimist

![Crates.io License](https://img.shields.io/crates/l/minimist)
[![Crates.io Version](https://img.shields.io/crates/v/minimist)](https://crates.io/crates/minimist)

<!-- cargo-rdme start -->

Transparent, ergonomic, no-dependencies arg processing.

For when clap feels like too much.

Inspired-by, but completely unaffiliated with a library in another language
that may or may not have a similar name.

## Examples

```rust
use minimist::Minimist;

let args = Minimist::parse(
    // don't forget to skip the cmd
    std::env::args_os().skip(1)
);

assert!(!args.as_flag("no-args-in-docs"));
```

### Flags

```rust
let args = Minimist::parse(["-ab", "--c"]);

assert!(args.as_flag("a"));
assert!(args.as_flag("b"));
assert!(args.as_flag("c"));
assert!(!args.as_flag("d"));
```

### Paths

```rust
let args = Minimist::parse(["--file", "pretend/this/is/not/utf8"]);

assert_eq!(
    &std::path::PathBuf::from("pretend/this/is/not/utf8"),
    args.as_one_path("file").unwrap(),
);
```

### Path lists

```rust
let args = Minimist::parse(["-f", "/f1", "-f", "/f2"]);

assert_eq!(
    vec![
        std::path::PathBuf::from("/f1"),
        std::path::PathBuf::from("/f2")
    ],
    args.as_list_path("f").unwrap().collect::<Vec<_>>(),
);
```

### Strings

```rust
let args = Minimist::parse(["--fruit", "banana"]);

assert_eq!("banana", args.to_one_str("fruit").unwrap());
```

### Positionals

```rust
let args = Minimist::parse(["one", "-f", "bob", "two"]);

assert_eq!(
    vec!["one", "two"],
    args.to_list_str("_").unwrap().collect::<Vec<_>>(),
);
```

### Pass-through

```rust
let args = Minimist::parse(["--t1", "1", "--", "--t2", "2"]);

assert_eq!(
    vec!["--t2", "2"],
    args.to_list_str("--").unwrap().collect::<Vec<_>>(),
);
```

### Clear debugging

```rust
let args = Minimist::parse(
    ["pos", "-f", "-p", "/", "-s", "hello", "--", "rest"]
);

assert_eq!(
    r#"Minimist({"--": ["rest"], "_": ["pos"], "f": [], "p": ["/"], "s": ["hello"]})"#,
    format!("{args:?}"),
);
```

### Set defaults after parsing

```rust
let mut args = Minimist::parse(["--nope"]);

args.entry("hello".into()).or_insert(vec!["world".into()]);

assert_eq!("world", args.to_one_str("hello").unwrap());
```

### Alias flags after parsing

```rust
let mut args = Minimist::parse(["-h"]);

if args.as_flag("h") {
    args.entry("help".into()).or_default();
}

assert!(args.as_flag("help"));
```

### Alias one's and lists after parsing

```rust
let mut args = Minimist::parse(["-f", "banana", "--fruit", "apple"]);

let mut f = args.entry("f".into()).or_default().clone();
args.entry("fruit".into()).or_default().append(&mut f);

assert_eq!(
    vec!["apple", "banana"],
    args.to_list_str("fruit").unwrap().collect::<Vec<_>>(),
);
```

<!-- cargo-rdme end -->
