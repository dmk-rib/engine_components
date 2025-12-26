use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone)]
pub struct UuidError {
    pub message: String,
}

impl fmt::Display for UuidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for UuidError {}

pub struct UUID;

static UUID_COUNTER: AtomicU64 = AtomicU64::new(1);

impl UUID {
    pub fn create() -> String {
        let counter = UUID_COUNTER.fetch_add(1, Ordering::Relaxed);
        let mut bytes = [0u8; 16];
        bytes[..8].copy_from_slice(&counter.to_be_bytes());
        bytes[8] = 0x40;
        bytes[9] = 0x80;
        let hex: String = bytes.iter().map(|b| format!("{:02x}", b)).collect();
        format!(
            "{}-{}-{}-{}-{}",
            &hex[0..8],
            &hex[8..12],
            &hex[12..16],
            &hex[16..20],
            &hex[20..32]
        )
    }

    pub fn validate(uuid: &str) -> Result<(), UuidError> {
        if uuid.len() != 36 {
            return Err(UuidError {
                message: format!("{uuid} is not a valid UUID v4"),
            });
        }
        let bytes = uuid.as_bytes();
        let hyphen_positions = [8, 13, 18, 23];
        for &pos in &hyphen_positions {
            if bytes[pos] != b'-' {
                return Err(UuidError {
                    message: format!("{uuid} is not a valid UUID v4"),
                });
            }
        }
        if bytes[14] != b'4' {
            return Err(UuidError {
                message: format!("{uuid} is not a valid UUID v4"),
            });
        }
        if !matches!(bytes[19], b'8' | b'9' | b'a' | b'b' | b'A' | b'B') {
            return Err(UuidError {
                message: format!("{uuid} is not a valid UUID v4"),
            });
        }
        for (idx, ch) in bytes.iter().enumerate() {
            if hyphen_positions.contains(&idx) {
                continue;
            }
            if !matches!(ch, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F') {
                return Err(UuidError {
                    message: format!("{uuid} is not a valid UUID v4"),
                });
            }
        }
        Ok(())
    }
}
