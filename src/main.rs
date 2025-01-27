fn letter_to_num(c: char) -> Option<u8> {
    match c {
        'A'..='Z' => Some(c as u8 - b'A'),
        'a'..='z' => Some(c as u8 - b'a'),
        _ => None
    }
}

fn num_to_letter(n: u8) -> char {
    (n % 26 + b'A') as char
}

fn decrypt_with_key(ciphertext: &str, key: &str) -> String {
    // Remove spaces from both ciphertext and key
    let clean_cipher: String = ciphertext.chars().filter(|c| !c.is_whitespace()).collect();
    let clean_key: String = key.chars().filter(|c| !c.is_whitespace()).collect();
    
    // Convert to numbers and decrypt
    clean_cipher
        .chars()
        .zip(clean_key.chars().cycle())
        .filter_map(|(cipher_char, key_char)| {
            match (letter_to_num(cipher_char), letter_to_num(key_char)) {
                (Some(cipher_num), Some(key_num)) => {
                    // Subtract key from cipher (with modulo 26)
                    let decrypted = (cipher_num as i32 - key_num as i32 + 26) % 26;
                    Some(num_to_letter(decrypted as u8))
                },
                _ => None
            }
        })
        .collect()
}

fn main() {
    let ciphertext = "bsaspp kkuosp";
    let key = "rsidpy dkawoa";
    
    let decrypted = decrypt_with_key(ciphertext, key);
    println!("Ciphertext: {}", ciphertext);
    println!("Key:        {}", key);
    println!("Decrypted:  {}", decrypted);
}