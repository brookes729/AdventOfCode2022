use crate::helpers::file_helper::read_file_to_string;

pub fn run_day() {
    let mut score_one = 0;
    let mut score_two = 0;

    find_repeating_pattern(4, &mut score_one);
    find_repeating_pattern(14, &mut score_two);

    println!("Part 1: {}", score_one);
    println!("Part 2: {}", score_two);
}

fn find_repeating_pattern(duplicate_count: usize, score: &mut i32) {
    let file_contents = read_file_to_string("day6.txt");
    let mut running_packets: Vec<char> = Vec::new();

    for packet in file_contents.chars() {
        *score += 1;
        running_packets.push(packet);

        if running_packets.len() == duplicate_count {
            let mut dedup_packets = running_packets.clone();
            dedup_packets.sort();
            dedup_packets.dedup();
            if running_packets.len() == dedup_packets.len() {
                break;
            }

            running_packets = Vec::from(&running_packets[1..]);
        }
    }
}
