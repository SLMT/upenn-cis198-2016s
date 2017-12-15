#![cfg(test)]

use problem1::{sum, dedup, filter};
use problem2::mat_mult;
use problem3::sieve;
use problem4::{hanoi, Peg};

#[test]
fn test_sum() {
    let array = [1,10,100,1000,10000,100000];
    assert_eq!(sum(&array), 111111);
}

#[test]
fn test_dedup() {
    let vs = vec![5,7,1,3,6,7,15,7,8,21,6,8,1,5,6,2,63,51,1];
    assert_eq!(dedup(&vs), vec![5,7,1,3,6,15,8,21,2,63]);
}

fn positive(x: i32) -> bool {
    (x % 2) == 0
}

#[test]
fn test_filter() {
    let vs = vec![1,20,6,-123,518,-15,-1256,-215213,8902,-11,160123,123123];
    assert_eq!(filter(&vs, &positive), vec![1,20,6,518,8902,160123,123123]);
}

#[test]
fn test_mat_mult() {
    let mut mat1 = vec![vec![0.;3]; 3];
    for i in 0..mat1.len() {
        mat1[i][i] = 1.;
    }
    let mat2 = vec![vec![5.;3]; 3];
    let result = mat_mult(&mat1, &mat2);
    for i in 0..result.len() {
        for j in 0..result[i].len() {
            assert_eq!(result[i][j], mat2[i][j]);
        }
    }
}

#[test]
fn test_sieve_basic() {
    assert_eq!(vec![2,3,5,7,11], sieve(12));
}

#[test]
fn test_hanoi_1_disks() {
    let result = hanoi(1, Peg::A, Peg::B, Peg::C);
    assert_eq!(vec![(Peg::A, Peg::C)], result);
    assert_eq!(1, result.len());
}
