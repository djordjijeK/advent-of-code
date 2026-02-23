use std::collections::HashSet;


fn main() {
    // embed the contents of "input.txt" into the binary at compile time
    let input: &'static str = include_str!("input.txt");

    let part_1 = solve_part(input, 4);
    let part_2 = solve_part(input,14);

    println!("Part 1 result: {:?}", part_1);
    println!("Part 2 result: {:?}", part_2);
}


fn solve_part(input: &str, sequence_size: usize) -> Option<usize> {
    let data = input.as_bytes()
        .windows(sequence_size)
        .position(|window| window.iter().collect::<HashSet<_>>().len() == sequence_size)
        .map(|position| position + sequence_size);

    data
}


#[cfg(test)]
mod test {
    use crate::solve_part;

    #[test]
    fn test_find_marker() {
        assert_eq!(Some(7), solve_part("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4));
        assert_eq!(Some(5), solve_part("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
        assert_eq!(Some(6), solve_part("nppdvjthqldpwncqszvftbrmjlhg", 4));
        assert_eq!(Some(10), solve_part("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4));
        assert_eq!(Some(11), solve_part("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4));
    }
}
