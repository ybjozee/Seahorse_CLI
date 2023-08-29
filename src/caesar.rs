const ALPHABETS: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn encrypt(plaintext: &str, rotations: i32) -> String {
    let mut ciphertext = String::new();
    for character in plaintext.chars() {
        let character_index = character_index(character.to_ascii_lowercase());
        if character_index == -1 {
            ciphertext = format!("{ciphertext}{character}")
        } else {
            let cipher_index = cipher_index(&character_index, &rotations);
            let cipher = replacement_character(cipher_index);
            if character.is_uppercase() {
                ciphertext = format!("{ciphertext}{}", cipher.to_uppercase())
            } else {
                ciphertext = format!("{ciphertext}{cipher}")
            }
        }
    }
    ciphertext
}

fn character_index(element: char) -> i32 {
    let index = ALPHABETS.find(element);
    match index {
        None => { -1 }
        Some(i) => { i.try_into().unwrap() }
    }
}

fn cipher_index(plain_index: &i32, key: &i32) -> usize {
    let new_index = plain_index + key;
    if new_index > 25 {
        return (new_index % 26) as usize;
    }
    new_index as usize
}

fn replacement_character(index: usize) -> char {
    ALPHABETS.as_bytes()[index] as char
}

pub fn decrypt(ciphertext: &str, rotations: i32) -> String {
    let mut plaintext = String::new();
    for character in ciphertext.chars() {
        let character_index = character_index(character.to_ascii_lowercase());
        if character_index == -1 {
            plaintext = format!("{plaintext}{character}")
        } else {
            let plain_index = plain_index(&character_index, &rotations);
            let plain_character = replacement_character(plain_index);
            if character.is_uppercase() {
                plaintext = format!("{plaintext}{}", plain_character.to_uppercase())
            } else {
                plaintext = format!("{plaintext}{plain_character}")
            }
        }
    }
    plaintext
}

fn plain_index(cipher_index: &i32, key: &i32) -> usize {
    let mut new_index = cipher_index - key;
    if new_index < 0 {
        new_index = 26 + (&new_index % 26)
    }
    if new_index > 25 {
        return (new_index % 26) as usize;
    }
    return new_index as usize;
}