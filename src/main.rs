pub mod printing;
pub mod cube;

use printing::*;
use cube::*;


fn main() {
    let solved_cube= Cube::new();

    let mut start_cube = Cube::new();
    start_cube.set_at(2, 3, 1);
    start_cube.set_at(2, 5, 3);
    start_cube.set_at(1, 5, 2);
    start_cube.set_at(3, 3, 2);

    let solution = solve_cube_two_way_breath_first(&start_cube, &solved_cube);

    match solution {
        Some(solution) => {
            print_solution(&start_cube, &solution);
        },
        None => {
            println!("NO SOLUTION FOUND");
        }
    }
}
