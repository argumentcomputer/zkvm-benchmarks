use risc0_zkvm::guest::env;

fn main() {
    let n: u64 = env::read();

    let result = fib(n);

    env::commit(&(n, result));
}

// [0, 1, 2, 3] = |0 1|
//                |2 3|
type Matrix2x2 = [u64; 4];

fn matmul(a: Matrix2x2, b: Matrix2x2) -> Matrix2x2 {
    [
        a[0] * b[0] + a[1] * b[2],
        a[0] * b[1] + a[1] * b[3],
        a[1] * b[0] + a[3] * b[2],
        a[2] * b[1] + a[3] * b[3],
    ]
}

fn fast_matexp(mut b: Matrix2x2, mut e: u64) -> Matrix2x2 {
    let mut acc = [1, 0, 0, 1]; // identity matrix

    while e > 0 {
        if e % 2 == 1 {
            // odd?
            acc = matmul(b, acc);
            e = (e - 1) / 2;
        } else {
            e = e / 2;
        }
        b = matmul(b, b);
    }
    acc
}

fn fib(n: u64) -> u64 {
    fast_matexp([0, 1, 1, 1], n + 1)[0]
}
