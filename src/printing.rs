use colored::Colorize;
use std::process::abort;
use crate::cube::{Cube, Rotation};


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

impl Cube {
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
}

pub fn print_solution(start_cube: &Cube, rotations: &Vec<Rotation>) {
    let mut cube = start_cube.clone();
    for rotation in rotations.iter() {
        cube.print_rot(rotation);
        println!();
        cube = cube.rotate(rotation);
    }
    cube.print();

    println!("SOLUTION MOVES:");
    for rotation in rotations.iter() {
        print!("{:?} ", rotation);
    }
    println!("");
}
