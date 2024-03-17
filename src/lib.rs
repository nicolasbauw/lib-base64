//! A base64 (with padding) encoding and decoding library, which implements the encode() and decode() methods for the String type.

use std::{error::Error, fmt, str};

const TABLE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Base64Error {
    /// Invalid input data length (must be a multiple of 4)
    InvalidDataLenght,
    /// Incorrectly encoded input data
    InvalidBase64Data,
    /// Encoding error
    EncodingError,
}

impl Error for Base64Error {}

impl fmt::Display for Base64Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Base64 error : ")?;
        f.write_str(match self {
            Base64Error::InvalidDataLenght => "Invalid input data length",
            Base64Error::InvalidBase64Data => "Invalid base64 data",
            Base64Error::EncodingError => "Cannot encode input data",
        })
    }
}

impl From<std::string::FromUtf8Error> for Base64Error {
    fn from(_e: std::string::FromUtf8Error) -> Base64Error {
        Base64Error::InvalidBase64Data
    }
}

impl From<std::str::Utf8Error> for Base64Error {
    fn from(_e: std::str::Utf8Error) -> Base64Error {
        Base64Error::EncodingError
    }
}

impl From<std::num::ParseIntError> for Base64Error {
    fn from(_e: std::num::ParseIntError) -> Base64Error {
        Base64Error::EncodingError
    }
}

impl From<Box<dyn Error>> for Base64Error {
    fn from(_e: Box<dyn Error>) -> Base64Error {
        Base64Error::EncodingError
    }
}

pub trait Base64 {
    fn encode(&self) -> Result<String, Base64Error>;
    fn decode(&self) -> Result<String, Base64Error>;
}

