
/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    // Check the dimension, panic if they don't agree
    if mat1[0].len() != mat2.len() {
        panic!("the dimensions do not agree.");
    }

    let out_row = mat1.len();
    let in_dim = mat1[0].len();
    let out_col = mat2[0].len();

    let mut result: Matrix = vec![];

    for i in 0..out_row {
        let mut row: Vec<f32> = vec![];

        for j in 0..out_col {
            let mut sum: f32 = 0.0;

            for k in 0..in_dim {
                sum += mat1[i][k] * mat2[k][j];
            }

            row.push(sum);
        }

        result.push(row);
    }

    result
}
