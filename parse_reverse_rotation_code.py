data = """
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
"""

for line in data.split("\n"):
    if "fn rotate" in line:
        print(line[:15] + "_r" + line[15:])
    elif "new_cube.set_at" in line:
        line_data = [l for l in line]
        line_data[24] = line[42]
        line_data[24 + 3] = line[42 + 3]
        line_data[42] = line[24]
        line_data[42 + 3] = line[24 + 3]
        print("".join(line_data))
    else:
        print(line)
