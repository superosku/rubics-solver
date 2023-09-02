use std::collections::HashMap;
use colored::Colorize;
use std::process::abort;
use rand::Rng;


#[derive(Clone)]
struct Side {
    pub colors: [u8; 8],
}

impl Side {
    fn new_full(color: u8) -> Side {
        Side {colors: [color; 8]}
    }
}

#[derive(Clone)]
struct Cube {
    sides: [Side; 6],
}

fn colorize(color: u8) -> colored::ColoredString {
    match color {
        0 => "#".white(),
        1 => "#".magenta(),
        2 => "#".green(),
        3 => "#".red(),
        4 => "#".blue(),
        5 => "#".yellow(),
        _ => "?".black(),
    }
}

#[derive(Debug, Clone)]
enum Rotation {
    U,
    D,
    R,
    L,
    F,
    B,
    Ur,
    Dr,
    Rr,
    Lr,
    Fr,
    Br,
}


impl Rotation {
    fn reverse(&self) -> Rotation {
        match self {
            Rotation::U => Rotation::Ur,
            Rotation::D => Rotation::Dr,
            Rotation::R => Rotation::Rr,
            Rotation::L => Rotation::Lr,
            Rotation::F => Rotation::Fr,
            Rotation::B => Rotation::Br,
            Rotation::Ur => Rotation::U,
            Rotation::Dr => Rotation::D,
            Rotation::Rr => Rotation::R,
            Rotation::Lr => Rotation::L,
            Rotation::Fr => Rotation::F,
            Rotation::Br => Rotation::B,
        }
    }
}


impl Cube {
    fn new() -> Cube {
        Cube {
            sides: [
                Side::new_full(0),
                Side::new_full(1),
                Side::new_full(2),
                Side::new_full(3),
                Side::new_full(4),
                Side::new_full(5),
            ]
        }
    }

    fn new_shuffled(shuffles: u32) -> Cube {
        let mut rng = rand::thread_rng();
        let mut new = Cube::new();
        for _ in 0..shuffles {
            let rotations = new.get_rotations();
            // Choose random rotation
            let random_index = (rng.gen::<f64>() * rotations.len() as f64) as usize;
            let rotation = rotations[random_index].clone();
            new = rotation;
            // new = new.rotate(&rotation);
        }
        new
    }

    fn new_debug() -> Cube {
        let mut new = Cube::new();
        for i in 0..6 {
            for j in 0..8 {
                new.sides[i].colors[j] = (i * 8 + j) as u8;
            }
        }
        new
    }

    fn rotate(&self, rotation: &Rotation) -> Cube {
        match rotation {
            // Match different variations of rotations

            Rotation::U => self.rotate_4_r(),
            Rotation::D => self.rotate_3_r(),
            Rotation::R => self.rotate_2(),
            Rotation::L => self.rotate_1_r(),
            Rotation::F => self.rotate_6(),
            Rotation::B => self.rotate_5_r(),
            Rotation::Ur => self.rotate_4(),
            Rotation::Dr => self.rotate_3(),
            Rotation::Rr => self.rotate_2_r(),
            Rotation::Lr => self.rotate_1(),
            Rotation::Fr => self.rotate_6_r(),
            Rotation::Br => self.rotate_5(),
        }
    }

    fn print_debug(&self) {
        for _ in 0..10 {
            print!(" ")
        }
        print!("{:02} ", self.sides[0].colors[0]);
        print!("{:02} ", self.sides[0].colors[1]);
        print!("{:02} ", self.sides[0].colors[2]);
        println!("");
        for _ in 0..10 {
            print!(" ")
        }
        print!("{:02} ", self.sides[0].colors[3]);
        print!("{:02} ", 0);
        print!("{:02} ", self.sides[0].colors[4]);
        println!("");
        for _ in 0..10 {
            print!(" ")
        }
        print!("{:02} ", self.sides[0].colors[5]);
        print!("{:02} ", self.sides[0].colors[6]);
        print!("{:02} ", self.sides[0].colors[7]);
        println!("");
        println!("");
        // Sides 1-4
        for i in 1..5 {
            print!("{:02} ", self.sides[i].colors[0]);
            print!("{:02} ", self.sides[i].colors[1]);
            print!("{:02} ", self.sides[i].colors[2]);
            print!(" ");
        }
        println!("");
        for i in 1..5 {
            print!("{:02} ", self.sides[i].colors[3]);
            print!("{:02} ", i);
            print!("{:02} ", self.sides[i].colors[4]);
            print!(" ");
        }
        println!("");
        for i in 1..5 {
            print!("{:02} ", self.sides[i].colors[5]);
            print!("{:02} ", self.sides[i].colors[6]);
            print!("{:02} ", self.sides[i].colors[7]);
            print!(" ");
        }
        println!("");
        println!("");
        // Side 5
        for _ in 0..10 {
            print!(" ")
        }
        print!("{:02} ", self.sides[5].colors[0]);
        print!("{:02} ", self.sides[5].colors[1]);
        print!("{:02} ", self.sides[5].colors[2]);
        println!("");
        for _ in 0..10 {
            print!(" ")
        }
        print!("{:02} ", self.sides[5].colors[3]);
        print!("{:02} ", 5);
        print!("{:02} ", self.sides[5].colors[4]);
        println!("");
        for _ in 0..10 {
            print!(" ")
        }
        print!("{:02} ", self.sides[5].colors[5]);
        print!("{:02} ", self.sides[5].colors[6]);
        print!("{:02} ", self.sides[5].colors[7]);
        println!("");
    }

