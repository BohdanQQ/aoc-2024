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
            let iter = c.iter();
            let mut found = 0;
            for i in 0..iter.clone().len() {
                if i + needle.clone().count() > iter.clone().len() {
                    continue;
                }
                let mut ok = true;
                for j in 0..needle.clone().count() {
                    if iter.clone().nth(i + j) != needle.clone().nth(j).as_ref() {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    found += 1;
                }
            }
            // this could be cleverly done in the loop above... idc
            let iter = c.iter().rev();
            for i in 0..iter.clone().count() {
                if i + needle.clone().count() > iter.clone().count() {
                    continue;
                }
                let mut ok = true;
                for j in 0..needle.clone().count() {
                    if iter.clone().nth(i + j) != needle.clone().nth(j).as_ref() {
                        ok = false;
                        break;
                    }
                }
                if ok {
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

pub fn extract_signature(inp: &[Vec<char>], i: usize, j: usize) -> Option<String> {
    if i + 1 >= inp.len() || i < 1 || j < 1 || j + 1 >= inp[i].len() {
        None
    } else {
        Some(
            "".to_owned()
                + &inp[i - 1][j - 1].to_string()
                + &inp[i - 1][j + 1].to_string()
                + &inp[i + 1][j - 1].to_string()
                + &inp[i + 1][j + 1].to_string(),
        )
    }
}

pub fn get_mas_cross(inp: &[Vec<char>]) -> u32 {
    let mut res = 0;
    for i in 0..inp.len() {
        for j in 0..inp[i].len() {
            if inp[i][j] != 'A' {
                continue;
            }
            if let Some(sig) = extract_signature(inp, i, j) {
                if sig == "MMSS" || sig == "MSMS" || sig == "SSMM" || sig == "SMSM" {
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
