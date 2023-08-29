use std::collections::HashMap;

const CHUNK_SIZE: usize = 5;

pub fn encrypt(plaintext: &str) -> String {
    let lookup: HashMap<char, &str> = HashMap::from([
        ('A', "aaaaa"), ('B', "aaaab"), ('C', "aaaba"), ('D', "aaabb"), ('E', "aabaa"),
        ('F', "aabab"), ('G', "aabba"), ('H', "aabbb"), ('I', "abaaa"), ('J', "abaab"),
        ('K', "ababa"), ('L', "ababb"), ('M', "abbaa"), ('N', "abbab"), ('O', "abbba"),
        ('P', "abbbb"), ('Q', "baaaa"), ('R', "baaab"), ('S', "baaba"), ('T', "baabb"),
        ('U', "babaa"), ('V', "babab"), ('W', "babba"), ('X', "babbb"), ('Y', "bbaaa"),
        ('Z', "bbaab"),
    ]);

    let mut ciphertext = String::new();
    for character in plaintext.chars() {
        let key = &character.to_ascii_uppercase();
        if lookup.contains_key(key) {
            let cipher = lookup.get(key).unwrap();

            if character.is_uppercase() {
                ciphertext = format!("{ciphertext}{}", cipher.to_uppercase())
            } else {
                ciphertext = format!("{ciphertext}{cipher}")
            }
        } else {
            ciphertext = format!("{ciphertext}{character}");
        }
    }
    ciphertext
}

pub fn decrypt(ciphertext: &str) -> String {
    let reverse_lookup: HashMap<&str, char> = HashMap::from([
        ("aaaaa", 'A'), ("aaaab", 'B'), ("aaaba", 'C'), ("aaabb", 'D'), ("aabaa", 'E'),
        ("aabab", 'F'), ("aabba", 'G'), ("aabbb", 'H'), ("abaaa", 'I'), ("abaab", 'J'),
        ("ababa", 'K'), ("ababb", 'L'), ("abbaa", 'M'), ("abbab", 'N'), ("abbba", 'O'),
        ("abbbb", 'P'), ("baaaa", 'Q'), ("baaab", 'R'), ("baaba", 'S'), ("baabb", 'T'),
        ("babaa", 'U'), ("babab", 'V'), ("babba", 'W'), ("babbb", 'X'), ("bbaaa", 'Y'),
        ("bbaab", 'Z'),
    ]);

    let mut plaintext = String::new();
    let mut chunk = String::new();
    let characters = ciphertext.as_bytes();

    for i in 0..ciphertext.len() {
        let character = characters[i] as char;
        if !character.is_alphabetic() {
            plaintext = format!("{plaintext}{character}");
        } else {
            chunk = format!("{chunk}{character}");
        }
        if chunk.len() == CHUNK_SIZE {
            let key = chunk.to_lowercase();
            if reverse_lookup.contains_key(key.as_str()) {
                let plain_character = reverse_lookup.get(key.as_str()).unwrap();
                if chunk.eq(chunk.to_lowercase().as_str()) {
                    plaintext = format!("{plaintext}{}", plain_character.to_lowercase())
                } else {
                    plaintext = format!("{plaintext}{plain_character}")
                }
            } else {
                plaintext = format!("{plaintext}{chunk}");
            }
            chunk = String::new();
        }
    }
    plaintext
}