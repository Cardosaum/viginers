use vigeners::{Cipher, TestFileInput, VigenereCipher};

fn main() {
    let file_input = TestFileInput::new("desafio1.txt");
    let contents = file_input.read_contents();

    let cipher = VigenereCipher {
        key: "ABC".to_string(),
    };
    let plaintext = "HELLO";
    let ciphertext = cipher.encrypt(plaintext);
    println!("{}", ciphertext); // prints "HFNLP"

    let decrypted_message = cipher.decrypt(&ciphertext);
    println!("{}", decrypted_message); // prints "HELLO"
}
