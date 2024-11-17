use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*()-_=+[]{}|;:,.<>?/~";

pub fn process_genpass(length: u8, upper: bool, lower: bool, number: bool, symbol: bool) -> anyhow::Result<String> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if upper {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng)
            .expect("UPPER won't be empty in this context.") as char);
    }
    if lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng)
            .expect("LOWER won't be empty in this context.") as char);
    }
    if number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng)
            .expect("NUMBER won't be empty in this context.") as char);
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng)
            .expect("SYMBOL won't be empty in this context.") as char);
    }

    for _ in 0..(length - password.len() as u8) {
        let c = chars.choose(&mut rng)
            .expect("chars won't be empty in this context.");
        password.push(*c as char);
    }

    password.shuffle(&mut rng);
    
    let password: String = password.iter().collect();

    // output password strength in stderr
    let estimate = zxcvbn(&password, &[]);

    match estimate.score() {
        zxcvbn::Score::Zero => eprintln!("Password strength: Very weak"),
        zxcvbn::Score::One => eprintln!("Password strength: Weak"),
        zxcvbn::Score::Two => eprintln!("Password strength: Medium"),
        zxcvbn::Score::Three => eprintln!("Password strength: Strong"),
        zxcvbn::Score::Four => eprintln!("Password strength: Very strong"),
        _ => eprintln!("Password strength: Unknown"),
    }
    
    Ok(password)
}

