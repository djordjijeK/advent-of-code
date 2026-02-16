use std::{fs::File, io::{self, BufRead, BufReader}, ops::RangeInclusive, path::Path};

use itertools::Itertools;

trait RangeInclusiveExtension {
    fn contains_range(&self, other: &Self) -> bool;

    fn overlaps(&self, other: &Self) -> bool;

    fn contains_or_is_contained(&self, other: &Self) -> bool {
        self.contains_range(other) || other.contains_range(self)
    }

    fn overlaps_or_is_overlapped(&self, other: &Self) -> bool {
        self.overlaps(other) || other.overlaps(self)
    }
} 

impl<T> RangeInclusiveExtension for RangeInclusive<T>
where 
    T: PartialOrd
{
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}

fn get_ranges_iterator(buffered_reader: BufReader<File>) -> impl Iterator<Item = (RangeInclusive<u32>, RangeInclusive<u32>)> {
    buffered_reader.lines()
        .map(|line| {
            line.expect("Each line must have a valid range defined")
                .split(",")
                .map(|range| {
                    let range = range.split("-")
                        .map(|number| number.parse().expect("Range start/end should be u32"))
                        .collect_tuple::<(u32, u32)>()
                        .map(|(start, end)| start..=end)
                        .expect("Each range should have a start and end");

                    range
                })
                .collect_tuple::<(RangeInclusive<u32>, RangeInclusive<u32>)>()
                .expect("Each line must have a valid pair of ranges")
        })
}

fn solve_part_1(buffered_reader: BufReader<File>) -> i32 {
    let mut result = 0;

    for (range_first, range_second) in get_ranges_iterator(buffered_reader) {
        if range_first.contains_or_is_contained(&range_second) {
            result = result + 1;
        }
    }

    result
}

fn solve_part_2(buffered_reader: BufReader<File>) -> i32 {
    let mut result = 0;

    for (first_range, second_range) in get_ranges_iterator(buffered_reader) {
        if first_range.overlaps_or_is_overlapped(&second_range) {
            result = result + 1;
        }
    }

    result
}

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");

    let part_1 = solve_part_1(BufReader::new(File::open(path)?));
    let part_2 = solve_part_2(BufReader::new(File::open(path)?));

    println!("Part 1 result: {part_1}");
    println!("Part 2 result: {part_2}");

    Ok(())
}