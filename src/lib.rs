use std::str;

pub trait Base64 {
    fn encode(&self) -> String;
}

impl Base64 for String {
    fn encode(&self) -> String {
        let a = &self.as_bytes();

        let mut octal = String::new();
        let mut i = 0;

        // The number of full sextets to process
        let blockstoprocess = match a.len() % 3 {
            0 => a.len(),
            _ => a.len() - a.len() % 3,
        };
        let padding = match a.len() % 3 {
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

        let table = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut result = String::new();

        for i in 0..(sextets.len() - padding) {
            result.push_str(&table[sextets[i]..(sextets[i] + 1)]);
        }
        match padding {
            1 => result.push_str("="),
            2 => result.push_str("=="),
            _ => {}
        };
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Base64;
    #[test]
    fn encode_works() {
        assert_eq!("SmUgdCdhaW1lIG1hIGNow6lyaWU=", String::from("Je t'aime ma chérie").encode());
    }
}
