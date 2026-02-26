use crate::grid::{Coordinate, Grid};

mod grid;


fn solve_part_1(grid: &Grid<usize>, coordinates: impl Iterator<Item = Coordinate>) -> usize {
    coordinates.filter(|&coordinate| {
        let coordinate_height = grid.get_cell(coordinate).unwrap();
        let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        
        directions.iter().any(|&(dx, dy)| {
            let mut line_heights = (1..).into_iter()
                .map_while(|i| {
                    let coordinate = Coordinate {
                        row: coordinate.row.checked_add_signed(dx * i)?,
                        column: coordinate.column.checked_add_signed(dy * i)?
                    };

                    grid.get_cell(coordinate)
                });

            line_heights.all(|height: &usize| height < coordinate_height)
        })
    })
    .count()
}


fn solve_part_2(grid: &Grid<usize>, coordinates: impl Iterator<Item = Coordinate>) -> (Coordinate, usize) {
    coordinates.map(|coordinate| {
        let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        
        let score = directions.into_iter()
            .map(|(dx, dy)| {
                let direction_heights = (1..).into_iter().map_while(|i| {
                    let coordinate = Coordinate {
                        row: coordinate.row.checked_add_signed(dx * i)?,
                        column: coordinate.column.checked_add_signed(dy * i)?
                    };

                    grid.get_cell(coordinate)
                });

                let mut score = 0;
                let current_height = grid.get_cell(coordinate).unwrap();

                for direction_height in direction_heights {
                    score += 1;

                    if direction_height >= current_height {
                        break;
                    }
                }

                score
            })
            .product();

        (coordinate, score)
    })
    .max_by_key(|(_, score)| *score)
    .unwrap()
}


fn main() {
    let input = include_str!("input.txt");
    let grid = create_grid(input);

    let coordinates = (0..grid.rows())
        .into_iter()
        .flat_map(|row| (0..grid.columns()).into_iter().map(move |column| Coordinate::from((row, column))));

    let part_1 = solve_part_1(&grid, coordinates.clone());
    let part_2 = solve_part_2(&grid, coordinates);
    
    println!("Part 1 result: {part_1}");
    println!("Part 2 result: {part_2:?}");
}


fn create_grid(input: &str) -> Grid<usize> {
    let rows = input.lines().count();
    let columns = input.lines().next().unwrap().len();

    let mut grid = Grid::new(rows, columns);

    for (row, line) in input.lines().enumerate() {
        for (column, character) in line.chars().enumerate() {
            *grid.get_cell_mut((row, column).into()).unwrap() = character as usize - '0' as usize; 
        }
    }

    grid
}
