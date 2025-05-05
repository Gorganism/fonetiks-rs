use input_loop::input_loop;
use cmudict_fast::Cmudict;
use std::collections::HashSet;

fn main() {
    let dict = match Cmudict::new("cmudict-0.7b.txt") {
        Ok(dict) => dict,
        Err(e) => {
            eprintln!("Error loading CMU dictionary: {}", e);
            eprintln!("Please ensure 'cmudict-0.7b.txt' is in your current directory");
            std::process::exit(1);
        }
    };

    let mut english: String = input_loop("Input a string: ");
    english = english.to_lowercase();

    let words: Vec<String> = english
        .split_whitespace()
        .map(|word| word.to_lowercase())
        .collect();

    let mut phoneme_sets = [
        ("DH", HashSet::new()), 
        ("TH", HashSet::new()),
        ("AE", HashSet::new()),
        ("OW", HashSet::new()),
    ];

    for word in &words {
        if let Some(pronunciations) = dict.get(word) {
            for pronunciation in pronunciations {
                for (phoneme, set) in &mut phoneme_sets {
                    let pron_str = format!("{:?}", pronunciation);
                    if pron_str.contains(*phoneme) {
                        set.insert(word.clone());
                    }
                }
            }
        }
    }

    let replacements = vec![
        ("thom", "tom"), ("coope", "cöpe"), ("co-op", "cöp"), ("alk", "ak"),
        ("sh", "ʃ"), ("tio", "ʃo"), ("sio", "ʃo"), ("sure", "ʃur"),
        ("wh", "w"), ("co", "ko"), ("cu", "ku"), ("ca", "ka"),
        ("ck", "k"), ("ic", "ik"), ("cr", "kr"), ("ci", "si"),
        ("ce", "se"), ("cy", "sy"), ("ch", "c"), ("ikh", "ic"),
        ("kn", "gn"), ("ec", "ek"), ("act", "akt"), ("cem", "kem"),
        ("whik", "whic"), ("nge", "nje"), ("ng", "ŋ"), ("nk", "ŋk"),
        ("ph", "f"), ("ause", "auz"), ("ouse", "aus"), ("cough", "koff"),
        ("laugh", "laff"), ("enough", "enuf"), ("tough", "tuff"), ("ough", "o"),
        ("gh", ""), ("exa", "egza"), ("exi", "egzi"), ("ax", "aks"),
        ("ox", "oks"), ("ux", "uks"), ("ix", "iks"), ("ex", "eks"),
        ("x", "z"), ("oo", "u"), ("þro", "þru"), ("þruw", "þrow"),
        ("of", "ov"), ("uld", "ud"), ("kss", "ks"), ("idk", "idg"),
        ("ture", "cur"), ("æcʃ", "ækʃ"), ("wið", "wiþ"), ("arsitekkur", "arkitekcur"),
        ("geo", "jeo"), ("rge", "rje"),
    ];

    let rune_replacements: Vec<(&str, &str)> = vec![
        ("ᛒᛥ", "v"), ("ᛋᛥ", "z"), ("ᚪ", "a"), ("ᛒ", "b"), ("ᚳ", "ch"), 
        ("ᛞ", "d"), ("ᛖ", "e"), ("ᚠ", "f"), ("ᚷ", "g"), ("ᚻ", "h"), ("ᛁ", "i"), 
        ("ᛡ", "j"), ("ᛣ", "k"), ("ᛚ", "l"), ("ᛗ", "m"), ("ᚾ", "n"), 
        ("ᚩ", "o"), ("ᛈ", "p"), ("ᛢ", "q"), ("ᚱ", "r"), ("ᛋ", "s"), ("ᛏ", "t"), 
        ("ᚢ", "u"), ("ᚹ", "w"), ("ᛉ", "x"), ("ᚣ", "y"), ("ᛇ", "ai"), 
        ("ᚫ", "ae"), ("ᚦ", "th"), ("ᛝ", "ng"), ("ᛠ", "ea"), ("ᛟ", "oe"), ("•", " ")
    ];

    let runes: Vec<(&str, &str)> = vec![
        ("h", "ᚻ"), ("f", "ᚠ"), ("u", "ᚢ"), ("ð", "ᚦ"), ("þ", "ᚦ"), 
        ("o", "ᚩ"), ("r", "ᚱ"), ("c", "ᚳ"), ("g", "ᚷ"), ("w", "ᚹ"),
        ("n", "ᚾ"), ("i", "ᛁ"), ("j", "ᛡ"), ("p", "ᛈ"), ("ks", "ᛉ"), 
        ("s", "ᛋ"), ("t", "ᛏ"), ("b", "ᛒ"), ("e", "ᛖ"), ("m", "ᛗ"), 
        ("l", "ᛚ"), ("ŋ", "ᛝ"), ("œ", "ᛟ"), ("d", "ᛞ"), ("a", "ᚪ"), 
        ("æ", "ᚫ"), ("ea", "ᛠ"), ("y", "ᚣ"), ("ʃ", "ᛋᚻ"), ("ö", "ᚩᚩ"), 
        ("k", "ᛣ"), ("q", "ᛢ"), ("v", "ᛒᛥ"), ("z", "ᛋᛥ"), (" ", "•"),
    ];

    let mut result = english.clone();

    let contains_runes = rune_replacements.iter().any(|(rune_char, _)| english.contains(rune_char));

    if contains_runes {
        for (rune_char, phonetic_char) in &rune_replacements {
            if result.contains(rune_char) {
                result = result.replace(rune_char, phonetic_char);
            }
        }

        for word in &phoneme_sets[0].1 {
            if word.contains("th") && word != "with" {
                result = result.replace(word, &format!("ð{}", &word[2..]));
            }
        }

        for word in &phoneme_sets[1].1 {
            if word.contains("th") && word != "with" {
                result = result.replace(word, &format!("þ{}", &word[2..]));
            }
        }
    }
    for (from, to) in &replacements {
        result = result.replace(from, to);
    }

    let mut rune_result: String = result.clone().to_lowercase();
    for (from, to) in &runes {
        rune_result = rune_result.replace(from, to);
    }

    println!("Fonetiks: {}", result);
    if !contains_runes {
        println!("Runetiks: {}", rune_result)
    }
}