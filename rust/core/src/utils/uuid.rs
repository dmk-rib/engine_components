use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct UuidError {
    pub message: String,
}

impl std::fmt::Display for UuidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for UuidError {}

pub struct UUID;

impl UUID {
    pub fn create() -> String {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let mut seed = nanos as u64 ^ (nanos >> 64) as u64;

        let mut bytes = [0u8; 16];
        for byte in &mut bytes {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            *byte = (seed & 0xFF) as u8;
        }

        bytes[6] = (bytes[6] & 0x0f) | 0x40;
        bytes[8] = (bytes[8] & 0x3f) | 0x80;

        format!(
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            bytes[0],
            bytes[1],
            bytes[2],
            bytes[3],
            bytes[4],
            bytes[5],
            bytes[6],
            bytes[7],
            bytes[8],
            bytes[9],
            bytes[10],
            bytes[11],
            bytes[12],
            bytes[13],
            bytes[14],
            bytes[15]
        )
    }

    pub fn validate(uuid: &str) -> Result<(), UuidError> {
        let bytes = uuid.as_bytes();
        if bytes.len() != 36 {
            return Err(Self::invalid(uuid));
        }
        for (index, ch) in bytes.iter().enumerate() {
            let is_dash = matches!(index, 8 | 13 | 18 | 23);
            if is_dash {
                if *ch != b'-' {
                    return Err(Self::invalid(uuid));
                }
                continue;
            }
            if !matches!(ch, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F') {
                return Err(Self::invalid(uuid));
            }
        }
        if bytes[14] != b'4' {
            return Err(Self::invalid(uuid));
        }
        let variant = bytes[19];
        if !matches!(variant, b'8' | b'9' | b'a' | b'A' | b'b' | b'B') {
            return Err(Self::invalid(uuid));
        }
        Ok(())
    }

    fn invalid(uuid: &str) -> UuidError {
        UuidError {
            message: format!(
                "{uuid} is not a valid UUID v4.\n\n- If you're the tool creator, you can take one from https://www.uuidgenerator.net/.\n\n- If you're using a platform tool, verify the uuid isn't misspelled or contact the tool creator.",
            ),
        }
    }
}
