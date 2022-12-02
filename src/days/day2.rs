use crate::helpers::file_helper::read_file_to_string_vector;

pub fn run_day() {
    let file_contents = read_file_to_string_vector("day2.txt");

    let mut score_one = 0;
    let mut score_two = 0;

    for line in file_contents {
        match line.as_str() {
            "A X" =>
            // A - Rock: 1: Rock - Draw, 2: loss - scissors
            {
                score_one += 1 + 3;
                score_two += 3 + 0;
            }
            "A Y" =>
            // A - Rock: 1: Paper - Win, 2: Draw - Rock
            {
                score_one += 2 + 6;
                score_two += 1 + 3;
            }
            "A Z" =>
            // A - Rock: Scissors - Loss, 2 win - paper
            {
                score_one += 3 + 0;
                score_two += 2 + 6;
            }
            "B X" =>
            // B - Paper: Rock - Loss, same
            {
                score_one += 1 + 0;
                score_two += 1 + 0;
            }
            "B Y" =>
            // B - Paper: Paper - Draw, same
            {
                score_one += 2 + 3;
                score_two += 2 + 3;
            }
            "B Z" =>
            // B - Paper: Scissors - Win, same
            {
                score_one += 3 + 6;
                score_two += 3 + 6;
            }
            "C X" =>
            // C - Scissors: Rock - Win, loss Paper
            {
                score_one += 1 + 6;
                score_two += 2 + 0;
            }
            "C Y" =>
            // C - Scissors: Paper - Loss, 2 draw - scissors
            {
                score_one += 2 + 0;
                score_two += 3 + 3;
            }
            "C Z" =>
            // C - Scissors: Scissors - Draw, win - rock
            {
                score_one += 3 + 3;
                score_two += 1 + 6;
            }

            _ => panic!("Missed one somehow!"),
        }
    }

    println!("Part 1 Score: {}", score_one);
    println!("Part 2 Score: {}", score_two);
}
