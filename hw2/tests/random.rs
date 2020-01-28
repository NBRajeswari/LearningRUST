use toy_rsa::*;

#[test]
fn test_example() {
    // Private Key: p = 0xed23e6cd q = 0xf050a04d
    // Public Key: p * q = 0xde9c5816141c8ba9
    // Message: 12345
    // Encrypted: 0x164e44b86776d497
    // Decrypted: 12345
    let (p,q) = genkey();
    println!("Private Key: p = {0} q = {1}", p, q);
    let public_key = p as u64 * q as u64;
    println!("Public Key: p * q = {}", public_key);
    let message = 12345_u32;
    println!("Message = {}", message);
    let encrypted = encrypt(public_key, message);
    println!("Encrypted = {}", encrypted);
    let decrypted = decrypt((p,q), encrypted);
    println!("Decrypted = {}", decrypted);
    assert_eq!(decrypted, message);
}

#[test]
#[should_panic(expected = "assertion failed: msg != 0")]
fn test_msg_0() {
    let (p,q) = genkey();
    let public_key = p as u64 * q as u64;
    let message = 0_u32;
    let _ = encrypt(public_key, message);
}