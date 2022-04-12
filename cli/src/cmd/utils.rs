/// Parse hex string to a vector of bytes.
pub fn parse_hex(hex: &str) -> eyre::Result<Vec<u8>> {
    let s = if hex.starts_with("0x") {
        &hex.as_bytes()[2..]
    } else {
        hex.as_bytes()
    };

    Ok(hex::decode(s)?)
}

/// A wrapper struct to overcome structopt's `Vec` special handling.
#[derive(Debug)]
pub struct Bytes(pub Vec<u8>);
impl std::str::FromStr for Bytes {
    type Err = eyre::Error;

    fn from_str(s: &str) -> eyre::Result<Self> {
        parse_hex(s).map(Bytes)
    }
}
