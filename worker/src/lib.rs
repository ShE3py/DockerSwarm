use md5::Digest;

/// Tente de retrouver un texte alphanumérique correspondant à un condensat MD5.
///
/// # Exemples
///
/// ```
/// use worker::md5break;
///
/// assert_eq!(md5break("d41d8cd98f00b204e9800998ecf8427e"), "");
/// assert_eq!(md5break("81dc9bdb52d04dc20036dbd8313ed055"), "1234");
/// assert_eq!(md5break("4a7d1ed414474e4033ac29ccb8653d9b"), "0000");
/// assert_eq!(md5break("cad77c7dffc10fcacc77ff0690f2897a"), "pina");
/// ```
#[must_use]
pub fn md5break(md5: &str) -> String {
    #![expect(clippy::almost_complete_range)]
    
    let mut digest = Digest([0; 16]);
    hex::decode_to_slice(md5, &mut digest.0).unwrap();
    
    let mut buf = Vec::with_capacity(8);
    
    loop {
        for c in &mut buf {
            match *c {
                b'0'..b'9' | b'A'..b'Z' | b'a'..b'z' => *c += 1,
                b'9' => *c = b'A',
                b'Z' => *c = b'a',
                b'z' => { *c = b'0'; continue },
                _ => unreachable!(),
            }
            
            break
        }
        
        if md5::compute(&buf) == digest {
            return String::from_utf8(buf).expect("entered unreachable code");
        }
        
        if buf.iter().all(|c| *c == b'z') {
            buf.push(b'0');
            buf.fill(b'0');
            
            if buf.len() > 255 {
                panic!();
            }
            
            if md5::compute(&buf) == digest {
                return String::from_utf8(buf).expect("entered unreachable code");
            }
        }
    }
}
