mod algo;

fn main() {
    let p = 59;
    let q = 37;
    println!("{:?}", algo::create_keys(p, q).unwrap());
}
