use hex::FromHexError;
use md5::Digest;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

const MAX_LEN: usize = 5;

/// Tente de retrouver un texte alphanumérique (`A-Za-z0-9`) correspondant à un condensat MD5.
///
/// # Exemples
///
/// ```
/// use worker::md5break;
///
/// assert_eq!(md5break("d41d8cd98f00b204e9800998ecf8427e").unwrap(), "");
/// assert_eq!(md5break("81dc9bdb52d04dc20036dbd8313ed055").unwrap(), "1234");
/// assert_eq!(md5break("4a7d1ed414474e4033ac29ccb8653d9b").unwrap(), "0000");
/// assert_eq!(md5break("cad77c7dffc10fcacc77ff0690f2897a").unwrap(), "pina");
/// ```
pub fn md5break(md5: &str) -> Result<String, BreakError> {
    // Convert the md5 hex into a byte array.
    let mut digest = Digest([0; 16]);
    hex::decode_to_slice(md5, &mut digest.0)?;

    // List all possible words.
    let mut buf = Vec::with_capacity(MAX_LEN);
    loop {
        // Increase the leftmost character
        for c in &mut buf {
            // ASCII table:
            // 0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz
            // ----------       --------------------------      --------------------------

            // At the end, reset to `0` and increase the next leftmost character
            if *c == b'z' {
                *c = b'0';
                continue;
            }

            match *c {
                // +1 on contiguous ASCII
                #[expect(clippy::almost_complete_range)]
                b'0'..b'9' | b'A'..b'Z' | b'a'..b'z' => *c += 1,

                // Direct jump on non-contiguous ASCII
                b'9' => *c = b'A',
                b'Z' => *c = b'a',

                _ => unreachable!(),
            }

            break
        }

        // Check the word
        if md5::compute(&buf) == digest {
            return Ok(String::from_utf8(buf).unwrap() /* it should be ASCII */);
        }

        // If all characters are `z`, add a character and reset to `0`s
        if buf.iter().all(|c| *c == b'z') {
            buf.push(b'0');
            buf.fill(b'0');

            // Avoid infinite loops
            if buf.len() > MAX_LEN {
                return Err(BreakError::NoResult);
            }

            // Try the `0`s word
            if md5::compute(&buf) == digest {
                return Ok("0".repeat(buf.len()));
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BreakError {
    /// The `md5` is malformed (not hex, too many digits)
    BadInput(FromHexError),

    /// Timed out after trying up to [`MAX_LEN`] digits.
    NoResult,
}

impl From<FromHexError> for BreakError {
    fn from(value: FromHexError) -> Self {
        BreakError::BadInput(value)
    }
}

impl Display for BreakError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BreakError::BadInput(e) => write!(f, "bad input: {e}"),
            BreakError::NoResult => write!(f, "no result (tried up to {MAX_LEN} characters)"),
        }
    }
}

impl Error for BreakError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            BreakError::BadInput(e) => Some(e),
            BreakError::NoResult => None,
        }
    }
}
