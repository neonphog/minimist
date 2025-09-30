#![deny(missing_docs)]
//! Transparent, ergonomic, no-dependencies arg processing.
//!
//! For when clap feels like too much.
//!
//! Inspired-by, but completely unaffiliated with a library in another language
//! that may or may not have a similar name.
//!
//! [See a more complete example in the
//! repository](https://github.com/neonphog/minimist/tree/main/examples).
//!
//! # Examples
//!
//! ```
//! use minimist::Minimist;
//!
//! let args = Minimist::parse(
//!     // don't forget to skip the cmd
//!     std::env::args_os().skip(1)
//! );
//!
//! assert!(!args.as_flag("no-args-in-docs"));
//! ```
//!
//! ## Flags
//!
//! ```
//! # use minimist::Minimist;
//! let args = Minimist::parse(["-ab", "--c"]);
//!
//! assert!(args.as_flag("a"));
//! assert!(args.as_flag("b"));
//! assert!(args.as_flag("c"));
//! assert!(!args.as_flag("d"));
//! ```
//!
//! ## Paths
//!
//! ```
//! # use minimist::Minimist;
//! let args = Minimist::parse(["--file", "pretend/this/is/not/utf8"]);
//!
//! assert_eq!(
//!     &std::path::PathBuf::from("pretend/this/is/not/utf8"),
//!     args.as_one_path("file").unwrap(),
//! );
//! ```
//!
//! ## Path lists
//!
//! ```
//! # use minimist::Minimist;
//! let args = Minimist::parse(["-f", "/f1", "-f", "/f2"]);
//!
//! assert_eq!(
//!     vec![
//!         std::path::PathBuf::from("/f1"),
//!         std::path::PathBuf::from("/f2")
//!     ],
//!     args.as_list_path("f").unwrap().collect::<Vec<_>>(),
//! );
//! ```
//!
//! ## Strings
//!
//! ```
//! # use minimist::Minimist;
//! let args = Minimist::parse(["--fruit", "banana"]);
//!
//! assert_eq!("banana", args.to_one_str("fruit").unwrap());
//! ```
//!
//! ## Positionals
//!
//! ```
//! # use minimist::Minimist;
//! let args = Minimist::parse(["one", "-f", "bob", "two"]);
//!
//! assert_eq!(
//!     vec!["one", "two"],
//!     args.to_list_str(Minimist::POS).unwrap().collect::<Vec<_>>(),
//! );
//! ```
//!
//! ## Pass-through
//!
//! ```
//! # use minimist::Minimist;
//! let args = Minimist::parse(["--t1", "1", "--", "--t2", "2"]);
//!
//! assert_eq!(
//!     vec!["--t2", "2"],
//!     args.to_list_str(Minimist::PASS).unwrap().collect::<Vec<_>>(),
//! );
//! ```
//!
//! ## Clear debugging
//!
//! ```
//! # use minimist::Minimist;
//! let args = Minimist::parse(
//!     ["pos", "-f", "-p", "/", "-s", "hello", "--", "rest"]
//! );
//!
//! assert_eq!(
//!     r#"Minimist({"-": ["pos"], "--": ["rest"], "f": [], "p": ["/"], "s": ["hello"]})"#,
//!     format!("{args:?}"),
//! );
//! ```
//!
//! ## Set defaults after parsing
//!
//! ```
//! # use minimist::Minimist;
//! let mut args = Minimist::parse(["--nope"]);
//!
//! args.set_default("hello", "world");
//!
//! assert_eq!("world", args.to_one_str("hello").unwrap());
//! ```
//!
//! ## Set env defaults after parsing
//!
//! ```
//! # use minimist::Minimist;
//! unsafe { std::env::set_var("MIN_TEST", "apple") }
//!
//! let mut args = Minimist::parse(["--nope"]);
//!
//! args.set_default_env("test", "MIN_TEST");
//!
//! assert_eq!("apple", args.to_one_str("test").unwrap());
//! ```
//!
//! ## Alias flags after parsing
//!
//! ```
//! # use minimist::Minimist;
//! let mut args = Minimist::parse(["-h"]);
//!
//! if args.as_flag("h") {
//!     args.entry("help".into()).or_default();
//! }
//!
//! assert!(args.as_flag("help"));
//! ```
//!
//! ## Alias values after parsing
//!
//! ```
//! # use minimist::Minimist;
//! let mut args = Minimist::parse(["-f", "banana", "--fruit", "apple"]);
//!
//! let mut f = args.entry("f".into()).or_default().clone();
//! args.entry("fruit".into()).or_default().append(&mut f);
//!
//! assert_eq!(
//!     vec!["apple", "banana"],
//!     args.to_list_str("fruit").unwrap().collect::<Vec<_>>(),
//! );
//! ```

