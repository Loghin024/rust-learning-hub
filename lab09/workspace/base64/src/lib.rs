// base64/src/lib.rs

/// Encode a slice of bytes into a base64-encoded string.
///
/// # Examples
///
/// ```
/// use base64::encode;
///
/// let input = b"hello world";
/// let encoded = encode(input);
/// assert_eq!(encoded, "aGVsbG8gd29ybGQ=");
/// ```
pub fn encode(input: &[u8]) -> String {
    const BASE64_CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut result = String::new();
    let mut i = 0;

    while i < input.len() {
        let mut chunk = [0; 3];
        let mut chunk_size = 0;

        // Fill the chunk with up to 3 bytes from the input
        while chunk_size < 3 && i < input.len() {
            chunk[chunk_size] = input[i];
            chunk_size += 1;
            i += 1;
        }

        // Encode the chunk
        result.push(BASE64_CHARS[(chunk[0] >> 2) as usize] as char);
        result.push(BASE64_CHARS[((chunk[0] & 0b0000_0011) << 4 | (chunk[1] >> 4)) as usize] as char);

        match chunk_size {
            1 => {
                result.push('=');
                result.push('=');
            }
            2 => {
                result.push(BASE64_CHARS[((chunk[1] & 0b0000_1111) << 2 | (chunk[2] >> 6)) as usize] as char);
                result.push('=');
            }
            3 => {
                result.push(BASE64_CHARS[((chunk[1] & 0b0000_1111) << 2 | (chunk[2] >> 6)) as usize] as char);
                result.push(BASE64_CHARS[(chunk[2] & 0b0011_1111) as usize] as char);
            }
            _ => {}
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_no_padding() {
        assert_eq!(encode(b"hello world"), "aGVsbG8gd29ybGQ=");
    }

    #[test]
    fn test_encode_padding_half() {
        assert_eq!(encode(b"hello"), "aGVsbG8=");
    }

    #[test]
    fn test_encode_padding_one() {
        assert_eq!(encode(b"hello!"), "aGVsbG8h");
    }

}