    fn print_diff(&self, other: &Cube) {
        println!("Cube:");
        for _ in 0..4 {
            print!(" ")
        }
        print!("{}", if self.sides[0].colors[0] == other.sides[0].colors[0] {"."} else {"#"});
        print!("{}", if self.sides[0].colors[1] == other.sides[0].colors[1] {"."} else {"#"});
        print!("{}", if self.sides[0].colors[2] == other.sides[0].colors[2] {"."} else {"#"});
        println!("");
        for _ in 0..4 {
            print!(" ")
        }
        print!("{}", if self.sides[0].colors[3] == other.sides[0].colors[3] {"."} else {"#"});
        print!(".");
        print!("{}", if self.sides[0].colors[4] == other.sides[0].colors[4] {"."} else {"#"});
        println!("");
        for _ in 0..4 {
            print!(" ")
        }
        print!("{}", if self.sides[0].colors[5] == other.sides[0].colors[5] {"."} else {"#"});
        print!("{}", if self.sides[0].colors[6] == other.sides[0].colors[6] {"."} else {"#"});
        print!("{}", if self.sides[0].colors[7] == other.sides[0].colors[7] {"."} else {"#"});
        println!("");
        println!("");
        // Sides 1-4
        for i in 1..5 {
            print!("{}", if self.sides[i].colors[0] == other.sides[i].colors[0] {"."} else {"#"});
            print!("{}", if self.sides[i].colors[1] == other.sides[i].colors[1] {"."} else {"#"});
            print!("{}", if self.sides[i].colors[2] == other.sides[i].colors[2] {"."} else {"#"});
            print!(" ");
        }
        println!("");
        for i in 1..5 {
            print!("{}", if self.sides[i].colors[3] == other.sides[i].colors[3] {"."} else {"#"});
            print!(".");
            print!("{}", if self.sides[i].colors[4] == other.sides[i].colors[4] {"."} else {"#"});
            print!(" ");
        }
        println!("");
        for i in 1..5 {
            print!("{}", if self.sides[i].colors[5] == other.sides[i].colors[5] {"."} else {"#"});
            print!("{}", if self.sides[i].colors[6] == other.sides[i].colors[6] {"."} else {"#"});
            print!("{}", if self.sides[i].colors[7] == other.sides[i].colors[7] {"."} else {"#"});
            print!(" ");
        }
        println!("");
        println!("");
        // Side 5
        for _ in 0..4 {
            print!(" ")
        }
        print!("{}", if self.sides[5].colors[0] == other.sides[5].colors[0] {"."} else {"#"});
        print!("{}", if self.sides[5].colors[1] == other.sides[5].colors[1] {"."} else {"#"});
        print!("{}", if self.sides[5].colors[2] == other.sides[5].colors[2] {"."} else {"#"});
        println!("");
        for _ in 0..4 {
            print!(" ")
        }
        print!("{}", if self.sides[5].colors[3] == other.sides[5].colors[3] {"."} else {"#"});
        print!(".");
        print!("{}", if self.sides[5].colors[4] == other.sides[5].colors[4] {"."} else {"#"});
        println!("");
        for _ in 0..4 {
            print!(" ")
        }
        print!("{}", if self.sides[5].colors[5] == other.sides[5].colors[5] {"."} else {"#"});
        print!("{}", if self.sides[5].colors[6] == other.sides[5].colors[6] {"."} else {"#"});
        print!("{}", if self.sides[5].colors[7] == other.sides[5].colors[7] {"."} else {"#"});
        println!("");
    }

    fn print(&self) {
        self.print_rot_op(None)
    }

    fn print_rot(&self, rotation: &Rotation) {
        self.print_rot_op(Some(&rotation))
    }

