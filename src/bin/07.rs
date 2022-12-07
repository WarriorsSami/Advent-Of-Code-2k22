use std::cmp::min;
use indextree::{Arena, NodeId};

const MAX_DIR_DIM: u64 = 100000_u64;

#[derive(Eq, PartialEq, Debug, Clone)]
enum FileType {
    File,
    Directory,
}

#[derive(Clone)]
struct NodeData {
    pub tag: String,
    pub size: u64,
    pub file_type: FileType
}

fn compute_size_part_one(final_size: &mut u64, node: NodeId, arena: &Arena<NodeData>) -> u64 {
    let mut size = 0;
    for child in node.children(arena) {
        let child_size = compute_size_part_one(final_size, child, arena);
        size += child_size;
    }
    let node_data = arena.get(node).unwrap().get();
    size += node_data.size;
    if node_data.file_type == FileType::Directory
        && node_data.tag != "/"
        && size <= MAX_DIR_DIM {
        *final_size += size;
    }
    size
}

fn compute_arena_tree(input: &str) -> (NodeId, Arena<NodeData>) {
    let arena: &mut Arena<NodeData> = &mut Arena::new();
    let mut pwd_node: Option<NodeId> = None;
    let mut root_node: Option<NodeId> = None;

    for line in input.lines() {
        let first_char = line.chars().next().unwrap();

        match first_char {
            '$' => {
                let command_parts = line.split_whitespace().collect::<Vec<&str>>();
                let command_name = command_parts[1];
                match command_name {
                    "cd" => {
                        let command_arg_file_name = command_parts[2];
                        match command_arg_file_name {
                            ".." => {
                                pwd_node = pwd_node.unwrap().ancestors(arena).nth(1);
                            }
                            file_name => {
                                match pwd_node {
                                    Some(current) => {
                                        if let Some(child) = current
                                            .children(arena)
                                            .find(|child| {
                                                arena
                                                    .get(child.to_owned())
                                                    .unwrap()
                                                    .get()
                                                    .tag == file_name
                                            }) { pwd_node = Some(child); }
                                    }
                                    None => {
                                        pwd_node = arena.new_node(
                                            NodeData {
                                                tag: file_name.to_string(),
                                                size: 0,
                                                file_type: FileType::Directory
                                            })
                                            .into();
                                        root_node = pwd_node;
                                    }
                                }
                            }
                        }
                    }
                    "ls" => continue,
                    _ => panic!("Unknown command: {}", command_name)
                }
            }
            _ => {
                let output_parts = line.split_whitespace().collect::<Vec<&str>>();
                let (first_part, file_name) = (output_parts[0], output_parts[1]);
                match first_part {
                    "dir" => {
                        pwd_node
                            .unwrap()
                            .append(
                                arena.new_node(
                                    NodeData {
                                        tag: file_name.to_string(),
                                        size: 0,
                                        file_type: FileType::Directory
                                    }), arena);
                    }
                    size => {
                        let file_size = size.parse::<u64>().unwrap();
                        pwd_node
                            .unwrap()
                            .append(
                                arena.new_node(
                                    NodeData {
                                        tag: file_name.to_string(),
                                        size: file_size,
                                        file_type: FileType::File
                                    }), arena);
                    }
                }
            }
        }
    }

    (root_node.unwrap().to_owned(), arena.clone())
}

pub fn part_one(input: &str) -> Option<u64> {
    let (root_node, arena) = compute_arena_tree(input);
    let mut final_size = 0_u64;
    compute_size_part_one(&mut final_size, root_node, &arena);
    final_size.into()
}

const TOTAL_SIZE: u64 = 70000000_u64;
const UPDATE_SIZE: u64 = 30000000_u64;

fn compute_size_part_two(node: NodeId, arena: &mut Arena<NodeData>) -> u64 {
    let mut size = 0;
    for child in node.children(&arena.to_owned()) {
        let child_size = compute_size_part_two(child, arena);
        size += child_size;
    }
    let node_data: &mut NodeData = arena.get_mut(node).unwrap().get_mut();
    size += node_data.size;
    node_data.size = size;
    size
}

fn compute_size_of_dir_to_delete(final_size: &mut u64, lower_bound: u64, node: NodeId, arena: &Arena<NodeData>) {
    let node_data = arena.get(node).unwrap().get();
    if node_data.file_type == FileType::Directory && node_data.size >= lower_bound {
        *final_size = min(*final_size, node_data.size);
    }

    for child in node.children(arena) {
        compute_size_of_dir_to_delete(final_size, lower_bound, child, arena);
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let (root_node, mut arena) = compute_arena_tree(input);
    let total_used_size = compute_size_part_two(root_node, &mut arena);
    let total_free_size = TOTAL_SIZE - total_used_size;
    let update_extra_necessary_size = UPDATE_SIZE - total_free_size;

    let mut final_size = TOTAL_SIZE;
    compute_size_of_dir_to_delete(&mut final_size, update_extra_necessary_size, root_node, &arena);
    final_size.into()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
