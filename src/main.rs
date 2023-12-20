use std::{iter::repeat, ops::BitXor};

fn main() {
    let m1 = [
        true, false, true, true, false, true, false, true, false, true, false,
    ];
    let k = [true, true, false, false];
    let pretty_k: Vec<u8> = repeat(k)
        .flat_map(|x| x.iter().map(|b| *b as u8).collect::<Vec<u8>>())
        .take(m1.len())
        .collect();
    println!("M  = {:?}", m1.map(|b| b as u8));
    println!("K  = {:?}", pretty_k);
    let m2 = verenc::<[bool; 11], Vec<bool>, bool, bool, [bool; 4]>(m1, k);
    println!(
        "M' = {:?}",
        m2.iter().map(|b| *b as u8).collect::<Vec<u8>>()
    );
    println!("K  = {:?}", pretty_k);
    let m3 = verenc::<Vec<bool>, Vec<bool>, bool, bool, [bool; 4]>(m2, k);
    println!(
        "M''= {:?}",
        m3.iter().map(|b| *b as u8).collect::<Vec<u8>>()
    );
}

fn verenc<M1, M2, O1, O2, K>(message: M1, key: K) -> M2
where
    M1: IntoIterator<Item = O1>,
    M2: FromIterator<O1>,
    K: IntoIterator<Item = O2>,
    <K as IntoIterator>::IntoIter: Clone,
    O1: BitXor<O2, Output = O1>,
{
    message
        .into_iter()
        .zip(key.into_iter().cycle())
        .map(|(m, k)| m ^ k)
        .collect()
}
