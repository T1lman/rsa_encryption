mod algo;

fn main() {
    let p = 739;
    let q = 199;
    let keys = algo::create_keys(p, q).unwrap();
    println!("{:?}", keys);
    let msg = String::from("Hello world!").as_bytes().to_vec();
    let encrypted = algo::encrypt(msg, keys.0);
    println!("{:?}", encrypted);
    let decrypted = algo::decrypt(encrypted, keys.1);
    println!("{}", String::from_utf8_lossy(&decrypted));
}