use std::collections::BTreeMap;
use std::ffi::OsString;

/// Transparent, ergonomic, no-dependencies arg processing.
///
/// See [module-level-docs](index.html) for details.
#[derive(Default, Debug)]
pub struct Minimist(pub BTreeMap<String, Vec<OsString>>);

impl std::ops::Deref for Minimist {
    type Target = BTreeMap<String, Vec<OsString>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Minimist {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Minimist {
    /// The arg key for positional items.
    pub const POS: &'static str = "-";

    /// The arg key for remaining pass-through items after `--`.
    pub const PASS: &'static str = "--";

    /// Parse an argument iterator.
    ///
    /// ```
    /// # use minimist::Minimist;
    /// let _args = Minimist::parse(std::env::args_os().skip(1));
    /// ```
    pub fn parse<S, I>(arg_list: I) -> Self
    where
        S: Into<OsString>,
        I: IntoIterator<Item = S>,
    {
        let mut map: BTreeMap<String, Vec<OsString>> = BTreeMap::new();

        let mut last_flag = None;

        let mut iter = arg_list.into_iter();

        for tok in Tok::parse(&mut iter) {
            match tok {
                Tok::Key(k) => {
                    last_flag = Some(k.clone());
                    map.entry(k).or_default();
                }
                Tok::Val(v) => {
                    if let Some(f) = last_flag.take() {
                        map.entry(f).or_default().push(v);
                    } else {
                        map.entry(Self::POS.into()).or_default().push(v);
                    }
                }
            }
        }

        for v in iter {
            map.entry(Self::PASS.into()).or_default().push(v.into());
        }

        Self(map)
    }

    /// Get an arg as a flag (bool).
    ///
    /// ```
    /// # use minimist::Minimist;
    /// Minimist::parse(["-f"]).as_flag("f");
    /// ```
    pub fn as_flag(&self, arg: &str) -> bool {
        self.as_list(arg).is_some()
    }

    /// Get an arg as a single (first) value.
    ///
    /// ```
    /// # use minimist::Minimist;
    /// Minimist::parse(["-o", "1"]).as_one("o");
    /// ```
    pub fn as_one(&self, arg: &str) -> Option<&OsString> {
        self.as_list(arg).and_then(|mut l| l.next())
    }

    /// Get an arg as a single (first) path value.
    ///
    /// ```
    /// # use minimist::Minimist;
    /// Minimist::parse(["-p", "/"]).as_one_path("p");
    /// ```
    pub fn as_one_path(&self, arg: &str) -> Option<&std::path::Path> {
        self.as_one(arg).map(|p| p.as_os_str().as_ref())
    }

    /// Get an arg as a list of values.
    ///
    /// ```
    /// # use minimist::Minimist;
    /// Minimist::parse(["-a", "1", "-a", "2"]).as_list("a");
    /// ```
    pub fn as_list(
        &self,
        arg: &str,
    ) -> Option<impl Iterator<Item = &OsString>> {
        self.get(arg).map(|l| l.iter())
    }

    /// Get an arg as a list of path values.
    ///
    /// ```
    /// # use minimist::Minimist;
    /// Minimist::parse(["-p", "/a", "-p", "/b"]).as_list_path("p");
    /// ```
    pub fn as_list_path(
        &self,
        arg: &str,
    ) -> Option<impl Iterator<Item = &std::path::Path>> {
        self.as_list(arg).map(|i| i.map(|p| p.as_os_str().as_ref()))
    }

    /// Get an arg as a single (first) lossy string.
    ///
    /// ```
    /// # use minimist::Minimist;
    /// Minimist::parse(["-s", "hello"]).to_one_str("s");
    /// ```
    pub fn to_one_str(&self, arg: &str) -> Option<std::borrow::Cow<'_, str>> {
        self.as_one(arg).map(|s| s.to_string_lossy())
    }

    /// Get an arg as a list of lossy string values.
    ///
    /// ```
    /// # use minimist::Minimist;
    /// Minimist::parse(["-s", "1", "-s", "2"]).to_list_str("s");
    /// ```
    pub fn to_list_str(
        &self,
        arg: &str,
    ) -> Option<impl Iterator<Item = std::borrow::Cow<'_, str>>> {
        self.as_list(arg).map(|i| i.map(|s| s.to_string_lossy()))
    }

