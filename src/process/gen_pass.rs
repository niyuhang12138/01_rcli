use rand::seq::SliceRandom;

const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBER: &[u8] = b"0123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(length: u8, uppercase: bool, lowercase: bool, number: bool, symbol: bool) {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

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

    for _ in 0..length {
        let c = chars
            .choose(&mut rng)
            .expect("charts won't be empty in this context");
        password.push(*c);
    }

    // TODO: make sure the password has at least one of each type
    password.shuffle(&mut rng);
    println!("{}", String::from_utf8_lossy(&password));
}
