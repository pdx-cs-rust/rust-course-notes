use toy_rsa::*;

#[test]
fn test_exp() {
    assert_eq!(EXP, 65_537);
}

#[test]
fn test_rsa_example() {
    let msg = 0x12345f;
    let (p, q) = (0xed23e6cd, 0xf050a04d);
    let cmsg = encrypt(p as u64 * q as u64, msg);
    assert_eq!(cmsg, 0x6418280e0c4d7675);
    let pmsg = decrypt((p, q), cmsg);
    assert_eq!(msg, pmsg);
}

#[test]
fn test_rsa_random() {
    let mut state = 0x123456789abcdef0u64;
    for _ in 0..100 {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let plain = (state >> 32) as u32;
        let (p, q) = toy_rsa::genkey();
        assert_ne!(p, q);
        assert!(p >= 1 << 31);
        assert!(q >= 1 << 31);
        let cipher = toy_rsa::encrypt(p as u64 * q as u64, plain);
        let decrypted = toy_rsa::decrypt((p, q), cipher);
        assert_eq!(plain, decrypted);
    }
}

#[test]
fn test_rsa_message_boundaries() {
    let key = toy_rsa::genkey();
    for plain in [0, 1, u32::MAX, key.0, key.1] {
        let cipher = toy_rsa::encrypt(u64::from(key.0) * u64::from(key.1), plain);
        assert_eq!(toy_rsa::decrypt(key, cipher), plain);
    }
}
