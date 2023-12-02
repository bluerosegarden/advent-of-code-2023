fn main() {
    println!("Hello, world!");
}

fn part_1(input: &str) -> i32 {
    let mut line_sum: Vec<i32> = Vec::new();
    for line in input.lines() {
        let mut int_vec: Vec<u32> = Vec::new();
        for chr in line.chars() {
            let digit = chr.to_digit(10);
            match digit {
                Some(digit) => int_vec.push(digit.try_into().unwrap()),
                None => (),
            };
        }
        match int_vec.last() {
            Some(last_num) => {
                let first: &u32 = &int_vec[0];
                let last: &u32 = last_num;
                let combined = format!("{}{}", first, last);
                let combined_i32: i32 = combined.parse::<i32>().unwrap();

                line_sum.push(combined_i32);
            }
            None => (),
        }
    }
    return line_sum.iter().sum();
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1_test() {
        use super::*;
        let input: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = part_1(input);
        println!("{}", result);
        assert!(result == 142);
    }

    // #[test]
    // fn part_2() {
    //     panic!("Make this test fail");
    // }
}
