use crate::helpers::{file_helper::read_file_to_string_vector, parse_helper::string_to_number};

pub fn run_day() {
    let file_contents = read_file_to_string_vector("day8.txt");

    let mut trees: Vec<Vec<u32>> = Vec::new();
    let mut visible: Vec<Vec<bool>> = Vec::new();
    let mut part_two_score = 0;

    for line in file_contents {
        let mut row_of_trees: Vec<u32> = Vec::new();
        for char in line.chars() {
            row_of_trees.push(string_to_number(char.to_string()))
        }
        trees.push(row_of_trees.clone());
    }

    for tree_row in 0..=trees.len() - 1 {
        let mut visible_row: Vec<bool> = Vec::new();
        for tree_column in 0..=trees[tree_row].len() - 1 {
            let (visible, score) = tree_visible(&trees, tree_row, tree_column);
            visible_row.push(visible);
            if score > part_two_score {
                part_two_score = score;
            }
        }
        visible.push(visible_row);
    }

    let visible_count: u32 = visible
        .into_iter()
        .map(|vr| {
            (vr.into_iter()
                .map(|vt| if vt { 1 } else { 0 })
                .collect::<Vec<u32>>())
            .into_iter()
            .sum::<u32>()
        })
        .sum();

    println!("Part one: {}", visible_count);
    println!("Part two: {}", part_two_score);
}

fn tree_visible(trees: &Vec<Vec<u32>>, tree_row: usize, tree_column: usize) -> (bool, u32) {
    let tree_value = trees[tree_row][tree_column];
    let (tree_visible_n, tree_score_n) =
        tree_visible_north(&trees, tree_value, tree_row, tree_column);
    let (tree_visible_s, tree_score_s) =
        tree_visible_south(&trees, tree_value, tree_row, tree_column);
    let (tree_visible_e, tree_score_e) =
        tree_visible_east(&trees, tree_value, tree_row, tree_column);
    let (tree_visible_w, tree_score_w) =
        tree_visible_west(&trees, tree_value, tree_row, tree_column);

    (
        tree_visible_n || tree_visible_s || tree_visible_e || tree_visible_w,
        tree_score_n * tree_score_s * tree_score_e * tree_score_w,
    )
}

fn tree_visible_north(
    trees: &Vec<Vec<u32>>,
    tree_value: u32,
    tree_row: usize,
    tree_column: usize,
) -> (bool, u32) {
    if tree_row == 0 {
        (true, 0)
    } else if tree_value > trees[tree_row - 1][tree_column] {
        let next_tree = tree_visible_north(trees, tree_value, tree_row - 1, tree_column);
        (next_tree.0, 1 + next_tree.1)
    } else {
        (false, 1)
    }
}
fn tree_visible_south(
    trees: &Vec<Vec<u32>>,
    tree_value: u32,
    tree_row: usize,
    tree_column: usize,
) -> (bool, u32) {
    if tree_row == trees.len() - 1 {
        (true, 0)
    } else if tree_value > trees[tree_row + 1][tree_column] {
        let next_tree = tree_visible_south(trees, tree_value, tree_row + 1, tree_column);
        (next_tree.0, 1 + next_tree.1)
    } else {
        (false, 1)
    }
}
fn tree_visible_east(
    trees: &Vec<Vec<u32>>,
    tree_value: u32,
    tree_row: usize,
    tree_column: usize,
) -> (bool, u32) {
    if tree_column == trees[tree_row].len() - 1 {
        (true, 0)
    } else if tree_value > trees[tree_row][tree_column + 1] {
        let next_tree = tree_visible_east(trees, tree_value, tree_row, tree_column + 1);
        (next_tree.0, 1 + next_tree.1)
    } else {
        (false, 1)
    }
}
fn tree_visible_west(
    trees: &Vec<Vec<u32>>,
    tree_value: u32,
    tree_row: usize,
    tree_column: usize,
) -> (bool, u32) {
    if tree_column == 0 {
        (true, 0)
    } else if tree_value > trees[tree_row][tree_column - 1] {
        let next_tree = tree_visible_west(trees, tree_value, tree_row, tree_column - 1);
        (next_tree.0, 1 + next_tree.1)
    } else {
        (false, 1)
    }
}
