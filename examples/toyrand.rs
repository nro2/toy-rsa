use toy_rsa::*;
//use std::convert::TryInto;

fn main() {
    let p: u32 = 0xed23e6cd;
    let q: u32 = 0xf050a04d;
    let mess: u32 = 12345;
    let _encryp = 0x164e44b86776d497;
    let _decryp = 12345;
    let key: u64 = 0xde9c5816141c8ba9;

    assert_eq!(encrypt(key, mess), _encryp, "Encrypt assert failed");

    assert_eq!(
        decrypt((p, q), _encryp.into()),
        _decryp,
        "Decrypt assert failed"
    );

    let gen = genkey();
    let testmess = 747364;
    let _p: u64 = gen.0.into();
    let _q: u64 = gen.1.into();
    let new_key: u64 = _p * _q;
    let encrypted = encrypt(new_key, testmess);
    let decrypted = decrypt(gen, encrypted);
    assert_eq!(decrypted, testmess, "did not match before and after")
}
//fn testEncryp(key, mess) {
//    assert_eq!(encrypt(key, mess))
//}