    fn print_rot_op(&self, rotation: Option<&Rotation>) {
        for _ in 0..4 {
            print!(" ")
        }
        print!("{}", colorize(self.sides[0].colors[0]));
        print!("{}", colorize(self.sides[0].colors[1]));
        print!("{}", colorize(self.sides[0].colors[2]));
        println!("");
        for _ in 0..4 {
            print!(" ")
        }
        print!("{}", colorize(self.sides[0].colors[3]));
        match rotation {
            Some(Rotation::Dr) => {print!("{}", '\u{21A9}');}
            Some(Rotation::D) => {print!("{}", '\u{21AA}');}
            _ => {print!("{}", colorize(0));}
        }
        print!("{}", colorize(self.sides[0].colors[4]));
        println!("");
        for _ in 0..4 {
            print!(" ")
        }
        print!("{}", colorize(self.sides[0].colors[5]));
        print!("{}", colorize(self.sides[0].colors[6]));
        print!("{}", colorize(self.sides[0].colors[7]));
        println!("");
        println!("");
        // Sides 1-4
        for i in 1..5 {
            print!("{}", colorize(self.sides[i].colors[0]));
            print!("{}", colorize(self.sides[i].colors[1]));
            print!("{}", colorize(self.sides[i].colors[2]));
            print!(" ");
        }
        println!("");
        for i in 1..5 {
            print!("{}", colorize(self.sides[i].colors[3]));
            match i {
                1 => {
                    match rotation {
                        Some(Rotation::L) => {print!("{}", '\u{21A9}'.to_string().magenta());}
                        Some(Rotation::Lr) => {print!("{}", '\u{21AA}'.to_string().magenta());}
                        _ => {print!("{}", colorize(i as u8));}
                    }
                },
                2 => {
                    match rotation {
                        Some(Rotation::B) => {print!("{}", '\u{21A9}'.to_string().green());}
                        Some(Rotation::Br) => {print!("{}", '\u{21AA}'.to_string().green());}
                        _ => {print!("{}", colorize(i as u8));}
                    }
                },
                3 => {
                    // Not done
                    match rotation {
                        Some(Rotation::R) => {print!("{}", '\u{21A9}'.to_string().red());}
                        Some(Rotation::Rr) => {print!("{}", '\u{21AA}'.to_string().red());}
                        _ => {print!("{}", colorize(i as u8));}
                    }
                },
                4 => {
                    match rotation {
                        Some(Rotation::F) => {print!("{}", '\u{21A9}'.to_string().blue());}
                        Some(Rotation::Fr) => {print!("{}", '\u{21AA}'.to_string().blue());}
                        _ => {print!("{}", colorize(i as u8));}
                    }
                },
                _ => {
                    abort();
                }
            }
            print!("{}", colorize(self.sides[i].colors[4]));
            print!(" ");
        }
        println!("");
        for i in 1..5 {
            print!("{}", colorize(self.sides[i].colors[5]));
            print!("{}", colorize(self.sides[i].colors[6]));
            print!("{}", colorize(self.sides[i].colors[7]));
            print!(" ");
        }
        println!("");
        println!("");
        // Side 5
        for _ in 0..4 {
            print!(" ")
        }
        print!("{}", colorize(self.sides[5].colors[0]));
        print!("{}", colorize(self.sides[5].colors[1]));
        print!("{}", colorize(self.sides[5].colors[2]));
        println!("");
        for _ in 0..4 {
            print!(" ")
        }
        print!("{}", colorize(self.sides[5].colors[3]));
        match rotation {
            Some(Rotation::U) => {print!("{}", '\u{21A9}'.to_string().yellow());}
            Some(Rotation::Ur) => {print!("{}", '\u{21AA}'.to_string().yellow());}
            _ => {print!("{}", colorize(5));}
        }
        print!("{}", colorize(self.sides[5].colors[4]));
        println!("");
        for _ in 0..4 {
            print!(" ")
        }
        print!("{}", colorize(self.sides[5].colors[5]));
        print!("{}", colorize(self.sides[5].colors[6]));
        print!("{}", colorize(self.sides[5].colors[7]));
        println!("");
    }

    fn get_at(&self, side: usize, face: usize) -> u8 {
        if face == 4 {
            panic!("Cannot set center piece")
        }
        let face_pos = if face < 4 {face} else {face - 1};
        self.sides[side].colors[face_pos]
    }
    fn set_at(&mut self, side: usize, face: usize, color: u8) {
        if face == 4 {
            panic!("Cannot set center piece")
        }
        let face_pos = if face < 4 {face} else {face - 1};
        self.sides[side].colors[face_pos] = color;
    }

    fn rotate_1(&self) -> Cube {
        let mut new_cube = self.clone();
        new_cube.set_at(0, 0, self.get_at(2, 0));
        new_cube.set_at(0, 3, self.get_at(2, 3));
        new_cube.set_at(0, 6, self.get_at(2, 6));

        new_cube.set_at(2, 0, self.get_at(5, 0));
        new_cube.set_at(2, 3, self.get_at(5, 3));
        new_cube.set_at(2, 6, self.get_at(5, 6));

        new_cube.set_at(5, 0, self.get_at(4, 8));
        new_cube.set_at(5, 3, self.get_at(4, 5));
        new_cube.set_at(5, 6, self.get_at(4, 2));

        new_cube.set_at(4, 8, self.get_at(0, 0));
        new_cube.set_at(4, 5, self.get_at(0, 3));
        new_cube.set_at(4, 2, self.get_at(0, 6));

        new_cube.set_at(1, 0, self.get_at(1, 2));
        new_cube.set_at(1, 1, self.get_at(1, 5));
        new_cube.set_at(1, 2, self.get_at(1, 8));
        new_cube.set_at(1, 5, self.get_at(1, 7));
        new_cube.set_at(1, 8, self.get_at(1, 6));
        new_cube.set_at(1, 7, self.get_at(1, 3));
        new_cube.set_at(1, 6, self.get_at(1, 0));
        new_cube.set_at(1, 3, self.get_at(1, 1));

        new_cube
    }

