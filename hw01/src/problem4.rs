
/// #[derive(...)] statements define certain properties on the enum for you for
/// free (printing, equality testing, the ability to copy values). More on this
/// when we cover Enums in detail.

/// You can use any of the variants of the `Peg` enum by writing `Peg::B`, etc.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Peg {
    A,
    B,
    C,
}

/// A move between two pegs: (source, destination).
pub type Move = (Peg, Peg);

/// Solves for the sequence of moves required to move all discs from `src` to
/// `dst`.
pub fn hanoi(num_discs: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {
    let mut result: Vec<Move> = vec![];
    hanoi_impl(num_discs, src, aux, dst, &mut result);
    result
}

fn hanoi_impl(n: u32, src: Peg, aux: Peg, dst: Peg, result: &mut Vec<Move>) {
    if n == 1 {
        result.push((src, dst));
    } else {
        hanoi_impl(n - 1, src, dst, aux, result);
        result.push((src, dst));
        hanoi_impl(n - 1, aux, src, dst, result);
    }
}
