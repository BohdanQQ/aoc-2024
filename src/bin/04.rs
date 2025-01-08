use std::str::Chars;

advent_of_code::solution!(4);
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn find_all_horizontal(cs: Vec<Vec<char>>, needle: Chars<'_>) -> u32 {
    cs.iter()
        .map(|c| {
            let mut found = 0;
            for window in c.windows(needle.clone().count()) {
                if needle.clone().zip(window.iter()).all(|(c1, c2)| c1 == *c2) {
                    found += 1;
                }
                if needle
                    .clone()
                    .zip(window.iter().rev())
                    .all(|(c1, c2)| c1 == *c2)
                {
                    found += 1;
                }
            }
            found
        })
        .sum()
}

pub fn get_diag_idx(inp: &[Vec<char>], idx1: usize, idx2: usize, len: usize) -> Vec<char> {
    let mut result = vec![];
    for i in 0..len {
        if idx1 + i >= inp.len() || idx2 + i >= inp[idx1 + i].len() {
            break;
        }
        result.push(inp[idx1 + i][idx2 + i]);
    }
    result
}

pub fn get_diag_idx_2(inp: &[Vec<char>], idx1: usize, idx2: usize, len: usize) -> Vec<char> {
    let mut result = vec![];
    for i in 0..len {
        if idx1 + i >= inp.len() || i > idx2 || idx2 - i >= inp[idx1 + i].len() {
            break;
        }
        result.push(inp[idx1 + i][idx2 - i]);
    }
    result
}

pub fn get_diagonals(inp: &[Vec<char>], size: usize) -> Vec<Vec<char>> {
    let mut result = vec![];
    for diag_idx in 0..size {
        result.push(get_diag_idx(inp, 0, diag_idx, size - diag_idx));
    }
    for diag_idx in 1..size {
        result.push(get_diag_idx(inp, diag_idx, 0, size - diag_idx));
    }
    for diag_idx in 0..size {
        result.push(get_diag_idx_2(inp, 0, diag_idx, diag_idx + 1));
    }
    for diag_idx in 1..size {
        result.push(get_diag_idx_2(inp, diag_idx, size - 1, size - diag_idx));
    }
    result
}

// packs 4 (supposedly 8-bit) chars into a single int
fn char_signature(chars: [char; 4]) -> u64 {
    chars[0] as u64
        + ((chars[1] as u64) << 8)
        + ((chars[2] as u64) << 16)
        + ((chars[3] as u64) << 24)
}

fn extract_char_signature(inp: &[Vec<char>], i: usize, j: usize) -> Option<u64> {
    if i + 1 >= inp.len() || i < 1 || j < 1 || j + 1 >= inp[i].len() {
        None
    } else {
        Some(char_signature([
            inp[i - 1][j - 1],
            inp[i - 1][j + 1],
            inp[i + 1][j - 1],
            inp[i + 1][j + 1],
        ]))
    }
}

pub fn get_mas_cross(inp: &[Vec<char>]) -> u32 {
    let mut res = 0;
    let sig1 = char_signature(['M', 'M', 'S', 'S']);
    let sig2 = char_signature(['M', 'S', 'M', 'S']);
    let sig3 = char_signature(['S', 'S', 'M', 'M']);
    let sig4 = char_signature(['S', 'M', 'S', 'M']);
    for i in 0..inp.len() {
        for j in 0..inp[i].len() {
            if inp[i][j] != 'A' {
                continue;
            }
            if let Some(sig) = extract_char_signature(inp, i, j) {
                if sig == sig1 || sig == sig2 || sig == sig3 || sig == sig4 {
                    res += 1;
                }
            }
        }
    }
    res
}

pub fn part_one(input: &str) -> Option<u32> {
    let vecs = input
        .split_ascii_whitespace()
        .filter(|v| !v.is_empty())
        .map(|v| Vec::from_iter(v.chars()))
        .collect::<Vec<_>>();
    let row_len = input.split_ascii_whitespace().next().unwrap().len();
    let transposed = transpose(vecs.clone());
    let diags = get_diagonals(&vecs, row_len);
    let needle = "XMAS".chars();
    Some(
        find_all_horizontal(vecs, needle.clone())
            + find_all_horizontal(transposed, needle.clone())
            + find_all_horizontal(diags, needle.clone()),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let vecs = input
        .split_ascii_whitespace()
        .filter(|v| !v.is_empty())
        .map(|v| Vec::from_iter(v.chars()))
        .collect::<Vec<_>>();

    Some(get_mas_cross(&vecs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