    fn rotate_2(&self) -> Cube {
        let mut new_cube = self.clone();
        new_cube.set_at(0, 2, self.get_at(2, 2));
        new_cube.set_at(0, 5, self.get_at(2, 5));
        new_cube.set_at(0, 8, self.get_at(2, 8));

        new_cube.set_at(2, 2, self.get_at(5, 2));
        new_cube.set_at(2, 5, self.get_at(5, 5));
        new_cube.set_at(2, 8, self.get_at(5, 8));

        new_cube.set_at(5, 2, self.get_at(4, 6));
        new_cube.set_at(5, 5, self.get_at(4, 3));
        new_cube.set_at(5, 8, self.get_at(4, 0));

        new_cube.set_at(4, 6, self.get_at(0, 2));
        new_cube.set_at(4, 3, self.get_at(0, 5));
        new_cube.set_at(4, 0, self.get_at(0, 8));

        new_cube.set_at(3, 2, self.get_at(3, 0));
        new_cube.set_at(3, 5, self.get_at(3, 1));
        new_cube.set_at(3, 8, self.get_at(3, 2));
        new_cube.set_at(3, 7, self.get_at(3, 5));
        new_cube.set_at(3, 6, self.get_at(3, 8));
        new_cube.set_at(3, 3, self.get_at(3, 7));
        new_cube.set_at(3, 0, self.get_at(3, 6));
        new_cube.set_at(3, 1, self.get_at(3, 3));

        new_cube
    }

    fn rotate_3(&self) -> Cube {
        let mut new_cube = self.clone();
        new_cube.set_at(1, 0, self.get_at(2, 0));
        new_cube.set_at(1, 1, self.get_at(2, 1));
        new_cube.set_at(1, 2, self.get_at(2, 2));

        new_cube.set_at(2, 0, self.get_at(3, 0));
        new_cube.set_at(2, 1, self.get_at(3, 1));
        new_cube.set_at(2, 2, self.get_at(3, 2));

        new_cube.set_at(3, 0, self.get_at(4, 0));
        new_cube.set_at(3, 1, self.get_at(4, 1));
        new_cube.set_at(3, 2, self.get_at(4, 2));

        new_cube.set_at(4, 0, self.get_at(1, 0));
        new_cube.set_at(4, 1, self.get_at(1, 1));
        new_cube.set_at(4, 2, self.get_at(1, 2));

        new_cube.set_at(0, 2, self.get_at(0, 0));
        new_cube.set_at(0, 5, self.get_at(0, 1));
        new_cube.set_at(0, 8, self.get_at(0, 2));
        new_cube.set_at(0, 7, self.get_at(0, 5));
        new_cube.set_at(0, 6, self.get_at(0, 8));
        new_cube.set_at(0, 3, self.get_at(0, 7));
        new_cube.set_at(0, 0, self.get_at(0, 6));
        new_cube.set_at(0, 1, self.get_at(0, 3));

        new_cube
    }

    fn rotate_4(&self) -> Cube {
        let mut new_cube = self.clone();
        new_cube.set_at(1, 6, self.get_at(2, 6));
        new_cube.set_at(1, 7, self.get_at(2, 7));
        new_cube.set_at(1, 8, self.get_at(2, 8));

        new_cube.set_at(2, 6, self.get_at(3, 6));
        new_cube.set_at(2, 7, self.get_at(3, 7));
        new_cube.set_at(2, 8, self.get_at(3, 8));

        new_cube.set_at(3, 6, self.get_at(4, 6));
        new_cube.set_at(3, 7, self.get_at(4, 7));
        new_cube.set_at(3, 8, self.get_at(4, 8));

        new_cube.set_at(4, 6, self.get_at(1, 6));
        new_cube.set_at(4, 7, self.get_at(1, 7));
        new_cube.set_at(4, 8, self.get_at(1, 8));

        new_cube.set_at(5, 0, self.get_at(5, 2));
        new_cube.set_at(5, 1, self.get_at(5, 5));
        new_cube.set_at(5, 2, self.get_at(5, 8));
        new_cube.set_at(5, 5, self.get_at(5, 7));
        new_cube.set_at(5, 8, self.get_at(5, 6));
        new_cube.set_at(5, 7, self.get_at(5, 3));
        new_cube.set_at(5, 6, self.get_at(5, 0));
        new_cube.set_at(5, 3, self.get_at(5, 1));

        new_cube
    }

    fn rotate_5(&self) -> Cube {
        let mut new_cube = self.clone();
        new_cube.set_at(0, 6, self.get_at(3, 0));
        new_cube.set_at(0, 7, self.get_at(3, 3));
        new_cube.set_at(0, 8, self.get_at(3, 6));

        new_cube.set_at(3, 0, self.get_at(5, 2));
        new_cube.set_at(3, 3, self.get_at(5, 1));
        new_cube.set_at(3, 6, self.get_at(5, 0));

        new_cube.set_at(5, 0, self.get_at(1, 2));
        new_cube.set_at(5, 1, self.get_at(1, 5));
        new_cube.set_at(5, 2, self.get_at(1, 8));

        new_cube.set_at(1, 2, self.get_at(0, 8));
        new_cube.set_at(1, 5, self.get_at(0, 7));
        new_cube.set_at(1, 8, self.get_at(0, 6));

        new_cube.set_at(2, 0, self.get_at(2, 2));
        new_cube.set_at(2, 1, self.get_at(2, 5));
        new_cube.set_at(2, 2, self.get_at(2, 8));
        new_cube.set_at(2, 5, self.get_at(2, 7));
        new_cube.set_at(2, 8, self.get_at(2, 6));
        new_cube.set_at(2, 7, self.get_at(2, 3));
        new_cube.set_at(2, 6, self.get_at(2, 0));
        new_cube.set_at(2, 3, self.get_at(2, 1));

        new_cube
    }

