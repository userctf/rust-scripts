use num::bigint::{BigInt, ToBigInt};
use std::io;
use tqdm::tqdm;
use std::collections::HashMap;

fn main() {
    // getting params for query y = a^x mod p
    println!("Equation is y = a^x mod p");
    println!("Write y");
    let y = get_number();
    println!("Write a");
    let a = get_number();
    println!("Write p");
    let p = get_number();
    // let y = 711087309.to_bigint().unwrap();
    // let a = 9.to_bigint().unwrap();
    // let p = 1073741823.to_bigint().unwrap();


    println!("Trying to solve equation {y} = {a}^x mod {p}");

    let x = baby_step_giant_step(&y, &a, &p).unwrap();

    print!("x is {x}");
}

fn baby_step_giant_step(y: &BigInt, a: &BigInt, p: &BigInt) -> Option<BigInt> {
    let m = p.sqrt() + 1.to_bigint().unwrap();
    let k = &m + 1;


    assert!(&k * &m > *p);

    let mut memory_table = HashMap::new();
    for i_m in tqdm(num_iter::range_inclusive(BigInt::from(1), m.clone() - BigInt::from(1))) {
        memory_table.insert(
            (y * a.modpow(&i_m, p)) % p,
            i_m,
        );
    }

    for i_k in tqdm(num_iter::range_inclusive(BigInt::from(1), k)) {
        let a_im = &a.modpow(&(&i_k * &m), p);
        if memory_table.contains_key(&a.modpow(&(&i_k * &m), p)) {
            // let a_im = &a.modpow(&(&i_k * &m), p);
            let j = memory_table.get(a_im).expect("No key found");

            return Some(i_k * m - j);
        }
    }

    return None;


}

fn get_number() -> BigInt {
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("failed to read from stdin");

    let number:BigInt =  match number.trim().parse() {
        Ok(i) => i,
        Err(_) => {println!("this was not an integer. Default is 1"); 1.to_bigint().unwrap()}
    };

    return number;
}
