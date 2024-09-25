use std::str::FromStr;

pub struct BinaryPattern(Vec<Option<u8>>);

impl FromStr for BinaryPattern {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = s
            .split_whitespace()
            .map(|s| match s {
                "?" => None,
                _ => Some(u8::from_str_radix(s, 16).expect("Invalid hex value")),
            })
            .collect();
        Ok(BinaryPattern(pattern))
    }
}

impl BinaryPattern {
    pub fn search(&self, data: &[u8]) -> Vec<usize> {
        let mut matches = Vec::new();
        let data_len = data.len();
        let pattern_len = self.0.len();
        for i in 0..data_len.saturating_sub(pattern_len) {
            if self
                .0
                .iter()
                .enumerate()
                .all(|(j, &p)| p.map_or(true, |byte| data[i + j] == byte))
            {
                matches.push(i)
            }
        }
        matches
    }
}
