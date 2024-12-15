advent_of_code::solution!(13);


// ! solves system:
// ax * A + bx * B = res_x
// ay * A + by * B = res_y
pub fn solve_eqation(
    ax: i64,
    bx: i64,
    res_x: i64,
    ay: i64,
    by: i64,
    res_y: i64,
) -> Option<(i64, i64)> {
    // ! / ax
    // A  = res_x / ax - bx * B / ax

    // ! subst A from above
    // ay * res_ x / ax - ay * bx * B / ax + by * B = res_y
    // ! * ax
    // ay * res_x - ay * bx * B + by * ax * B = res_y * ax
    // ! extract B
    // B * (by ax - ay bx) = resy ax - resx ay
    // B = (resy ax - resx ay) / (by ax - ay bx)

    let aux_1 = res_y * ax - res_x * ay;
    let aux_2 = by * ax - ay * bx;
    if aux_2 == 0 || ax == 0 || aux_1 % aux_2 != 0 {
        None
    } else {
        let b = aux_1 / aux_2;
        let aux = res_x - bx * b;
        if aux % ax != 0 {
            None
        } else {
            Some((aux / ax, b))
        }
    }
}

fn helper_extract(s: &str, delim: char) -> i64 {
    s.split(delim)
        .nth(1)
        .unwrap()
        .trim_end_matches(',')
        .parse()
        .unwrap()
}

fn get_num(s: &str) -> i64 {
    if s.contains("+") {
        helper_extract(s, '+')
    } else if s.contains('=') {
        helper_extract(s, '=')
    } else {
        panic!("invalid {}", s);
    }
}

fn part_with_shift(input: &str, shift: i64) -> i64 {
    let nums = input
        .split_ascii_whitespace()
        .filter(|s| s.starts_with("X") || s.starts_with("Y"))
        .collect::<Vec<_>>();

    let buttons = nums
        .iter()
        .filter(|s| !s.starts_with("X=") && !s.starts_with("Y="))
        .collect::<Vec<_>>();
    let prizes = nums
        .iter()
        .filter(|s| s.starts_with("X=") || s.starts_with("Y="))
        .collect::<Vec<_>>();

    let mut res = 0;

    for prize_idx in 0..(prizes.len() / 2) {
        let button_a_idx = prize_idx * 4;
        let button_b_idx = button_a_idx + 2;
        let (sax, say) = (buttons[button_a_idx], buttons[button_a_idx + 1]);
        let (sbx, sby) = (buttons[button_b_idx], buttons[button_b_idx + 1]);
        let (pr_x, pr_y) = (prizes[prize_idx * 2], prizes[prize_idx * 2 + 1]);

        let (ax, ay) = (get_num(sax), get_num(say));
        let (bx, by) = (get_num(sbx), get_num(sby));
        let (res_x, res_y) = (get_num(pr_x), get_num(pr_y));

        if let Some((a_count, b_count)) =
            solve_eqation(ax, bx, res_x + shift, ay, by, res_y + shift)
        {
            res += a_count * 3 + b_count;
        }
    }
    res
}

pub fn part_one(input: &str) -> Option<i64> {
    Some(part_with_shift(input, 0))
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(part_with_shift(input, 10000000000000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
