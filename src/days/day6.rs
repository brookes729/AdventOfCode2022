use crate::helpers::file_helper::read_file_to_string;

pub fn run_day() {
    let file_contents = read_file_to_string("day6.txt");

    let mut running_packets: Vec<char> = Vec::new();
    let mut score_one = 0;
    let mut score_two = 0;

    for packet in file_contents.chars() {
        score_one += 1;
        running_packets.push(packet);

        if running_packets.len() == 4 {
            let mut dedup_packets = running_packets.clone();
            dedup_packets.sort();
            dedup_packets.dedup();
            if running_packets.len() == dedup_packets.len() {
                break;
            }

            running_packets = Vec::from(&running_packets[1..]);
        }
    }

    // Part 2
    running_packets = Vec::new();
    for packet in file_contents.chars() {
        score_two += 1;
        running_packets.push(packet);

        if running_packets.len() == 14 {
            let mut dedup_packets = running_packets.clone();
            dedup_packets.sort();
            dedup_packets.dedup();
            if running_packets.len() == dedup_packets.len() {
                break;
            }

            running_packets = Vec::from(&running_packets[1..]);
        }
    }

    println!("Part 1: {}", score_one);
    println!("Part 2: {}", score_two);
}
