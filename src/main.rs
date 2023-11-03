use rand::Rng;
use std::io;
use bitvec::prelude::*;

fn string_to_bitvec(input: &String) -> BitVec<u8> {
    BitVec::from_vec(input.as_bytes().to_vec())
}

fn control_value(ind: usize, input: &BitVec<u8>) -> bool {
    let mut actual_ind = ind - 1;
    let mut cnt = 0;
    while actual_ind < input.len() {
        for i in actual_ind..actual_ind + ind {
            if i >= input.len() {
                break;
            }
            if input[i] {
                cnt += 1;
            }
        }
        actual_ind += 2 * ind;
    }
    cnt % 2 != 0
}

fn hemming_encode(inp: &BitVec<u8>) -> BitVec<u8> {
    let mut input = inp.clone();
    let mut pow: usize = 1;
    let mut len = input.len();
    loop {
        input.insert(pow - 1, false);
        len += 1;
        pow *= 2;
        if pow >= len {
            break;
        }
    }
    pow = 1;
    while pow < input.len() {
        let tmp = control_value(pow, &input);
        input.replace(pow - 1, tmp);
        pow *= 2;
    }
    input
}

fn corrupt(input: &mut BitVec<u8>) {
    let ind = rand::thread_rng().gen_range(0..input.len());
    let tmp = !input[ind];
    input.replace(ind, tmp);
}

fn hemming_fix(bits: &mut BitVec<u8>) -> BitVec<u8> {
    let mut result = bits.clone();
    let mut pow: usize = 1;

    while pow < result.len() {
        result.replace(pow - 1, false);
        pow *= 2;
    }

    pow = 1;
    let mut corrupted = false;
    let mut rev = 0;

    while pow < result.len() {
        let tmp = control_value(pow, &result);
        result.set(pow - 1, tmp);
        if tmp != bits[pow - 1] {
            corrupted = true;
            rev += pow;
            result.set(pow - 1, !tmp);
        }
        pow *= 2;
    }

    if corrupted {
        let val = !result[rev - 1];
        result.replace(rev - 1, val);
    }
    result
}

fn rm_control_bits(bits: &BitVec<u8>) -> BitVec<u8> {
    let mut input = bits.clone();
    let mut offset = 1;
    let mut pow = 1;
    while pow < bits.len() {
        input.remove(pow - offset);
        offset += 1;
        pow *= 2;
    }
    input
}

fn string_from_bitvec(bits: &BitVec<u8>) -> String {
    String::from_utf8(bits.clone().into_vec()).unwrap()
}

fn string_from_hemming_bits(bits: &BitVec<u8>) -> String {
    string_from_bitvec(&rm_control_bits(&bits))
}

fn main() {
    // let mut bv: BitVec<u8> = BitVec::new();
    // vec![0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1];
    // //   0  1  0  0  0  1  0  0  0  0  1  1  1  1  0  1
    // for i in vec![0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1] {
    //     if i == 0 {
    //         bv.push(false);
    //     } else {
    //         bv.push(true);
    //     }
    // }
    // let mut s = String::new();
    // for i in hemming_encode(&mut bv) {
    //     if i {
    //         s.push('1');
    //     } else {
    //         s.push('0');
    //     }
    // }
    // println!("{s}");
    // println!("100110000100001011101")

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read string");

    let mut bits = hemming_encode(&string_to_bitvec(&input));
    let copy = bits.clone();
    corrupt(&mut bits);

    println!("corrupted string:\n{}", string_from_hemming_bits(&bits));

    let fixed = hemming_fix(&mut bits);
    assert_eq!(copy, fixed);
    let output = string_from_hemming_bits(&fixed);
    assert_eq!(output, input);
    println!("{output}");
}
