//! Main Censorship module
use std::collections::HashSet;

use once_cell::sync::Lazy;
use regex::Regex;
use rustrict::CensorStr;

static LINK_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()!@:%_\+.~#?&\/\/=]*)"#).expect("Failed to create regex")
});
static IP_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(\b25[0-5]|\b2[0-4][0-9]|\b[01]?[0-9][0-9]?)(\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}"#).expect("Failed to create regex")
});
static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"[^@ \t\r\n]+@[^@ \t\r\n]+\.[^@ \t\r\n]+"#).expect("Failed to create regex")
});

/// Types to add additional Censor Methods
#[derive(PartialEq, Eq, Hash)]
pub enum CensorTypes {
    /// E.g. <https://example.net>
    Link,
    /// E.g. 127.0.0.1
    IP,
    /// E.g. example@example.net
    Email,
}

/// Response struct containing info about censor
#[derive(Debug, PartialEq, Eq)]
pub struct Censored {
    original: String,
    censored: String,
    valid: bool,
}

/// Main censor function
pub fn censor(sentence: String, types: HashSet<CensorTypes>) -> Censored {
    let mut custom = sentence.clone();

    for typ in types {
        match typ {
            CensorTypes::Link => regex_censor(&mut custom, LINK_REGEX.clone()),
            CensorTypes::IP => regex_censor(&mut custom, IP_REGEX.clone()),
            CensorTypes::Email => regex_censor(&mut custom, EMAIL_REGEX.clone()),
        }
    }

    let censored = custom.censor();
    let censored = fix_sentence(custom, censored);

    Censored {
        original: sentence.clone(),
        censored: censored.clone(),
        valid: sentence == censored,
    }
}

/// Censor by given regex pattern
fn regex_censor(sentence: &mut String, regex: Regex) {
    let binding = sentence.clone();
    let matches: Vec<&str> = regex.find_iter(&binding).map(|v| v.as_str()).collect();

    // Replace links with coresponding number of stars
    for value in matches {
        *sentence = sentence.replace(value, &"*".repeat(value.len()));
    }
}

fn fix_sentence(original: String, censored: String) -> String {
    censored
        .chars()
        .zip(original.chars())
        .map(|(censor_char, original_char)| {
            if censor_char != '*' {
                original_char
            } else {
                censor_char
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn censor_word() {
        let sentence = String::from("fuck world");
        let censored = censor(sentence, HashSet::new());
        assert_eq!(
            censored,
            Censored {
                original: "fuck world".to_owned(),
                censored: "f*** world".to_owned(),
                valid: false,
            }
        );
    }

    #[test]
    fn utf8_chars() {
        let sentence = String::from("fuck ąćęłńśóźżäöüß fuck");
        let censored = censor(sentence, HashSet::new());
        assert_eq!(
            censored,
            Censored {
                original: "fuck ąćęłńśóźżäöüß fuck".to_owned(),
                censored: "f*** ąćęłńśóźżäöüß f***".to_owned(),
                valid: false,
            }
        );
    }

    #[test]
    fn link_regex_censor() {
        let sentence = String::from("go to this website: https://example.net/");
        let censored = censor(sentence, HashSet::from([CensorTypes::Link]));
        assert_eq!(
            censored,
            Censored {
                original: "go to this website: https://example.net/".to_owned(),
                censored: "go to this website: ********************".to_owned(),
                valid: false,
            }
        );
    }

    #[test]
    fn ip_regex_censor() {
        let sentence = String::from("ip leak 127.0.0.1");
        let censored = censor(sentence, HashSet::from([CensorTypes::IP]));
        assert_eq!(
            censored,
            Censored {
                original: "ip leak 127.0.0.1".to_owned(),
                censored: "ip leak *********".to_owned(),
                valid: false,
            }
        );
    }

    #[test]
    fn email_regex_censor() {
        let sentence = String::from("email leak example@example.net");
        let censored = censor(sentence, HashSet::from([CensorTypes::Email]));
        assert_eq!(
            censored,
            Censored {
                original: "email leak example@example.net".to_owned(),
                censored: "email leak *******************".to_owned(),
                valid: false,
            }
        );
    }
}
