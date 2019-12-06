fn main() {
    let mut part1_count = 0;
    let mut part2_count = 0;
    for v in 145852..616942 {
        let s = format!("{}", v);

        if is_valid_part1(&s) {
            part1_count = part1_count + 1;
        }

        if is_valid_part2(&s) {
            part2_count = part2_count + 1;
        }
    }
    println!("count part1={}", part1_count);
    println!("count part2={}", part2_count);
}

fn is_valid_part1(pw:&str) -> bool {
    let mut last_value = 0;
    let mut has_pair = false;
    for pos in 0..pw.len() {
        let value:usize = pw[pos..pos+1].parse().unwrap();
        
        if value < last_value {
            return false;
        } else if value == last_value {
            has_pair = true;
        }

        last_value = value;
    }

    has_pair
}


fn is_valid_part2(pw:&str) -> bool {
    let mut values: Vec<(usize, usize)> = Vec::new();
    
    for pos in 0..pw.len() {
        let value:usize = pw[pos..pos+1].parse().unwrap();

        if values.len() == 0 {
            values.push((value, 1));
        } else {
            let (last_value, last_count) = values.pop().unwrap();

            if value < last_value {
                return false;
            } else if value == last_value {
                values.push((last_value, last_count + 1));
            } else {
                values.push((last_value, last_count));
                values.push((value, 1));
            }
        }
    }


    let mut has_pair = false;
    for (_, count) in values.iter() {
        if count == &2 {
            has_pair = true;
        }
    }

    has_pair
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_part1() {
        assert_eq!(true, is_valid_part1("111111"));
        assert_eq!(true, is_valid_part1("122345"));
        assert_eq!(false, is_valid_part1("223450"));
        assert_eq!(false, is_valid_part1("123789"));
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(false, is_valid_part2("111111"));
        assert_eq!(true, is_valid_part2("122345"));
        assert_eq!(true, is_valid_part2("112233"));
        assert_eq!(false, is_valid_part2("112232"));
        assert_eq!(false, is_valid_part2("223450"));
        assert_eq!(false, is_valid_part2("123789"));
        assert_eq!(false, is_valid_part2("123444"));
    }

}