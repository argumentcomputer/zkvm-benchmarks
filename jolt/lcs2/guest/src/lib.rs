#![no_main]

fn lcs_dyn(s1: &str, s2: &str) -> String {
    let s1len = s1.len();
    let s2len = s2.len();

    let s1arr: Vec<char> = s1.chars().collect();
    let s2arr: Vec<char> = s2.chars().collect();

    let lcs_matrix = calculate_lcs_dyn(&s1arr, &s2arr, s1len, s2len);

    lcs_matrix[s1len][s2len].clone()
}

fn calculate_lcs_dyn(s1: &[char], s2: &[char], s1len: usize, s2len: usize) -> Vec<Vec<String>> {
    let mut lcs_matrix: Vec<Vec<String>> = vec![vec![String::new(); s2len + 1]; s1len + 1];

    for i in 1..=s1len {
        for j in 1..=s2len {
            if s1[i - 1] == s2[j - 1] {
                lcs_matrix[i][j] = format!("{}{}", lcs_matrix[i - 1][j - 1], s1[i - 1]);
            } else {
                let l1 = &lcs_matrix[i][j - 1];
                let l2 = &lcs_matrix[i - 1][j];
                lcs_matrix[i][j] = if l1.len() > l2.len() {
                    l1.clone()
                } else {
                    l2.clone()
                };
            }
        }
    }

    lcs_matrix
}

#[jolt::provable]
fn lcs2(input: (String, String)) -> String {
    lcs_dyn(&input.0, &input.1)
}
