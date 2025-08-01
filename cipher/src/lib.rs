#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let solution: String = original
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let offset = (c as u8) - base;
                (base + (25 - offset)) as char
            } else {
                c
            }
        })
        .collect();

    if &ciphered == &solution {
        Ok(())
    } else {
        Err(CipherError {
            expected: solution,
        })
    }
}