    fn rotate_6(&self) -> Cube {
        let mut new_cube = self.clone();
        new_cube.set_at(0, 1, self.get_at(3, 5));
        new_cube.set_at(0, 0, self.get_at(3, 2));
        new_cube.set_at(0, 2, self.get_at(3, 8));

        new_cube.set_at(3, 2, self.get_at(5, 8));
        new_cube.set_at(3, 5, self.get_at(5, 7));
        new_cube.set_at(3, 8, self.get_at(5, 6));

        new_cube.set_at(5, 6, self.get_at(1, 0));
        new_cube.set_at(5, 7, self.get_at(1, 3));
        new_cube.set_at(5, 8, self.get_at(1, 6));

        new_cube.set_at(1, 0, self.get_at(0, 2));
        new_cube.set_at(1, 6, self.get_at(0, 0));
        new_cube.set_at(1, 3, self.get_at(0, 1));

        new_cube.set_at(4, 2, self.get_at(4, 0));
        new_cube.set_at(4, 5, self.get_at(4, 1));
        new_cube.set_at(4, 8, self.get_at(4, 2));
        new_cube.set_at(4, 7, self.get_at(4, 5));
        new_cube.set_at(4, 6, self.get_at(4, 8));
        new_cube.set_at(4, 3, self.get_at(4, 7));
        new_cube.set_at(4, 0, self.get_at(4, 6));
        new_cube.set_at(4, 1, self.get_at(4, 3));

        new_cube
    }

    fn rotate_1_r(&self) -> Cube {
        let mut new_cube = self.clone();
        new_cube.set_at(2, 0, self.get_at(0, 0));
        new_cube.set_at(2, 3, self.get_at(0, 3));
        new_cube.set_at(2, 6, self.get_at(0, 6));

        new_cube.set_at(5, 0, self.get_at(2, 0));
        new_cube.set_at(5, 3, self.get_at(2, 3));
        new_cube.set_at(5, 6, self.get_at(2, 6));

        new_cube.set_at(4, 8, self.get_at(5, 0));
        new_cube.set_at(4, 5, self.get_at(5, 3));
        new_cube.set_at(4, 2, self.get_at(5, 6));

        new_cube.set_at(0, 0, self.get_at(4, 8));
        new_cube.set_at(0, 3, self.get_at(4, 5));
        new_cube.set_at(0, 6, self.get_at(4, 2));

        new_cube.set_at(1, 2, self.get_at(1, 0));
        new_cube.set_at(1, 5, self.get_at(1, 1));
        new_cube.set_at(1, 8, self.get_at(1, 2));
        new_cube.set_at(1, 7, self.get_at(1, 5));
        new_cube.set_at(1, 6, self.get_at(1, 8));
        new_cube.set_at(1, 3, self.get_at(1, 7));
        new_cube.set_at(1, 0, self.get_at(1, 6));
        new_cube.set_at(1, 1, self.get_at(1, 3));

        new_cube
    }

    fn rotate_2_r(&self) -> Cube {
        let mut new_cube = self.clone();
        new_cube.set_at(2, 2, self.get_at(0, 2));
        new_cube.set_at(2, 5, self.get_at(0, 5));
        new_cube.set_at(2, 8, self.get_at(0, 8));

        new_cube.set_at(5, 2, self.get_at(2, 2));
        new_cube.set_at(5, 5, self.get_at(2, 5));
        new_cube.set_at(5, 8, self.get_at(2, 8));

        new_cube.set_at(4, 6, self.get_at(5, 2));
        new_cube.set_at(4, 3, self.get_at(5, 5));
        new_cube.set_at(4, 0, self.get_at(5, 8));

        new_cube.set_at(0, 2, self.get_at(4, 6));
        new_cube.set_at(0, 5, self.get_at(4, 3));
        new_cube.set_at(0, 8, self.get_at(4, 0));

        new_cube.set_at(3, 0, self.get_at(3, 2));
        new_cube.set_at(3, 1, self.get_at(3, 5));
        new_cube.set_at(3, 2, self.get_at(3, 8));
        new_cube.set_at(3, 5, self.get_at(3, 7));
        new_cube.set_at(3, 8, self.get_at(3, 6));
        new_cube.set_at(3, 7, self.get_at(3, 3));
        new_cube.set_at(3, 6, self.get_at(3, 0));
        new_cube.set_at(3, 3, self.get_at(3, 1));

        new_cube
    }

    fn rotate_3_r(&self) -> Cube {
        let mut new_cube = self.clone();
        new_cube.set_at(2, 0, self.get_at(1, 0));
        new_cube.set_at(2, 1, self.get_at(1, 1));
        new_cube.set_at(2, 2, self.get_at(1, 2));

        new_cube.set_at(3, 0, self.get_at(2, 0));
        new_cube.set_at(3, 1, self.get_at(2, 1));
        new_cube.set_at(3, 2, self.get_at(2, 2));

        new_cube.set_at(4, 0, self.get_at(3, 0));
        new_cube.set_at(4, 1, self.get_at(3, 1));
        new_cube.set_at(4, 2, self.get_at(3, 2));

        new_cube.set_at(1, 0, self.get_at(4, 0));
        new_cube.set_at(1, 1, self.get_at(4, 1));
        new_cube.set_at(1, 2, self.get_at(4, 2));

        new_cube.set_at(0, 0, self.get_at(0, 2));
        new_cube.set_at(0, 1, self.get_at(0, 5));
        new_cube.set_at(0, 2, self.get_at(0, 8));
        new_cube.set_at(0, 5, self.get_at(0, 7));
        new_cube.set_at(0, 8, self.get_at(0, 6));
        new_cube.set_at(0, 7, self.get_at(0, 3));
        new_cube.set_at(0, 6, self.get_at(0, 0));
        new_cube.set_at(0, 3, self.get_at(0, 1));

        new_cube
    }

