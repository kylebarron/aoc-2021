use std::fs;

fn read_file() -> String {
    let filename = "./input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}

fn main() {
    let text = read_file();
    let lines = text.split('\n');

    let mut depth: i32 = 0;
    let mut horiz: i32 = 0;

    for line in lines {
        let mut split = line.split(' ');
        let (word, number) = (split.next().unwrap(), split.next().unwrap());

        if word == "forward" {
            horiz += number.parse::<i32>().unwrap();
        }

        else if word == "down" {
            depth += number.parse::<i32>().unwrap();
        }

        else if word == "up" {
            depth -= number.parse::<i32>().unwrap();
        }
    }

    println!("{}", depth * horiz);
}