    /// Sets a default value to an arg, only if a value doesn't already exist.
    pub fn set_default(
        &mut self,
        arg: impl std::fmt::Display,
        val: impl Into<OsString>,
    ) {
        let arg = self.0.entry(arg.to_string()).or_default();
        if arg.is_empty() {
            arg.push(val.into());
        }
    }

    /// Sets a default value to an arg based on an environment variable, only
    /// if a value for the arg doesn't already exist, and the env var does.
    pub fn set_default_env(
        &mut self,
        arg: impl std::fmt::Display,
        env: impl AsRef<std::ffi::OsStr>,
    ) {
        if let Some(env) = std::env::var_os(env) {
            self.set_default(arg, env);
        }
    }
}

enum Tok {
    Key(String),
    Val(OsString),
}

impl Tok {
    fn parse<S, I>(arg_iter: &mut I) -> Vec<Tok>
    where
        S: Into<OsString>,
        I: Iterator<Item = S>,
    {
        let mut out = Vec::new();

        for arg in arg_iter {
            let arg = arg.into();
            let as_str = arg.to_string_lossy();

            if as_str == "--" {
                return out;
            }

            if as_str.len() < 2 || !as_str.starts_with('-') {
                if !arg.is_empty() {
                    out.push(Tok::Val(arg));
                }
                continue;
            }

            if as_str.starts_with("--") {
                out.push(Tok::Key(as_str.trim_start_matches("--").to_string()));
                continue;
            }

            for c in as_str.trim_start_matches('-').chars() {
                out.push(Tok::Key(c.to_string()));
            }
        }

        out
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const FIX: &[(&str, &[&str], &str)] = &[
        (
            "single dashes",
            &["-", "-ab", "-", "--c", "-", "-", "--", "-"],
            r#"Minimist({"-": ["-", "-"], "--": ["-"], "a": [], "b": ["-"], "c": ["-"]})"#,
        ),
        (
            "sane repeats",
            &["-s", "1", "-s", "-s", "-s", "2", "-s"],
            r#"Minimist({"s": ["1", "2"]})"#,
        ),
        (
            "newlines ( not sure this is a good thing :/ )",
            &["--a\nb", "a\nb"],
            r#"Minimist({"a\nb": ["a\nb"]})"#,
        ),
        (
            "whitespace",
            &["-a", " ", "-b", "\t"],
            r#"Minimist({"a": [" "], "b": ["\t"]})"#,
        ),
    ];

    #[test]
    fn fixtures() {
        for (name, input, output) in FIX.iter() {
            let result = format!("{:?}", Minimist::parse(input.iter()));
            assert_eq!(*output, result, "fixture({name})");
        }
    }
}