    fn rotate_4_r(&self) -> Cube {
        let mut new_cube = self.clone();
        new_cube.set_at(2, 6, self.get_at(1, 6));
        new_cube.set_at(2, 7, self.get_at(1, 7));
        new_cube.set_at(2, 8, self.get_at(1, 8));

        new_cube.set_at(3, 6, self.get_at(2, 6));
        new_cube.set_at(3, 7, self.get_at(2, 7));
        new_cube.set_at(3, 8, self.get_at(2, 8));

        new_cube.set_at(4, 6, self.get_at(3, 6));
        new_cube.set_at(4, 7, self.get_at(3, 7));
        new_cube.set_at(4, 8, self.get_at(3, 8));

        new_cube.set_at(1, 6, self.get_at(4, 6));
        new_cube.set_at(1, 7, self.get_at(4, 7));
        new_cube.set_at(1, 8, self.get_at(4, 8));

        new_cube.set_at(5, 2, self.get_at(5, 0));
        new_cube.set_at(5, 5, self.get_at(5, 1));
        new_cube.set_at(5, 8, self.get_at(5, 2));
        new_cube.set_at(5, 7, self.get_at(5, 5));
        new_cube.set_at(5, 6, self.get_at(5, 8));
        new_cube.set_at(5, 3, self.get_at(5, 7));
        new_cube.set_at(5, 0, self.get_at(5, 6));
        new_cube.set_at(5, 1, self.get_at(5, 3));

        new_cube
    }

    fn rotate_5_r(&self) -> Cube {
        let mut new_cube = self.clone();
        new_cube.set_at(3, 0, self.get_at(0, 6));
        new_cube.set_at(3, 3, self.get_at(0, 7));
        new_cube.set_at(3, 6, self.get_at(0, 8));

        new_cube.set_at(5, 2, self.get_at(3, 0));
        new_cube.set_at(5, 1, self.get_at(3, 3));
        new_cube.set_at(5, 0, self.get_at(3, 6));

        new_cube.set_at(1, 2, self.get_at(5, 0));
        new_cube.set_at(1, 5, self.get_at(5, 1));
        new_cube.set_at(1, 8, self.get_at(5, 2));

        new_cube.set_at(0, 8, self.get_at(1, 2));
        new_cube.set_at(0, 7, self.get_at(1, 5));
        new_cube.set_at(0, 6, self.get_at(1, 8));

        new_cube.set_at(2, 2, self.get_at(2, 0));
        new_cube.set_at(2, 5, self.get_at(2, 1));
        new_cube.set_at(2, 8, self.get_at(2, 2));
        new_cube.set_at(2, 7, self.get_at(2, 5));
        new_cube.set_at(2, 6, self.get_at(2, 8));
        new_cube.set_at(2, 3, self.get_at(2, 7));
        new_cube.set_at(2, 0, self.get_at(2, 6));
        new_cube.set_at(2, 1, self.get_at(2, 3));

        new_cube
    }

    fn rotate_6_r(&self) -> Cube {
        let mut new_cube = self.clone();
        new_cube.set_at(3, 5, self.get_at(0, 1));
        new_cube.set_at(3, 2, self.get_at(0, 0));
        new_cube.set_at(3, 8, self.get_at(0, 2));

        new_cube.set_at(5, 8, self.get_at(3, 2));
        new_cube.set_at(5, 7, self.get_at(3, 5));
        new_cube.set_at(5, 6, self.get_at(3, 8));

        new_cube.set_at(1, 0, self.get_at(5, 6));
        new_cube.set_at(1, 3, self.get_at(5, 7));
        new_cube.set_at(1, 6, self.get_at(5, 8));

        new_cube.set_at(0, 2, self.get_at(1, 0));
        new_cube.set_at(0, 0, self.get_at(1, 6));
        new_cube.set_at(0, 1, self.get_at(1, 3));

        new_cube.set_at(4, 0, self.get_at(4, 2));
        new_cube.set_at(4, 1, self.get_at(4, 5));
        new_cube.set_at(4, 2, self.get_at(4, 8));
        new_cube.set_at(4, 5, self.get_at(4, 7));
        new_cube.set_at(4, 8, self.get_at(4, 6));
        new_cube.set_at(4, 7, self.get_at(4, 3));
        new_cube.set_at(4, 6, self.get_at(4, 0));
        new_cube.set_at(4, 3, self.get_at(4, 1));

        new_cube
    }

    fn get_rotations(&self) -> Vec<Cube> {
        let mut rotations = Vec::new();

        // rotations.push(self.rotate_2().rotate_2());
        // rotations.push(self.rotate_3());
        // rotations.push(self.rotate_5());

        // rotations.push(self.rotate_1().rotate_1());
        // rotations.push(self.rotate_4().rotate_4());
        // rotations.push(self.rotate_6().rotate_6());

        rotations.push(self.rotate_1());
        rotations.push(self.rotate_2());
        rotations.push(self.rotate_3());
        rotations.push(self.rotate_4());
        rotations.push(self.rotate_5());
        rotations.push(self.rotate_6());
        rotations.push(self.rotate_1_r());
        rotations.push(self.rotate_2_r());
        rotations.push(self.rotate_3_r());
        rotations.push(self.rotate_4_r());
        rotations.push(self.rotate_5_r());
        rotations.push(self.rotate_6_r());
        rotations
    }

