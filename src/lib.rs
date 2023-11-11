use rustrict::Trie;

pub mod censor;
pub mod error;
pub use rustrict::Type;

use error::Error;

/// A struct representing a vulgar word with its associated type.
///
/// # Examples
///
/// ```
/// use little_censor::{Vulgar, Type};
///
/// let vulgar_word = Vulgar::new(String::from("VulgarWord"), Some(Type::INAPPROPRIATE));
///
/// assert_eq!(vulgar_word.word, "VulgarWord");
/// assert_eq!(vulgar_word.word_type, Type::INAPPROPRIATE);
/// ```
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Vulgar {
    pub word: String,
    pub word_type: Type,
}

impl Vulgar {
    pub fn new(word: String, word_type: Option<Type>) -> Self {
        Self {
            word,
            word_type: word_type.unwrap_or_default(),
        }
    }
}

/// Adds a collection of vulgar words to the Trie.
///
/// This function takes a vector of `Vulgar` instances and adds each word to the Trie
/// data structure with its corresponding word type.
///
/// # Arguments
///
/// * `vulgars` - A vector of `Vulgar` instances containing words and their types.
///
/// # Errors
///
/// Returns an `Err` variant if any of the following conditions are met:
///
/// * The word in any `Vulgar` instance is empty, resulting in an `Error::EmptyWord`.
///
/// # Examples
///
/// ```
/// use little_censor::{add_words, Vulgar, error::Error, Type};
///
/// let vulgars = vec![
///     Vulgar::new("bad_word1".to_string(), Some(Type::INAPPROPRIATE)),
///     Vulgar::new("bad_word2".to_string(), None),
/// ];
///
/// assert_eq!(add_words(vulgars).unwrap(), ());
/// ```
pub fn add_words(vulgars: Vec<Vulgar>) -> Result<(), Error> {
    unsafe {
        for vulgar in vulgars {
            if vulgar.word.is_empty() {
                return Err(Error::EmptyWord);
            }
            Trie::customize_default().set(&vulgar.word, vulgar.word_type);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use rustrict::CensorStr;

    use super::*;

    #[test]
    fn vulgar_construct() {
        let vulgar_default = Vulgar::default();
        let vulgar_new = Vulgar::new(String::new(), Default::default());
        assert_eq!(
            vulgar_new,
            Vulgar {
                word: String::new(),
                word_type: Type::INAPPROPRIATE
            }
        );

        assert_eq!(
            vulgar_default,
            Vulgar {
                word: String::new(),
                word_type: Type::INAPPROPRIATE
            }
        );
    }

    #[test]
    #[should_panic]
    fn empty_word() {
        let empty = Vulgar::new(String::new(), None);

        add_words(vec![empty]).expect("This word is empty");
    }

    #[test]
    fn add_words_to_dict() {
        let words = vec![
            Vulgar::new(String::from("bad_word1"), Some(Type::SEXUAL)),
            Vulgar::new(String::from("bad_word2"), None),
        ];

        add_words(words).unwrap();

        let word = String::from("bad_word1");

        assert!(word.is_inappropriate());
    }
}
