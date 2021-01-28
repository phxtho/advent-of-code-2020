/*From https://adventofcode.com/2020/day/3 */

use std::convert::TryFrom;

fn main () {
    let input = 
    "..##.......
    #...#...#..
    .#....#..#.
    ..#.#...#.#
    .#...##..#.
    ..#.##.....
    .#.#.#....#
    .#........#
    #.##...#...
    #...##....#
    .#..#...#.#";

    
    let map = str_to_vec2d(input);
    let tree_count = number_of_trees_on_slope(&map, (3,1));
    println!("Tree count = {}",tree_count);
}

fn str_to_vec2d(input_str: &str) -> Vec<Vec<char>> {
    let mut vec2d: Vec<Vec<char>> = Vec::new();
    for mut row in input_str.lines() {
        row = row.trim();
        let char_vec: Vec<char> = row.chars().collect();
        vec2d.push(char_vec);
    }
    return vec2d;
}

fn number_of_trees_on_slope (map : &Vec<Vec<char>>, slope : (i32,i32)) -> i32 {
    let map_height : i32 = i32::try_from(map.len()).unwrap() -1;
    let map_width: i32 = i32::try_from(map[0].len()).unwrap() -1;
    let mut tree_count = 0;

    if map_width > 0  && map_height > 0 {

        let mut pos = (0,0);
        let mut journey_complete = false;
        while !journey_complete {
            // Update step
            pos.0 = (pos.0 + slope.0) % map_width; // Account for map wrap around 
            pos.1 += slope.1;
            // Check for completion
            if pos.1 >= map_height {
                journey_complete = true;
                return tree_count;   
            }
            println!("(x:{},y:{}): {:?}", pos.0,pos.1,map[pos.1 as usize][pos.0 as usize]);
            // Check for tree
            if map[pos.1 as usize][pos.0 as usize] == '#' {
                tree_count += 1;
            }
            
        }
    } else {
        tree_count = -1
    }
    
    return tree_count;   
}