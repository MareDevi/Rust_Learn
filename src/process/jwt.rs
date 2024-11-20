//rcli jwt sign -- sub acme -- aud device1 -- exp 14d
//rcli jwt verify -t <token-value>
use jsonwebtoken::{encode, Algorithm, Header, EncodingKey, DecodingKey, Validation, decode};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims {
    sub: String,
    aud: String,
    exp: usize,
}

pub fn process_jwt_sign(sub: String, aud: String, exp: String) -> anyhow::Result<String> {
    let duration = match exp.chars().next_back().unwrap() {
        'd' => exp[..exp.len()-1].parse::<u64>().unwrap() * 24 * 60 * 60,
        'h' => exp[..exp.len()-1].parse::<u64>().unwrap() * 60 * 60,
        'm' => exp[..exp.len()-1].parse::<u64>().unwrap() * 60,
        's' => exp[..exp.len()-1].parse::<u64>().unwrap(),
        _ => exp.parse::<u64>().unwrap(),
    };
    let exp = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + duration) as usize;
    let claims = Claims {
        sub,
        aud,
        exp,
    };
    println!("{:?}", claims);
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())).unwrap();
    Ok(token)
}

pub fn process_jwt_verify(token: String) -> anyhow::Result<()> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&["device1"]);
    let token_data = decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &validation)?;
    println!("{:?}", token_data.claims);
    Ok(())
}