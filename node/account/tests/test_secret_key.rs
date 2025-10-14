use std::env;

use mina_node_account::AccountSecretKey;

#[test]
fn test_account_secret_key_bs58check_decode() {
    let parsed: AccountSecretKey = "EKFWgzXsoMYcP1Hnj7dBhsefxNucZ6wyz676Qg5uMFNzytXAi2Ww"
        .parse()
        .unwrap();
    // Test by comparing the public key
    assert_eq!(
        parsed.public_key().to_string(),
        "B62qjVQLxt9nYMWGn45mkgwYfcz8e8jvjNCBo11VKJb7vxDNwv5QLPS"
    );
}

#[test]
fn test_account_secret_key_display() {
    let parsed: AccountSecretKey = "EKFWgzXsoMYcP1Hnj7dBhsefxNucZ6wyz676Qg5uMFNzytXAi2Ww"
        .parse()
        .unwrap();
    assert_eq!(
        &parsed.to_string(),
        "EKFWgzXsoMYcP1Hnj7dBhsefxNucZ6wyz676Qg5uMFNzytXAi2Ww"
    );
}

#[test]
fn test_encrypt_decrypt() {
    let password = "not-very-secure-pass";

    let new_key = AccountSecretKey::rand();
    let tmp_dir = env::temp_dir();
    let tmp_path = format!("{}/{}-key", tmp_dir.display(), new_key.public_key());

    // dump encrypted file
    new_key
        .to_encrypted_file(&tmp_path, password)
        .expect("Failed to encrypt secret key");

    // load and decrypt
    let decrypted = AccountSecretKey::from_encrypted_file(&tmp_path, password)
        .unwrap_or_else(|_| panic!("Failed to decrypt secret key file: {}", tmp_path));

    assert_eq!(
        new_key.public_key(),
        decrypted.public_key(),
        "Encrypted and decrypted public keys do not match"
    );
}
