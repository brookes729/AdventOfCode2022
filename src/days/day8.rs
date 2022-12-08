use crate::helpers::{file_helper::read_file_to_string_vector, parse_helper::string_to_number};

pub fn run_day() {
    let file_contents = read_file_to_string_vector("day8.txt");

    let mut trees: Vec<Vec<u32>> = Vec::new();
    let mut visible: Vec<Vec<bool>> = Vec::new();

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
            visible_row.push(tree_visible(&trees, tree_row, tree_column));
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

    println!("Part one: {}", visible_count)
}

fn tree_visible(trees: &Vec<Vec<u32>>, tree_row: usize, tree_column: usize) -> bool {
    let tree_value = trees[tree_row][tree_column];
    tree_visible_north(&trees, tree_value, tree_row, tree_column)
        || tree_visible_south(&trees, tree_value, tree_row, tree_column)
        || tree_visible_east(&trees, tree_value, tree_row, tree_column)
        || tree_visible_west(&trees, tree_value, tree_row, tree_column)
}

fn tree_visible_north(
    trees: &Vec<Vec<u32>>,
    tree_value: u32,
    tree_row: usize,
    tree_column: usize,
) -> bool {
    if tree_row == 0 {
        true
    } else if trees[tree_row][tree_column] > trees[tree_row - 1][tree_column] {
        tree_visible_north(trees, tree_value, tree_row - 1, tree_column)
    } else {
        false
    }
}
fn tree_visible_south(
    trees: &Vec<Vec<u32>>,
    tree_value: u32,
    tree_row: usize,
    tree_column: usize,
) -> bool {
    if tree_row == trees.len() - 1 {
        true
    } else if tree_value > trees[tree_row + 1][tree_column] {
        tree_visible_south(trees, tree_value, tree_row + 1, tree_column)
    } else {
        false
    }
}
fn tree_visible_east(
    trees: &Vec<Vec<u32>>,
    tree_value: u32,
    tree_row: usize,
    tree_column: usize,
) -> bool {
    if tree_column == trees[tree_row].len() - 1 {
        true
    } else if tree_value > trees[tree_row][tree_column + 1] {
        tree_visible_east(trees, tree_value, tree_row, tree_column + 1)
    } else {
        false
    }
}
fn tree_visible_west(
    trees: &Vec<Vec<u32>>,
    tree_value: u32,
    tree_row: usize,
    tree_column: usize,
) -> bool {
    if tree_column == 0 {
        true
    } else if tree_value > trees[tree_row][tree_column - 1] {
        tree_visible_west(trees, tree_value, tree_row, tree_column - 1)
    } else {
        false
    }
}
