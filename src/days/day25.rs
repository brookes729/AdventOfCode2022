use crate::helpers::file_helper::read_file_to_string_vector;

pub fn run_day() {
    let file_contents = read_file_to_string_vector("day25.txt");

    //let mut fuel: Vec<String> = Vec::new();
    let mut running_total = 0;
    for line in file_contents {
        let number = de_snafu(line.clone());
        running_total += number;
        let snaffud = re_snafu(number);
        println!("{} -> {} -> {}", line, number, snaffud);
    }

    let snaffud = re_snafu(running_total);
    println!("Part One: {} -> {}", running_total, snaffud);
}

fn re_snafu(number: i64) -> String {
    let mut running_total = number;
    let mut i = 0;
    let base: i64 = 5;
    let mut output: Vec<char> = vec![];
    while running_total >= base.pow(i) {
        match running_total % base.pow(i + 1) / base.pow(i) {
            3 => {
                output.push('=');
                running_total += base.pow(i + 1)
            }
            4 => {
                output.push('-');
                running_total += base.pow(i + 1)
            }
            0 => {
                output.push('0');
            }
            1 => {
                output.push('1');
            }
            2 => {
                output.push('2');
            }
            _ => (),
        }
        i += 1;
    }

    output.reverse();
    output.into_iter().collect()
}

fn de_snafu(line: String) -> i64 {
    let base: i64 = 5;
    let line_length = line.len() - 1;
    line.chars()
        .enumerate()
        .map(|(i, c)| match c {
            '=' => -2 as i64 * base.pow((line_length - i) as u32),
            '-' => -1 as i64 * base.pow((line_length - i) as u32),
            '1' => 1 as i64 * base.pow((line_length - i) as u32),
            '2' => 2 as i64 * base.pow((line_length - i) as u32),
            _ => 0,
        })
        .sum()
}
