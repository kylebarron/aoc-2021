use std::fs;

fn main() {
    let filename = "./input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut lines = contents.split("\n");

    let mut count = 0;
    let mut previous: i32 = lines.next().unwrap().parse().unwrap();
    for line in lines {
        let my_int: i32 = line.parse().unwrap();
        if my_int > previous {
            count += 1;
        }
        previous = my_int;
    }

    println!("{}", count);
}
