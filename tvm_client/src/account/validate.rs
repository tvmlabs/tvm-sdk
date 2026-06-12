use crate::error::ClientResult;
use crate::processing::Error as ProcessingError;

/// Validates that the value is a 64-character lowercase or mixed-case hex
/// string with no `0x` prefix and no workchain. Used for `account_id` and
/// `dapp_id` fields in the v3 dapp_id API.
pub fn validate_hex_id(field: &str, value: &str) -> ClientResult<()> {
    if value.len() != 64 || !value.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(ProcessingError::invalid_data(format!(
            "`{}` must be a 64-character hex string (no 0x prefix, no workchain)",
            field
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_hex_id;

    const VALID: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    #[test]
    fn accepts_64_lowercase_hex() {
        assert!(validate_hex_id("account_id", VALID).is_ok());
    }

    #[test]
    fn accepts_64_uppercase_hex() {
        assert!(validate_hex_id("dapp_id", &VALID.to_uppercase()).is_ok());
    }

    #[test]
    fn rejects_short() {
        assert!(validate_hex_id("dapp_id", "abc").is_err());
    }

    #[test]
    fn rejects_long() {
        let too_long = format!("{}f", VALID);
        assert!(validate_hex_id("dapp_id", &too_long).is_err());
    }

    #[test]
    fn rejects_0x_prefix() {
        let prefixed = format!("0x{}", &VALID[..62]);
        assert!(validate_hex_id("dapp_id", &prefixed).is_err());
    }

    #[test]
    fn rejects_workchain_colon() {
        let with_wc = format!("0:{}", &VALID[..62]);
        assert!(validate_hex_id("account_id", &with_wc).is_err());
    }

    #[test]
    fn rejects_non_hex_char() {
        let bad: String = std::iter::once('z').chain(VALID.chars().skip(1)).collect();
        assert!(validate_hex_id("dapp_id", &bad).is_err());
    }

    #[test]
    fn rejects_empty() {
        assert!(validate_hex_id("dapp_id", "").is_err());
    }
}
