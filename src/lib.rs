//! Yet another base64 encoding library, which implements the encode() and decode() methods for the String type.
//! My goals were:
//! - to encode / decode with padding
//! - to use it in an interactive command line string encoding utility that fits my needs.

use std::{fmt, error::Error, str};

const TABLE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Base64Error {
    /// Invalid input data length (must be a multiple of 4)
    InvalidDataLenght,
    /// Incorrectly encoded input data
    InvalidBase64Data,
}

impl Error for Base64Error {}

impl fmt::Display for Base64Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Decoding error : ")?;
        f.write_str(match self {
            Base64Error::InvalidDataLenght => "Invalid input data length",
            Base64Error::InvalidBase64Data => "Invalid base64 data",
        })
    }
}

impl From<std::string::FromUtf8Error> for Base64Error {
    fn from(_e: std::string::FromUtf8Error) -> Base64Error {
        Base64Error::InvalidBase64Data
    }
}

pub trait Base64 {
    fn encode(&self) -> String;
    fn decode(&self) -> Result<String, Base64Error>;
}

impl Base64 for String {
    /// Encodes a String with the base64 scheme
    ///
    /// Example:
    /// ```
    /// use lib_base64::Base64;
    /// let s = String::from("Test");
    /// assert_eq!("VGVzdA==", s.encode())
    /// ```
    fn encode(&self) -> String {
        let a = &self.as_bytes();

        let mut octal = String::new();
        let mut i = 0;

        // The number of full sextets to process
        let inputlenmod = a.len() % 3;
        let blockstoprocess = match inputlenmod {
            0 => a.len(),
            _ => a.len() - inputlenmod,
        };
        let padding = match inputlenmod {
            0 => 0,
            _ => 3 - (a.len() - blockstoprocess),
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
            .map(|s| usize::from_str_radix(str::from_utf8(s).unwrap(), 8).unwrap())
            .collect::<Vec<usize>>();

        // For dev and debug
        #[cfg(debug_assertions)]
        {
            println!("Input as bytes : {:?}", a);
            println!("Length of string to encode : {}", a.len());
            println!("24 bits blocks to process : {}", blockstoprocess);
            println!("Padding : {}", padding);
            println!("Padded input as octal : {}", octal);
            println!("Decimal encoded data : {:?}", sextets);
        }

        let mut result = String::new();

        for i in 0..(sextets.len() - padding) {
            result.push_str(&TABLE[sextets[i]..(sextets[i] + 1)]);
        }
        match padding {
            1 => result.push_str("="),
            2 => result.push_str("=="),
            _ => {}
        };
        result
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
        let padding = encoded_data.matches("=").count();

        if encoded_data.len() % 4 != 0 {
            return Err(Base64Error::InvalidDataLenght);
        };

        // replaces padding characters by characters decoded as zero
        for _ in 0..padding {
            encoded_data.pop();
        }

        for _ in 0..padding {
            encoded_data.push_str("A");
        }

        // Retrieves octal indexes of encoded characters
        let octal = encoded_data
            .chars()
            .map(|c| format!("{:02o}", TABLE.find(c).unwrap_or(65)))
            .collect::<Vec<String>>();

        // Gathers the 4 sextets (24 bits) collection
        let mut octalsextets = Vec::new();
        let mut n = 0;
        while n < encoded_data.len() {
            let mut s = String::new();
            for i in 0..4 {
                if octal[n + i] == "101" { return Err(Base64Error::InvalidBase64Data) }
                s.push_str(octal[n + i].as_str());
            }
            n += 4;
            octalsextets.push(s);
        }

        // Decodes the 4 sextets to 3 bytes
        let decimal = octalsextets
            .iter()
            .map(|s| usize::from_str_radix(s, 8).unwrap())
            .collect::<Vec<usize>>();

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

        // For dev and debug
        #[cfg(debug_assertions)]
        {
            println!("Encoded data length : {}", encoded_data.len());
            println!("24 bits blocks to process : {}", encoded_data.len() / 4);
            println!("Padding : {}", padding);
            println!("Octal indexes of encoded characters : {:?}", octal);
            println!("Gathering sextets 4 by 4 : {:?}", octalsextets);
            println!("Decoded data (4 sextets to 3 bytes) : {:x?}", decimal);
            println!("Decoded bytes : {:x?}", &bytes);
        }

        let result = String::from_utf8(bytes)?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::Base64;
    #[test]
    fn encode_works() {
        assert_eq!(
            "SmUgdCdhaW1lIG1hIGNow6lyaWU=",
            String::from("Je t'aime ma chérie").encode()
        );
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
}
