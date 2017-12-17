
/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    let mut is_prime: Vec<bool> = vec![true; n as usize];
    let mut result: Vec<u32> = vec![];

    for num in 2..n {
        if !is_prime[num as usize] {
            continue;
        }

        result.push(num);

        let mut mul: u32 = num;
        let mut not_prime: u32 = mul * num;
        while not_prime < n {
            is_prime[not_prime as usize] = false;
            mul += 1;
            not_prime = mul * num;
        }
    }

    result
}
