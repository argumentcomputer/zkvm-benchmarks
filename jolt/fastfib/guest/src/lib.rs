#![cfg_attr(feature = "guest", no_std)]
#![no_main]

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

fn fast_matexp(b: Matrix2x2, e: u64) -> Matrix2x2 {
    if e == 0 {
        [1, 0, 0, 1] // identity matrix
    } else if e % 2 == 1 {
        // odd?
        matmul(b, fast_matexp(matmul(b, b), (e - 1) / 2))
    } else {
        fast_matexp(matmul(b, b), e / 2)
    }
}

#[jolt::provable]
fn fastfib(n: u64) -> u64 {
    fast_matexp([0, 1, 1, 1], n + 1)[0]
}
