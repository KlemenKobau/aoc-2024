use crate::util::read_lines;

pub fn three() {
    let lines = read_lines("data/day-3/input.txt");
    let input = lines.join("");

    let match_indices = input.match_indices("mul(");

    let mut sum = 0;

    'outer: for (index, _) in match_indices {
        let mut first_num = Vec::new();

        let mut match_iterator = input.chars().skip(index + 4);

        loop {
            let letter = match_iterator.next();
            println!("{:?}", letter);
            if letter.is_none() {
                break 'outer;
            }
            let letter = letter.unwrap();

            if letter.is_numeric() {
                first_num.push(letter);
            } else if letter == ',' {
                break;
            } else {
                continue 'outer;
            }
        }

        println!("First num: {:?}", &first_num);

        let mut second_num = Vec::new();

        loop {
            let letter = match_iterator.next();
            if letter.is_none() {
                break 'outer;
            }
            let letter = letter.unwrap();

            if letter.is_numeric() {
                second_num.push(letter);
            } else if letter == ')' {
                break;
            } else {
                continue 'outer;
            }
        }

        let first_num: i32 = String::from_iter(first_num).parse().unwrap();
        let second_num: i32 = String::from_iter(second_num).parse().unwrap();

        sum += first_num * second_num;
    }

    println!("{}", sum);
}
