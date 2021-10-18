use regex::Regex;

pub fn create_iban() -> regex::Regex {
    return Regex::new(r"(?m)[A-Z]{2}\d{25}\b").unwrap();
}

pub fn create_email_address() -> regex::Regex {
    return Regex::new(r"(?m)([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
}

pub fn create_ip_address() -> regex::Regex {
    return Regex::new(r"(?m)[0-9]{1,3}(\.[0-9]{1,3}){3}\b").unwrap();
}

pub fn create_japanese_phone_number1() -> regex::Regex {
    return Regex::new(r"(?m)[0][0-9]{1}\-[0-9]{4}\-[0-9]{4}\b").unwrap();
}

pub fn create_japanese_phone_number2() -> regex::Regex {
    return Regex::new(r"(?m)[0][0-9]{2}\-[0-9]{3}\-[0-9]{4}\b").unwrap();
}

pub fn create_japanese_phone_number3() -> regex::Regex {
    return Regex::new(r"(?m)[0][0-9]{3}\-[0-9]{2}\-[0-9]{4}\b").unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn iban_check() {
        let re=create_iban();
        assert!(re.is_match("FR7630001007941234567890185"));
    }
    #[test]
    fn email_check() {
        let re=create_email_address();
        assert!(re.is_match("gawen@georepublic.de"));
    }
    #[test]
    fn ip_check() {
        let re=create_ip_address();
        assert!(re.is_match("255.255.255.255"));
    }
    #[test]
    fn japanese_phone_number1_check() {
        let re=create_japanese_phone_number1();
        assert!(re.is_match("00-0000-0000"));
    }
    #[test]
    fn japanese_phone_number2_check() {
        let re=create_japanese_phone_number2();
        assert!(re.is_match("000-000-0000"));
    }
    #[test]
    fn japanese_phone_number3_check() {
        let re=create_japanese_phone_number3();
        assert!(re.is_match("0000-00-0000"));
    }
}
