use std::fs;

fn read_file() -> String {
    let filename = "./input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}

fn parse_to_integers(text: &str) -> Vec<i32> {
    let lines = text.split("\n");
    let integers: Vec<i32> = lines.map(|x| x.parse::<i32>().unwrap()).collect();
    integers
}

fn main() {
    let text = read_file();
    let integers = parse_to_integers(&text);

    let mut first: Option<i32> = None;
    let mut second: Option<i32> = None;
    let mut third: Option<i32> = None;
    let mut previous_sum : Option<i32> = None;
    let mut count: i32 = 0;

    for integer in integers {
        // Initialize first, second, and third while iterating over the vector
        if first == None {
            first = Some(integer);
            continue;
        }
        if second == None {
            second = Some(integer);
            continue;
        }
        if third == None {
            third = Some(integer);
            previous_sum = Some(first.unwrap() + second.unwrap() + third.unwrap());
            continue;
        }

        // Now first, second, and third are fully initialized
        first = second;
        second = third;
        third = Some(integer);
        if first.unwrap() + second.unwrap() + third.unwrap() > previous_sum.unwrap() {
            count += 1;
        }

        previous_sum = Some(first.unwrap() + second.unwrap() + third.unwrap());
    }

    println!("{}", count);
}
