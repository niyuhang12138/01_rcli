use std::io::Read;

use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

use crate::{cli::Base64Format, get_reader};

pub fn process_encode(input: &str, format: Base64Format) -> Result<String> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encode = match format {
        Base64Format::Standard => STANDARD.encode(buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buf),
    };
    Ok(encode)
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();

    let decode = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };

    // TODO: decode data might not be string (but for this example, we assume it is)
    // let decode = String::from_utf8(decode)?;
    Ok(decode)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn test_process_encode() -> Result<()> {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_encode(input, format).is_ok());
        Ok(())
    }

    #[test]
    fn test_process_decode() -> Result<()> {
        let input = "fixtures/b64.txt";
        let format = Base64Format::Standard;
        assert!(process_decode(input, format).is_ok());
        Ok(())
    }
}
