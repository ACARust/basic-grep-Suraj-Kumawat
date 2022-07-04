use anyhow::Error;

/// While using `&[&str]` to handle flags is convenient for task purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this task.
///
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html

#[derive(Debug)]
pub struct Flags;

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        unimplemented!(
            "Given the flags {:?} implement your own 'Flags' struct to handle flags-related logic",
            flags
        );
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    unimplemented!(
        "Search the files '{:?}' for '{}' pattern and save the matches in a vector. Your search logic should be aware of the given flags '{:?}'",
        files,
        pattern,
        flags
    );
}