impl Base64 for String {
    /// Encodes a String with the base64 scheme
    ///
    /// Example:
    /// ```
    /// use lib_base64::Base64;
    /// let s = String::from("Test");
    /// assert_eq!(Ok(String::from("VGVzdA==")), s.encode())
    /// ```
    fn encode(&self) -> Result<String, Base64Error> {
        let a = self.as_bytes();

        let mut octal = String::new();
        let mut i = 0;

        // The number of full sextets to process
        let inputlenmod = a.len() % 3;
        let blockstoprocess = if inputlenmod == 0 {
            a.len()
        } else {
            a.len() - inputlenmod
        };
        let padding = if inputlenmod != 0 {
            3 - (a.len() - blockstoprocess)
        } else {
            0
        };

        // Creating octal output from bytes converted to sextets (3 * 8 bytes = 24 bits = four sextets)
        while i < blockstoprocess {
            octal.push_str(
                format!("{:o}", u32::from_be_bytes([0, a[i], a[i + 1], a[i + 2]])).as_str(),
            );
            i += 3;
        }

        match padding {
            1 => {
                octal
                    .push_str(format!("{:o}", u32::from_be_bytes([0, a[i], a[i + 1], 0])).as_str());
            }
            2 => {
                octal.push_str(format!("{:o}", u32::from_be_bytes([0, a[i], 0, 0])).as_str());
            }
            _ => {}
        };

        // Converting octal output to a decimal index vector
        let sextets = octal
            .as_bytes()
            .chunks(2)
            .map(str::from_utf8)
            .map(|u| {
                u.map_err::<Box<dyn Error>, _>(|e| e.into())
                    .and_then(|u| usize::from_str_radix(u, 8).map_err(|e| e.into()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mut result = String::new();

        for i in 0..(sextets.len() - padding) {
            result.push_str(&TABLE[sextets[i]..(sextets[i] + 1)]);
        }
        match padding {
            1 => result.push('='),
            2 => result.push_str("=="),
            _ => {}
        };
        Ok(result)
    }

    /// Decodes a String encoded with the base64 scheme
    ///
    /// Example:
    /// ```
    /// use lib_base64::Base64;
    /// let s = String::from("VGVzdA==");
    /// assert_eq!(Ok(String::from("Test")), s.decode())
    /// ```
    fn decode(&self) -> Result<String, Base64Error> {
        let mut encoded_data = self.to_owned();
        let padding = encoded_data.matches('=').count();

        if encoded_data.len() % 4 != 0 {
            return Err(Base64Error::InvalidDataLenght);
        };

        // replaces padding characters by characters decoded as zero
        for _ in 0..padding {
            encoded_data.pop();
        }

        for _ in 0..padding {
            encoded_data.push('A');
        }

        // Retrieves octal indexes of encoded characters
        let octal = encoded_data
            .chars()
            .map(|c| format!("{:02o}", TABLE.find(c).unwrap_or(65))) // 65 for invalid base64 input detection
            .collect::<Vec<String>>();

        // Gathers the 4 sextets (24 bits) collection
        let mut octalsextets = Vec::new();
        let mut n = 0;
        while n < encoded_data.len() {
            let mut s = String::new();
            for i in 0..4 {
                if octal[n + i] == "101" {
                    return Err(Base64Error::InvalidBase64Data);
                } // 65 decimal = 101 octal
                s.push_str(octal[n + i].as_str());
            }
            n += 4;
            octalsextets.push(s);
        }

        // Decodes the 4 sextets to 3 bytes
        let decimal = octalsextets
            .iter()
            .map(|s| usize::from_str_radix(s, 8))
            .collect::<Result<Vec<_>, _>>()?;

        // Extracts the significants bytes of the usize to a vector of bytes
        let mut bytes: Vec<u8> = Vec::new();
        for i in 0..decimal.len() {
            let a = decimal[i].to_be_bytes();
            bytes.push(a[5]);
            bytes.push(a[6]);
            bytes.push(a[7]);
        }

        // Removes padding bytes inserted for decoding
        for _ in 0..padding {
            bytes.pop();
        }

        let result = String::from_utf8(bytes)?;
        Ok(result)
    }
}

impl Base64 for Vec<u8> {
    /// Encodes a Vec<u8> with the base64 scheme
    ///
    /// Example:
    /// ```
    /// use lib_base64::Base64;
    /// let s = String::from("Test");
    /// assert_eq!(Ok(String::from("VGVzdA==")), s.encode())
    /// ```
    fn encode(&self) -> Result<String, Base64Error> {
        let a = self;
        let table = TABLE.as_bytes();

        let mut input_buffer = Vec::new();
        let mut i = 0;

        // The number of full sextets to process
        let inputlenmod = a.len() % 3;
        let blockstoprocess = if inputlenmod == 0 {
            a.len()
        } else {
            a.len() - inputlenmod
        };

        let padding = if inputlenmod != 0 {
            3 - (a.len() - blockstoprocess)
        } else {
            0
        };

        let mut base64_buffer: Vec<u8> = Vec::new();

        // Creating octal output from bytes converted to sextets (3 * 8 bytes = 24 bits = four sextets)
        // step 1 : put 3 bytes (24 bits) in a 32-bit word
        while i < blockstoprocess {
            input_buffer.push(u32::from_be_bytes([0, a[i], a[i + 1], a[i + 2]]));
            i += 3;
        }

        match padding {
            1 => {
                input_buffer.push(u32::from_be_bytes([0, a[i], a[i + 1], 0]));
            }
            2 => {
                input_buffer.push(u32::from_be_bytes([0, a[i], 0, 0]));
            }
            _ => {}
        };

        // step 2 : octal conversion (three bytes = four sextets)
        i = 0;
        while i < input_buffer.len() {
            let t0 = ((input_buffer[i] & 0xFC0000) >> 18) as u8;
            let t1 = ((input_buffer[i] & 0x3F000) >> 12) as u8;
            let t2 = ((input_buffer[i] & 0xFC0) >> 6) as u8;
            let t3 = (input_buffer[i] & 0x3F) as u8;
            base64_buffer.push(table[t0 as usize]);
            base64_buffer.push(table[t1 as usize]);
            base64_buffer.push(table[t2 as usize]);
            base64_buffer.push(table[t3 as usize]);
            i = i + 1;
        }

        let mut result = String::from_utf8(base64_buffer)?;
        match padding {
            1 => {
                result.pop();
                result.push('=');
            }
            2 => {
                result.pop();
                result.pop();
                result.push_str("==");
            }
            _ => {}
        };

        Ok(result)
    }

    fn decode(&self) -> Result<String, Base64Error> {
        Ok(String::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::Base64;
    #[test]
    fn encode_works() {
        assert_eq!(
            Ok(String::from("SmUgdCdhaW1lIG1hIGNow6lyaWU=")),
            String::from("Je t'aime ma chérie").encode()
        );
    }

    #[test]
    fn encode_no_padding() {
        assert_eq!(Ok(String::from("TWFu")), String::from("Man").encode());
    }

    #[test]
    fn encode_one_padding() {
        assert_eq!(Ok(String::from("TWE=")), String::from("Ma").encode());
    }

    #[test]
    fn encode_two_padding() {
        assert_eq!(Ok(String::from("TQ==")), String::from("M").encode());
    }

    #[test]
    fn decode_works() {
        assert_eq!(
            Ok(String::from("Joyeux anniversaire !")),
            String::from("Sm95ZXV4IGFubml2ZXJzYWlyZSAh").decode()
        );
    }

    #[test]
    fn datalength_check() {
        assert_eq!(
            Err(crate::Base64Error::InvalidDataLenght),
            String::from("TWF").decode()
        );
    }

    #[test]
    fn validb64data_check() {
        assert_eq!(
            Err(crate::Base64Error::InvalidBase64Data),
            String::from("TWF$").decode()
        );
    }

    #[test]
    fn encode_u8_no_padding() {
        let input: Vec<u8> = vec![0x4d, 0x61, 0x6e];
        assert_eq!(Ok(String::from("TWFu")), input.encode());
    }

    #[test]
    fn encode_u8_one_padding() {
        let input: Vec<u8> = vec![0x4d, 0x61];
        assert_eq!(Ok(String::from("TWE=")), input.encode());
    }

    #[test]
    fn encode_u8_two_padding() {
        let input: Vec<u8> = vec![0x4d];
        assert_eq!(Ok(String::from("TQ==")), input.encode());
    }

    #[test]
    fn encode_u8() {
        let input: Vec<u8> = String::from("light work.").as_bytes().to_vec();
        assert_eq!(Ok(String::from("bGlnaHQgd29yay4=")), input.encode());
    }
}
