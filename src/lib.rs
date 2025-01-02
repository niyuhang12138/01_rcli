mod cli;
mod process;
mod utils;

pub use cli::{Base64SubCommand, HttpSubCommand, Opts, SubCommand, TextSignFormat, TextSubCommand};
pub use process::{
    process_csv, process_decode, process_encode, process_genpass, process_http_serve,
    process_text_generate, process_text_sign, process_text_verify,
};
pub use utils::*;

#[cfg(test)]
mod tests {
    #[test]
    fn test_fn() {}
}
