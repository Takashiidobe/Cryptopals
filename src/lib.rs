use anyhow::Result;

#[allow(deprecated)]
pub fn hex_to_base64(input: &[u8]) -> Result<String> {
    use base64::encode;
    use hex::decode;

    let bytes = decode(input)?;
    let output = encode(bytes);

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        assert_eq!(
            hex_to_base64(input.as_bytes()).unwrap(),
            String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t")
        );
    }
}
