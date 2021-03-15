/// file: main.rs
use concrete::*;
use std::time::{Duration, Instant};

fn main() -> Result<(), CryptoAPIError> {

    println!("\n\nPRIMERA IMPLEMENTACIÓN: cifrado y descifrado \n"); 
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
    
    //////////////////////////////////////////////////////////////////////////////////7
    
    println!("\n //////////////////////////////////// \n");
    println!("SEGUNDA IMPLEMENTACIÓN: Key Switching \n"); 
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

