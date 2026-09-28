use toy_rsa::*;
use rand;

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
    for _ in 0..100 {
        let plain = rand::random();
        let (p, q) = toy_rsa::genkey();
        let cipher = toy_rsa::encrypt(p as u64 * q as u64, plain);
        let decrypted = toy_rsa::decrypt((p, q), cipher);
        assert_eq!(plain, decrypted);
    }
}
