mod opts;
mod process;

pub use opts::{Opts, SubCommand};
pub use process::{process_csv, Player};

#[cfg(test)]
mod tests {
    #[test]
    fn test_fn() {}
}