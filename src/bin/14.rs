advent_of_code::solution!(14);

type Pair = (i64, i64);

fn step_robot(robot_pos: &mut (i64, i64), robot_speed: &(i64, i64), bound_x: i64, bound_y: i64) {
    let (mut new_x, mut new_y) = (robot_pos.0 + robot_speed.0, robot_pos.1 + robot_speed.1);
    if new_x > 0 {
        new_x %= bound_x;
    } else if new_x < 0 {
        new_x += bound_x;
    }
    if new_y > 0 {
        new_y %= bound_y;
    } else if new_y < 0 {
        new_y += bound_y;
    }
    robot_pos.0 = new_x;
    robot_pos.1 = new_y;
}

fn get_quadrant_idx(x: i64, y: i64, x_mid: i64, y_mid: i64) -> usize {
    if x < x_mid && y < y_mid {
        return 0;
    } else if x < x_mid {
        return 1;
    } else if y < y_mid {
        return 2;
    } else {
        return 3;
    }
}

fn get_score(positions: &mut [Pair], bound_x: i64, bound_y: i64) -> usize {
    let mut quadrant_count = [0, 0, 0, 0];
    //    let x_mid_ignored = bound_x % 2 == 1;
    let x_mid = (bound_x - 1) / 2;
    //    let y_mid_ignored = bound_y % 2 == 1;
    let y_mid = (bound_y - 1) / 2;
    for (x, y) in positions {
        if (x_mid == *x) || (y_mid == *y) {
            continue;
        }
        quadrant_count[get_quadrant_idx(*x, *y, x_mid, y_mid)] += 1;
    }

    quadrant_count.iter().product()
}

fn step_second(robots: &mut [(Pair, Pair)], bound_x: i64, bound_y: i64) {
    for (robot_pos, robot_speed) in robots {
        step_robot(robot_pos, robot_speed, bound_x, bound_y);
    }
}

fn print_robots(robots: &mut [(Pair, Pair)], bound_x: i64, bound_y: i64) {
    let mut fld = vec![vec!['.'; bound_x as usize]; bound_y as usize];
    for (pos, _) in robots {
        fld[pos.1 as usize][pos.0 as usize] = 'X';
    }
    for row in fld {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut robots = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|x| {
            let mut split = x.split_ascii_whitespace();
            (split.next().unwrap(), split.next().unwrap())
        })
        .map(|(pos, sp)| {
            let mut ps = pos.split(',');
            let mut ss = sp.split(',');
            (
                (
                    ps.next()
                        .unwrap()
                        .trim_start_matches("p=")
                        .parse::<i64>()
                        .unwrap(),
                    ps.next().unwrap().parse::<i64>().unwrap(),
                ),
                (
                    ss.next()
                        .unwrap()
                        .trim_start_matches("v=")
                        .parse::<i64>()
                        .unwrap(),
                    ss.next().unwrap().parse::<i64>().unwrap(),
                ),
            )
        })
        .collect::<Vec<_>>();

    let bound_x = 101; //11;
    let bound_y = 103; //7;

    for _ in 0..100 {
        step_second(&mut robots, bound_x, bound_y);
        print_robots(&mut robots, bound_x, bound_y);
    }
    Some(get_score(
        &mut robots.iter().map(|x| x.0).collect::<Vec<_>>(),
        bound_x,
        bound_y,
    ))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut robots = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|x| {
            let mut split = x.split_ascii_whitespace();
            (split.next().unwrap(), split.next().unwrap())
        })
        .map(|(pos, sp)| {
            let mut ps = pos.split(',');
            let mut ss = sp.split(',');
            (
                (
                    ps.next()
                        .unwrap()
                        .trim_start_matches("p=")
                        .parse::<i64>()
                        .unwrap(),
                    ps.next().unwrap().parse::<i64>().unwrap(),
                ),
                (
                    ss.next()
                        .unwrap()
                        .trim_start_matches("v=")
                        .parse::<i64>()
                        .unwrap(),
                    ss.next().unwrap().parse::<i64>().unwrap(),
                ),
            )
        })
        .collect::<Vec<_>>();

    let bound_x = 101; //11;
    let bound_y = 103; //7;

    for i in 0..10000 * 2 {
        step_second(&mut robots, bound_x, bound_y);
        println!("second {} VVVV", i + 1);
        print_robots(&mut robots, bound_x, bound_y);
    }
    // just look (XXXXXX)
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
