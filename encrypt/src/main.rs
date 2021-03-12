/// file: main.rs
use concrete::*;
use std::time::{Duration, Instant};

fn main() -> Result<(), CryptoAPIError> {
    // generate a secret key and save it
    
    let start_key = Instant::now();
    let secret_key = LWESecretKey::new(&LWE128_630);
    let duration_key = start_key.elapsed();
    
        // a list of messages
    let messages: Vec<f64> = vec![-6.276, 4.3, 0.12, -1.1, 7.78];
    println!("Mensaje original: {:?}", messages);
    
    secret_key.save("my_very_secret_key.json");
    
    let start_encryption = Instant::now();
    // create an encoder
    let encoder = Encoder::new(-10., 10., 8, 0)?;
    // encode and encrypt message vector
    let ciphertext = VectorLWE::encode_encrypt(&secret_key, &messages, &encoder)?;
    let duration_encryption = start_encryption.elapsed();
    
    
    let start_decryption = Instant::now();
    // decrypt
    let outputs: Vec<f64> = ciphertext.decrypt_decode(&secret_key)?;    
    let duration_decryption = start_decryption.elapsed();
    
    

    println!("Mensaje descifrado: {:?}", outputs);
    println!("Tiempo generando claves: {:?}", duration_key);
    println!("Tiempo de cifrado: {:?}", duration_encryption);
    println!("Tiempo de descifrado: {:?}", duration_decryption);
    println!("Tiempo de ejecución total: {:?}", start_key.elapsed());
    
    Ok(())
}
