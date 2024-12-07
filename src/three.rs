use crate::util::read_lines;

pub fn three() {
    let lines = read_lines("data/day-3/input.txt");
    let input = lines.join("");

    let mut match_indices: Vec<i32> = input.match_indices("mul(").map(|x| x.0 as i32).collect();

    let dos: Vec<usize> = input.match_indices("do()").map(|x| x.0).collect();
    let mut dos: Vec<(i32, Order)> = dos.into_iter().map(|x| (x as i32, Order::Do)).collect();

    let donts: Vec<usize> = input.match_indices("don't()").map(|x| x.0).collect();
    let mut donts: Vec<(i32, Order)> = donts.into_iter().map(|x| (x as i32, Order::Dont)).collect();

    let mut orders = Vec::new();
    orders.push((-1, Order::Do));
    orders.append(&mut dos);
    orders.append(&mut donts);
    orders.sort_by(|(index1, _), (index2, _)| index1.cmp(index2).reverse());

    let mut last_order = orders.pop().unwrap();
    let mut next_order = orders.pop();

    let mut to_remove = Vec::new();
    for (index, match_indice) in match_indices.iter().enumerate() {
        if let Some(order) = &next_order {
            if match_indice > &order.0 {
                last_order = next_order.unwrap();
                next_order = orders.pop();
            }
        }

        match last_order.1 {
            Order::Do => {}
            Order::Dont => to_remove.push(index),
        }
    }

    to_remove.reverse();

    for ele in to_remove {
        match_indices.remove(ele);
    }

    let mut sum = 0;

    'outer: for index in match_indices.iter() {
        let mut first_num = Vec::new();

        let mut match_iterator = input.chars().skip((index + 4).try_into().unwrap());

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

enum Order {
    Do,
    Dont,
}