    fn get_hash(&self) -> Hash {
        let mut hash: Hash = [0; 3];
        let mut index = 0;
        for i in 0..9 {
            for j in 0..6 {
                if i == 4 {
                    continue;
                }
                let value = self.get_at(j, i);
                if index <= 20 {
                    let fixed_index = index as u64;
                    hash[0] += (value as u64).rotate_left(3 * fixed_index as u32);
                } else if index <= 39 {
                    let fixed_index = (index - 20) as u64;
                    hash[1] += (value as u64).rotate_left(3 * fixed_index as u32);
                } else {
                    let fixed_index = (index - 39) as u64;
                    hash[2] += (value as u64).rotate_left(3 * fixed_index as u32)
                }
                index += 1;
            }
        }
        hash
    }

    // fn get_hash(&self) -> Hash {
    //     let mut hash: [u8; 48] = [0; 48];
    //     let mut hash_index = 0;
    //     for i in 0..9 {
    //         for j in 0..6 {
    //             if i == 4 {
    //                 continue;
    //             }
    //             let value = self.get_at(j, i);
    //             hash[hash_index] = value;
    //             hash_index += 1;
    //         }
    //     }
    //     // println!("Hash: {:#64b} {:#64b}", hash1, hash2);
    //     hash
    // }

    // fn from_hash(hash: Hash) -> Cube {
    //     let mut cube = Cube::new();
    //     let mut hash1 = hash[0];
    //     let mut hash2 = hash[1];
    //     for i in 0..9 {
    //         for j in 0..6 {
    //             if i == 4 {
    //                 continue;
    //             }
    //             let index = i * 6 + j;
    //             if index <= 20 {
    //                 let fixed_index = index;
    //
    //             } else {
    //                 let fixed_index = index - 20;
    //
    //             }
    //         }
    //     }
    //     cube
    // }
}


type Hash = [u64; 3];
// type Hash = [u8; 8 * 6];


fn get_solution_from_two_way_hashmaps(
    middle_hash: Hash,
    middle_cube: &Cube,
    begin_hashes: &HashMap<Hash, Option<Rotation>>,
    end_hashes: &HashMap<Hash, Option<Rotation>>
) -> Option<Vec<Rotation>> {
    let mut solution_rotations: Vec<Rotation> = vec![];

    // Find route forward
    let mut lookup_cube = middle_cube.clone();
    let mut lookup_hash = middle_hash;
    loop {
        let lookup_rotation = end_hashes.get(&lookup_hash).unwrap();
        match lookup_rotation {
            Some(rotation) => {
                println!("ROTATION {:?}", rotation);
                lookup_cube = lookup_cube.rotate(&rotation.reverse());
                lookup_hash = lookup_cube.get_hash();
                solution_rotations.insert(0, rotation.clone());
            },
            None => {
                break
            }
        }
    }
    // Find route backward
    let mut lookup_cube = middle_cube.clone();
    let mut lookup_hash = middle_hash;
    loop {
        let lookup_rotation = begin_hashes.get(&lookup_hash).unwrap();
        match lookup_rotation {
            Some(rotation) => {
                println!("ROTATION {:?}", rotation);
                lookup_cube = lookup_cube.rotate(&rotation.reverse());
                lookup_hash = lookup_cube.get_hash();
                solution_rotations.push(rotation.reverse());
            },
            None => {
                println!("STUFF DONE 2");
                break
            }
        }
    }

    Some(solution_rotations)
}

fn solve_cube_two_way_breath_first(start_cube: &Cube, end_cube: &Cube) -> Option<Vec<Rotation>> {
    let mut solution_rotations: Vec<Rotation> = vec![];

    Some(solution_rotations)
}

fn print_solution(start_cube: &Cube, rotations: &Vec<Rotation>) {
    let mut cube = start_cube.clone();
    for rotation in rotations.iter() {
        cube.print_rot(rotation);
        println!();
        cube = cube.rotate(rotation);
    }
    cube.print();

    println!("SOLUTION:");
    for rotation in rotations.iter() {
        print!("{:?} ", rotation);
    }
    println!("");

}


