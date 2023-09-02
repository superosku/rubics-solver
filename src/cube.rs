use std::collections::HashMap;
use colored::Colorize;
use rand::Rng;

#[derive(Clone)]
pub struct Side {
    pub colors: [u8; 8],
}

impl Side {
    fn new_full(color: u8) -> Side {
        Side {colors: [color; 8]}
    }
}

#[derive(Clone)]
pub struct Cube {
    // TODO Should not be pub but accessed through get and set methods
    pub sides: [Side; 6],
}

#[derive(Debug, Clone)]
pub enum Rotation {
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
    pub fn new() -> Cube {
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

    pub fn new_shuffled(shuffles: u32) -> Cube {
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

    pub fn new_debug() -> Cube {
        let mut new = Cube::new();
        for i in 0..6 {
            for j in 0..8 {
                new.sides[i].colors[j] = (i * 8 + j) as u8;
            }
        }
        new
    }

    pub fn rotate(&self, rotation: &Rotation) -> Cube {
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

    pub fn get_at(&self, side: usize, face: usize) -> u8 {
        if face == 4 {
            panic!("Cannot set center piece")
        }
        let face_pos = if face < 4 {face} else {face - 1};
        self.sides[side].colors[face_pos]
    }
    pub fn set_at(&mut self, side: usize, face: usize, color: u8) {
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


fn get_solution_from_hashmap(
    middle_hash: Hash,
    middle_cube: &Cube,
    hash_map: &HashMap<Hash, Option<Rotation>>,
    reverse: bool
) -> Option<Vec<Rotation>> {
    let mut solution_rotations: Vec<Rotation> = vec![];

    // Find route forward
    let mut lookup_cube = middle_cube.clone();
    let mut lookup_hash = middle_hash;
    loop {
        let lookup_rotation = hash_map.get(&lookup_hash).unwrap();
        match lookup_rotation {
            Some(rotation) => {
                // println!("ROTATION {:?}", rotation);
                lookup_cube = lookup_cube.rotate(&rotation.reverse());
                lookup_hash = lookup_cube.get_hash();
                solution_rotations.push(if reverse {rotation.reverse()} else {rotation.clone()});
            },
            None => {
                break
            }
        }
    }
    Some(solution_rotations)
}


fn get_solution_from_two_way_hashmaps(
    middle_hash: Hash,
    middle_cube: &Cube,
    begin_hashes: &HashMap<Hash, Option<Rotation>>,
    end_hashes: &HashMap<Hash, Option<Rotation>>
) -> Option<Vec<Rotation>> {
    let mut solution_rotations: Vec<Rotation> = vec![];

    let forward_solution = get_solution_from_hashmap(middle_hash, middle_cube, end_hashes, false).unwrap();
    for rotation in forward_solution.iter().rev() {
        solution_rotations.push(rotation.clone());
    }

    let backward_solution = get_solution_from_hashmap(middle_hash, middle_cube, begin_hashes, true).unwrap();
    for rotation in backward_solution.iter() {
        solution_rotations.push(rotation.clone());
    }

    Some(solution_rotations)
}

fn extend_breath_first_search(
    old_cubes: & Vec<Cube>,
    new_cubes: &mut Vec<Cube>,
    hashes: &mut HashMap<Hash, Option<Rotation>>,
    other_hashes: &HashMap<Hash, Option<Rotation>>,
) -> Option<Vec<Rotation>> {
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

    let mut hash_collisions = 0;

    for cube in old_cubes.iter() {
        let rotations = cube.get_rotations();
        for rotation in all_rotations.iter() {
            // for rotation in rotations {
            let rotated_cube = cube.rotate(&rotation);
            let hash = rotated_cube.get_hash();
            if hashes.contains_key(&hash) {
                hash_collisions += 1;
                continue;
            }
            hashes.insert(hash, Some(rotation.clone()));
            if other_hashes.contains_key(&hash) {
                let solution_rotations = get_solution_from_two_way_hashmaps(
                    hash,
                    &rotated_cube,
                    &hashes,
                    &other_hashes,
                ).unwrap();
                return Some(solution_rotations)
            }
            new_cubes.push(rotated_cube);
        }
    }

    return None
}

fn rotations_reversed(rotations: &Vec<Rotation>) -> Vec<Rotation> {
    let mut reversed_rotations: Vec<Rotation> = vec![];
    for rotation in rotations.iter().rev() {
        reversed_rotations.push(rotation.reverse());
    }
    reversed_rotations
}

pub fn solve_cube_two_way_breath_first(start_cube: &Cube, end_cube: &Cube) -> Option<Vec<Rotation>> {
    let mut a_hashes: HashMap<Hash, Option<Rotation>> = HashMap::new();
    let mut a_old_cubes: Vec<Cube> = vec![end_cube.clone()];
    let mut a_new_cubes: Vec<Cube> = Vec::new();
    a_hashes.insert(end_cube.get_hash(), None);

    let mut b_hashes: HashMap<Hash, Option<Rotation>> = HashMap::new();
    let mut b_old_cubes: Vec<Cube> = vec![start_cube.clone()];
    let mut b_new_cubes: Vec<Cube> = Vec::new();
    b_hashes.insert(start_cube.get_hash(), None);

    for _ in 0..10 {
        // 1 step of front
        println!("Front step");
        let found_solution = extend_breath_first_search(
            &a_old_cubes,
            &mut a_new_cubes,
            &mut a_hashes,
            &b_hashes,
        );
        match found_solution {
            Some(solution) => {
                println!("FOUND SOLUTION REVERSE");
                return Some(solution)
            },
            None => {}
        }
        a_old_cubes = a_new_cubes;
        a_new_cubes = Vec::new();

        // 1 step of back
        println!("Back step");
        let found_solution = extend_breath_first_search(
            &b_old_cubes,
            &mut b_new_cubes,
            &mut b_hashes,
            &a_hashes,
        );
        match found_solution {
            Some(solution) => {
                println!("FOUND SOLUTION");
                return Some(rotations_reversed(&solution))
            },
            None => {}
        }
        b_old_cubes = b_new_cubes;
        b_new_cubes = Vec::new();
    }

    None
}
