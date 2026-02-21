use std::{cell::{RefCell}, fmt::Debug, io, path::Path, vec};
use nom::{Finish, IResult, Parser, branch::alt, bytes::complete::{tag, take_while1}, character::complete::one_of, combinator::{all_consuming, map, map_res, opt, value}, sequence::{delimited, preceded}};


#[derive(Clone)]
struct Crate(char);


#[derive(Debug)]
struct Instruction {
    quantity: usize,
    src: usize,
    destination: usize
}


fn parse_crate(input: &str) -> IResult<&str, Crate> {
    let crate_parser = delimited(tag("["), one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), tag("]")); 
    map(crate_parser, Crate).parse(input)
}


fn parse_hole(input: &str) -> IResult<&str, ()> {
    value((), tag("   ")).parse(input)
}


fn parse_crate_or_hole(input: &str) -> IResult<&str, Option<Crate>> {
    alt((map(parse_crate, Some), map(parse_hole, |_| None))).parse(input)
}


fn parse_line_of_crates(input: &str) -> IResult<&str, Vec<Option<Crate>>> {
    let (mut input, maybe_crate) = parse_crate_or_hole(input)?;
    let mut result = vec![maybe_crate];

    loop {
        let (rest_input, maybe_crate) = opt(preceded(tag(" "), parse_crate_or_hole)).parse(input)?;
        match maybe_crate {
            Some(maybe_crate) => result.push(maybe_crate),
            None => break,
        }

        input = rest_input;
    }

    Ok((input, result))
}


fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(take_while1(|character: char| character.is_ascii_digit()), |digit: &str| digit.parse::<usize>()).parse(input)
}


fn parse_pile_number(input: &str) -> IResult<&str, usize> {
    map(parse_number, |number| number - 1).parse(input)
}


fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    map(
        (
            preceded(tag("move "), parse_number),
            preceded(tag(" from "), parse_pile_number),
            preceded(tag(" to "), parse_pile_number),
        ),
        |(quantity, src, destination)| Instruction {
            quantity,
            src,
            destination,
        },
    )
    .parse(input)
}


fn transpose_reversed<T>(matrix: Vec<Vec<Option<T>>>) -> Vec<RefCell<Vec<T>>> {
    let n = matrix.first().map_or(0, |row| row.len());

    let mut iters: Vec<_> = matrix.into_iter().map(|row| row.into_iter()).collect();

    (0..n)
        .map(|_| {
            iters
                .iter_mut()
                .rev()
                .filter_map(|it| it.next().flatten())
                .collect::<Vec<T>>()
        })
        .map(|vector| RefCell::new(vector))
        .collect()
}


fn solve_part_1(mut crates_matrix: Vec<RefCell<Vec<Crate>>>, instructions: &[Instruction]) -> String {
    for instruction in instructions {
        for _ in 0..instruction.quantity {
            let top_crate = crates_matrix[instruction.src].get_mut().pop().unwrap();
            crates_matrix[instruction.destination].get_mut().push(top_crate);
        }
    }

    crates_matrix.iter().map(|row| row.borrow().last().unwrap().0).collect()
}


fn solve_part_2(crates_matrix: Vec<RefCell<Vec<Crate>>>, instructions: &Vec<Instruction>) -> String {
    for instruction in instructions {
        let mut source_vector = crates_matrix.get(instruction.src).unwrap().borrow_mut();
        let mut destination_vector = crates_matrix.get(instruction.destination).unwrap().borrow_mut();
        
        for crt in (0..instruction.quantity).map(|_| source_vector.pop().unwrap()).collect::<Vec<_>>().into_iter().rev() {
            destination_vector.push(crt);
        } 
    }

    crates_matrix.iter().map(|row| row.borrow().last().unwrap().0).collect() 
}


fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let input = std::fs::read_to_string(path)?;
    let mut lines = input.lines();

    let crates_matrix: Vec<_> = (&mut lines)
        .map_while(|line| all_consuming(parse_line_of_crates)
            .parse(line)
            .finish()
            .ok()
            .map(|(_, crates_vector)| crates_vector)
        )
        .collect();

    let crates_matrix: Vec<RefCell<Vec<Crate>>> = transpose_reversed(crates_matrix);

    assert!(lines.next().unwrap().is_empty());

    let instructions: Vec<_> = lines
        .map_while(|line| all_consuming(parse_instruction)
            .parse(line)
            .finish()
            .ok()
            .map(|(_, instruction)| instruction)
        )
        .collect();
    
    let crates1 = crates_matrix.iter()
        .map(|c| RefCell::new(c.borrow().clone()))
        .collect::<Vec<_>>();

    let crates2 = crates_matrix.iter()
        .map(|c| RefCell::new(c.borrow().clone()))
        .collect::<Vec<_>>();

    let part_1 = solve_part_1(crates1, &instructions);
    let part_2 = solve_part_2(crates2, &instructions);

    println!("Part 1 result: {part_1}");
    println!("Part 2 result: {part_2}");

    Ok(())
}