fn main() {
    {
        let cube = Cube::new_debug();
        // cube.print();

        assert_eq!(cube.rotate_1().rotate_1().rotate_1().rotate_1().get_hash(), cube.get_hash());
        assert_eq!(cube.rotate_2().rotate_2().rotate_2().rotate_2().get_hash(), cube.get_hash());
        assert_eq!(cube.rotate_3().rotate_3().rotate_3().rotate_3().get_hash(), cube.get_hash());
        assert_eq!(cube.rotate_4().rotate_4().rotate_4().rotate_4().get_hash(), cube.get_hash());
        assert_eq!(cube.rotate_5().rotate_5().rotate_5().rotate_5().get_hash(), cube.get_hash());
        assert_eq!(cube.rotate_6().rotate_6().rotate_6().rotate_6().get_hash(), cube.get_hash());

        assert_eq!(cube.rotate_1().get_hash(), cube.rotate_1_r().rotate_1_r().rotate_1_r().get_hash());
        assert_eq!(cube.rotate_2().get_hash(), cube.rotate_2_r().rotate_2_r().rotate_2_r().get_hash());
        assert_eq!(cube.rotate_3().get_hash(), cube.rotate_3_r().rotate_3_r().rotate_3_r().get_hash());
        assert_eq!(cube.rotate_4().get_hash(), cube.rotate_4_r().rotate_4_r().rotate_4_r().get_hash());
        assert_eq!(cube.rotate_5().get_hash(), cube.rotate_5_r().rotate_5_r().rotate_5_r().get_hash());
        assert_eq!(cube.rotate_6().get_hash(), cube.rotate_6_r().rotate_6_r().rotate_6_r().get_hash());
    }

    // let cube = Cube::new();
    println!("CUBE:");
    let solved_cube= Cube::new();
    solved_cube.print();

    println!("GOAL CUBE:");
    let mut start_cube = Cube::new();
    start_cube.set_at(2, 3, 1);
    start_cube.set_at(2, 5, 3);
    start_cube.set_at(1, 5, 2);
    start_cube.set_at(3, 3, 2);
    start_cube.print();
    println!();

    let start_cube = Cube::new_shuffled(8);

    // return;

    // println!("ITERATIONS:");
    // for rotations in cube.get_rotations() {
    //     cube.print_debug();
    //     println!();
    //     rotations.print_debug();
    //     println!();
    //     cube.print_diff(&rotations);
    //     println!("*******************************************");
    // }
    // return;

    let all_rotations: Vec<Rotation> = vec![
        Rotation::U,
        Rotation::D,
        Rotation::R,
        Rotation::L,
        Rotation::F,
        Rotation::B,
        Rotation::Ur,
        Rotation::Dr,
        Rotation::Rr,
        Rotation::Lr,
        Rotation::Fr,
        Rotation::Br,
    ];
    //
    // // Iterate Rotation enum
    // for rotation in all_rotations.iter() {
    //     println!("ROTATION {:?}", rotation);
    //     let rotated_cube = cube.rotate(rotation);
    //     cube.print_rot(rotation);
    //     println!();
    //     rotated_cube.print();
    //     println!("****************************************");
    // }

    // for (i, cube) in cube.get_rotations().iter().enumerate() {
    //     println!("ROTATION {}", if i < 6 {(i + 1).to_string()} else {(i + 1 - 6).to_string() + "r"});
    //     cube.print();
    // }

    // return;

    let mut hashes: HashMap<Hash, Option<Rotation>> = HashMap::new();
    {
        let mut old_cubes: Vec<Cube> = vec![solved_cube.clone()];
        let mut new_cubes: Vec<Cube> = Vec::new();

        hashes.insert(solved_cube.get_hash(), None);

        let mut hash_collisions_total = 0;
        for _ in 0..5 {
            let mut hash_collisions = 0;

            for cube in old_cubes.iter() {
                let rotations = cube.get_rotations();
                for rotation in all_rotations.iter() {
                // for rotation in rotations {
                    let rotated_cube = cube.rotate(&rotation);
                    let hash = rotated_cube.get_hash();
                    if hashes.contains_key(&hash) {
                        hash_collisions += 1;
                        hash_collisions_total += 1;
                        continue;
                    }
                    hashes.insert(hash, Some(rotation.clone()));
                    new_cubes.push(rotated_cube);
                }
            }

            println!(
                "GOAL Old: {}, New: {} Collisions: {}, CollisionsTot: {}, Total: {}",
                old_cubes.len(), new_cubes.len(), hash_collisions, hash_collisions_total, hashes.len()
            );

            old_cubes = new_cubes;
            new_cubes = Vec::new();
        }
    }

    {
        let mut goal_hashes: HashMap<Hash, Option<Rotation>> = HashMap::new();
        let mut old_cubes: Vec<Cube> = vec![start_cube.clone()];
        let mut new_cubes: Vec<Cube> = Vec::new();

        goal_hashes.insert(start_cube.get_hash(), None);

        let mut hash_collisions_total = 0;
        for _ in 0..5 {
            let mut hash_collisions = 0;

            for cube in old_cubes.iter() {
                let rotations = cube.get_rotations();
                for rotation in all_rotations.iter() {
                // for rotation in rotations {
                    let rotated_cube = cube.rotate(&rotation);
                    let hash = rotated_cube.get_hash();

                    if goal_hashes.contains_key(&hash) {
                        hash_collisions += 1;
                        hash_collisions_total += 1;
                        continue;
                    }
                    goal_hashes.insert(hash, Some(rotation.clone()));

                    if hashes.contains_key(&hash) {
                        let solution_rotations = get_solution_from_two_way_hashmaps(
                            hash,
                            &rotated_cube,
                            &hashes,
                            &goal_hashes,

                        ).unwrap();

                        print_solution(&start_cube, &solution_rotations);

                        return;
                    }

                    new_cubes.push(rotated_cube);
                }
            }

            println!(
                "GOAL Old: {}, New: {} Collisions: {}, CollisionsTot: {}, Total: {}",
                old_cubes.len(), new_cubes.len(), hash_collisions, hash_collisions_total, goal_hashes.len()
            );

            old_cubes = new_cubes;
            new_cubes = Vec::new();
        }
    }
}
