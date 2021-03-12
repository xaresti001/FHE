/// file: main.rs
use concrete::*;
use std::time::{Duration, Instant};

fn main() -> Result<(), CryptoAPIError> {
    
    
    let start_key = Instant::now();
    // encoder
    let encoder = Encoder::new(100., 110., 5, 0)?;
    // generate two secret keys
    let secret_key_before = LWESecretKey::new(&LWE128_1024);
    let secret_key_after = LWESecretKey::new(&LWE128_630);
    // generate the key switching key
    let ksk = crypto_api::LWEKSK::new(&secret_key_before, &secret_key_after, 2, 6);
    let duration_key = start_key.elapsed();
    
    // a list of messages that we encrypt
    let messages: Vec<f64> = vec![106.276, 104.3, 100.12, 101.1, 107.78];
    println!("Mensaje original: {:?}", messages);
    
    let start_encryption1 = Instant::now();
    let ciphertext_before = VectorLWE::encode_encrypt(&secret_key_before, &messages, &encoder)?;
    let duration_encryption1 = start_encryption1.elapsed();
    
    let start_encryption2 = Instant::now();
    // key switch
    let ciphertext_after = ciphertext_before.keyswitch(&ksk);
    let duration_encryption2 = start_encryption2.elapsed();
    
    let start_decryption = Instant::now();
    // decryption
    let outputs: Vec<f64> = ciphertext_before.decrypt_decode(&secret_key_before)?;
    let duration_decryption = start_decryption.elapsed();
    
    println!("Mensaje descifrado: {:?}", outputs);
    println!("Tiempo generando claves: {:?}", duration_key);
    println!("Tiempo de cifrado 1: {:?}", duration_encryption1);
    println!("Tiempo de cifrado 2 (cambio de clave): {:?}", duration_encryption2);
    println!("Tiempo de descifrado: {:?}", duration_decryption);
    println!("Tiempo de ejecución total: {:?}", start_key.elapsed());

    Ok(())
}

