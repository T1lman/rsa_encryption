pub fn euler_totient(p: u128, q: u128) -> u128 {
    return (p - 1) * (q - 1);
}

pub fn generate_e(totient: u128, n: u128) -> Vec<u128> {
    let mut res: Vec<u128> = Vec::new();
    for i in 2..totient {
        let gcds: (u128, u128) = (gcd(i, totient), gcd(i, n));
        match gcds {
            (1, 1) => res.push(i),
            _ => {}
        }
    }
    res
}

pub fn generate_d(totient: u128, e: u128) -> Vec<u128> {
    let mut res: Vec<u128> = Vec::new();
    for i in 0..totient * e {
        match (i * e) % totient {
            1 => res.push(i),
            _ => {}
        }
    }
    res
}

pub fn create_keys(p: u128, q: u128) -> Result<((u128, u128), (u128, u128)), &'static str> {
    let check = (check_prime(p), check_prime(q));
    match check {
        (true, true) => {
            let n = p * q;
            let phi = euler_totient(p, q);
            let e = generate_e(phi, n);
            let e = e[e.len() - 1];
            let d = generate_d(phi, e);
            let d = d[d.len() - 1];
            return Ok(((e, n), (d, n)));
        }
        _ => Err("At least one of the provided numbers is not a prime number!"),
    }
}

fn check_prime(prime: u128) -> bool {
    for i in 2..prime / 2 {
        if prime % i == 0 {
            return false;
        }
    }
    true
}

fn fast_mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}

pub fn xgcd(mut x: isize, mut y: isize) -> (isize, isize, isize) {
    let (mut a0, mut a1, mut b0, mut b1) = (1, 0, 0, 1);

    while y != 0 {
        // dbg!(x,y);
        let (q, r) = (x / y, x % y);
        let (c, d) = (a0 - q * a1, b0 - q * b1);

        x = y;
        y = r;
        a0 = a1;
        a1 = c;
        b0 = b1;
        b1 = d;
    }
    (x, a0, b0)
}

pub fn gcd(a: u128, b: u128) -> u128 {
    match b {
        0 => return a,
        _ => return gcd(b, a % b),
    }
}
