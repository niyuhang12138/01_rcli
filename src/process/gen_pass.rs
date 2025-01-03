//! generate a random password
use anyhow::Result;
use rand::seq::SliceRandom;

const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBER: &[u8] = b"0123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

/// generate a random password
pub fn process_genpass(
    length: u8,
    uppercase: bool,
    lowercase: bool,
    number: bool,
    symbol: bool,
) -> Result<String> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    // the following conditional judgments guarantee that there will be at least one, which guarantees randomly generated uncertainty

    if uppercase {
        password.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty"));
        chars.extend_from_slice(UPPER);
    }

    if lowercase {
        password.push(*LOWER.choose(&mut rng).expect("LOWER won't be empty"));
        chars.extend_from_slice(LOWER);
    }

    if number {
        password.push(*NUMBER.choose(&mut rng).expect("NUMBER won't be empty"));
        chars.extend_from_slice(NUMBER);
    }

    if symbol {
        password.push(*SYMBOL.choose(&mut rng).expect("SYMBOL won't be empty"));
        chars.extend_from_slice(SYMBOL);
    }

    // generate the rest of the password
    for _ in 0..length {
        let c = chars
            .choose(&mut rng)
            .expect("charts won't be empty in this context");
        password.push(*c);
    }

    // disrupt the order
    password.shuffle(&mut rng);

    // convert to string
    let password = String::from_utf8(password)?;

    Ok(password)
}
