//! Main Censorship module

use once_cell::sync::Lazy;
use regex::Regex;
use rustrict::CensorStr;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(feature = "wasm")]
use wasm_bindgen::JsError;

use crate::error::Error;

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
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub enum CensorTypes {
    /// E.g. <https://example.net>
    Link,
    /// E.g. 127.0.0.1
    IP,
    /// E.g. example@example.net
    Email,
    /// E.g. (\w+)
    Custom,
}

/// Response struct containing info about censor
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Censored {
    original: String,
    censored: String,
    valid: bool,
}

#[cfg(feature = "wasm")]
pub trait Printable {
    fn debug_str(&self) -> String;
}

#[cfg(feature = "wasm")]
impl Printable for Censored {
    fn debug_str(&self) -> String {
        format!(
            "Censored {{ original: {}, censored: {}, valid: {} }}",
            self.original, self.censored, self.valid
        )
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl Censored {
    #[wasm_bindgen(js_name = debug)]
    pub fn wasm_debug(&self) -> String {
        self.debug_str()
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl Censored {
    #[wasm_bindgen(getter)]
    pub fn original(&self) -> String {
        self.original.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn censored(&self) -> String {
        self.censored.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn valid(&self) -> bool {
        self.valid
    }
}

/// Censors given string
///
/// # Arguments
///
/// * `sentence` - Sentence to be censored
/// * `types` - Additional types of censoring
/// * `arg` - Additional argument for censoring
///
/// # Errors
///
/// Returns an Error variant if any of the following conditions are met:
///
/// * If argument was't provided when it was needed `Error::NoArgs`.
/// * When invalid regex was passed `Error::Regex`.
///
/// # Examples
///
/// let sentence = "Some sentence"
/// let censored = censor(sentence, [CensorTypes.Custom], "(\\w+)");
#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "censor")]
pub fn censor_w(
    sentence: String,
    types: Box<[CensorTypes]>,
    arg: Option<String>,
) -> Result<Censored, JsError> {
    let res = censor(sentence, types, arg)?;
    Ok(res)
}

/// Censors given string
///
/// # Arguments
///
/// * `sentence` - Sentence to be censored
/// * `types` - Additional types of censoring
/// * `arg` - Additional argument for censoring
///
/// # Errors
///
/// Returns an `Err` variant if any of the following conditions are met:
///
/// * If argument was't provided when it was needed `Error::NoArgs`.
/// * When invalid regex was passed `Error::Regex`.
///
/// # Examples
///
/// ```
/// use little_censor::censor::{CensorTypes, censor};
///
/// let sentence = String::from("Some sentence");
/// let censored = censor(sentence, Box::new([CensorTypes::Custom]), Some(r"(\w+)".to_owned()));
/// ```
pub fn censor(
    sentence: String,
    types: Box<[CensorTypes]>,
    arg: Option<String>,
) -> Result<Censored, Error> {
    let mut types = types.into_vec();
    types.sort();
    types.dedup();
    let mut custom = sentence.clone();

    for typ in types {
        match typ {
            CensorTypes::Link => regex_censor(&mut custom, LINK_REGEX.clone()),
            CensorTypes::IP => regex_censor(&mut custom, IP_REGEX.clone()),
            CensorTypes::Email => regex_censor(&mut custom, EMAIL_REGEX.clone()),
            CensorTypes::Custom => {
                let regex = Regex::new(arg.as_ref().ok_or(Error::NoArgs)?)?;
                regex_censor(&mut custom, regex);
            }
        }
    }

    let censored = custom.censor();
    let censored = fix_sentence(custom, censored);

    Ok(Censored {
        original: sentence.clone(),
        censored: censored.clone(),
        valid: sentence == censored,
    })
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
        let censored = censor(sentence, Box::new([]), None);
        assert_eq!(
            censored.unwrap(),
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
        let censored = censor(sentence, Box::new([]), None);
        assert_eq!(
            censored.unwrap(),
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
        let censored = censor(sentence, Box::new([CensorTypes::Link]), None);
        assert_eq!(
            censored.unwrap(),
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
        let censored = censor(sentence, Box::new([CensorTypes::IP]), None);
        assert_eq!(
            censored.unwrap(),
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
        let censored = censor(sentence, Box::new([CensorTypes::Email]), None);
        assert_eq!(
            censored.unwrap(),
            Censored {
                original: "email leak example@example.net".to_owned(),
                censored: "email leak *******************".to_owned(),
                valid: false,
            }
        );
    }
}
