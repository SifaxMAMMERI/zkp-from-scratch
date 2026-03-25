

fn main() {
  
    let p: i128 = 17; 
    let a: i128 = 5;
    let b: i128 = 10;

    println!("Field Prime P = {}", p);

  // Addition: (a + b) % p
    let sum = (a + b) % p;
    println!("Addition: ({} + {}) % {} = {}", a, b, p, sum);

    //  Subtraction: (a - b) % p
    // We add P to avoid negative results before the modulo
    let sub = (a - b + p) % p;
    println!("Subtraction: ({} - {}) % {} = {}", a, b, p, sub);

    // Multiplication: (a * b) % p
    let mul = (a * b) % p;
    println!("Multiplication: ({} * {}) % {} = {}", a, b, p, mul);

   // Using Fermat's Little Theorem: inv(a) = a^(p-2) % p
    let inv_b = modular_pow(b, p - 2, p);
    let div = (a * inv_b) % p;
    println!("Division ({} / {}) % {} = {}", a, b, p, div);
}

fn modular_pow(mut base: i128, mut exp: i128, m: i128) -> i128 {
    let mut res = 1;
    base %= m;
    while exp > 0 {
        if exp % 2 == 1 { res = (res * base) % m; }
        base = (base * base) % m;
        exp /= 2;
    }
    res
}
