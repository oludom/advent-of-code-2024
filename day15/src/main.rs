use std::{fmt::Debug, fs::File, io::{self, Read, Write}, path::Path, thread, time::Duration};

use crossterm::{cursor, style, terminal, ExecutableCommand, QueueableCommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = read_file("../data/day15.txt")?;
//     let data = "##########
// #..O..O.O#
// #......O.#
// #.OO..O.O#
// #..O@..O.#
// #O#..O...#
// #O..O..O.#
// #.OO.O.OO#
// #....O...#
// ##########

// <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
// vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
// ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
// <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
// ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
// ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
// >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
// <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
// ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
// v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^".to_string();
    // println!("{}", data);

    let lines: Vec<&str> = data.split("\n").collect();

    let width = get_width(&lines[0]);
    let height = get_height(&lines);

    let mut wh = Warehouse::new(&lines[0..height].to_vec(), width, height);
    // println!("warehouse: {:?}", wh);
    wh.write_to_terminal()?;

    let moves = lines[height..].to_vec();
    for mov in moves {
        for m in mov.chars() {
            wh.flyt(m);
            // println!("Move: {} {:?}", m, wh);
            wh.write_to_terminal()?;
        }
    }
    
    // println!("width: {width}, height: {height}");
    // println!("result: {}", wh.get_sum());

    Ok(())
}



fn read_file(filename: impl AsRef<Path>) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}



fn get_width(data: &str) -> usize {
    data.len()
}

fn get_height(data: &Vec<&str>) -> usize {
    data.iter().filter(|line| line.starts_with("#")).count()
}

enum Object {
    Robot,  // @
    Box,    // O
    Wall,   // #
    Air,    // .
}

struct Warehouse {
    map: Vec<Object>,
    pos: i32,
    width: i32,
    height: i32,
}


impl Warehouse {
    pub fn new(data: &Vec<&str>, width: usize, height: usize) -> Self {
        let mut pos = 0;
        let mut map = vec![];
        for h in 0..height {
            for w in 0..width {
                let c = &data[h][w..w+1];
                match c {
                    "@" => {pos = (h * width + w) as i32; map.push(Object::Robot)},
                    "O" => map.push(Object::Box),
                    "#" => map.push(Object::Wall),
                    "." => map.push(Object::Air),
                    _ => continue
                }
            }
        }
        Self { map, pos, width: width as i32, height: height as i32 }
    }

    pub fn flyt(&mut self, c: char) {
        let MOVE_LEFT: i32 = -1;
        let MOVE_RIGHT: i32 = 1;
        let MOVE_UP: i32 = -(self.width as i32);
        let MOVE_DOWN: i32 = self.width as i32;

        let (h, w) = self.to_coord(self.pos);

        let (vel, lim) = match c {
            '<' => (MOVE_LEFT, w),
            '>' => (MOVE_RIGHT, self.width - w),
            '^' => (MOVE_UP, h),
            'v' => (MOVE_DOWN, self.height - h),
            _ => return,
        };

        if self.can_move(vel, lim) {
            self.make_move(vel, lim);
        }
    }

    fn can_move(&self, vel: i32, lim: i32) -> bool {
        let cpos = self.pos;
        for i in 1..lim {
            let idx = cpos + vel * i;
            let c = self.map.get(idx as usize).unwrap();
            match c {
                &Object::Air => return true,
                &Object::Box => continue,
                &Object::Wall => return false,
                &Object::Robot => panic!(),
            }
        }
        false
    }

    fn make_move(&mut self, vel: i32, lim: i32) {
        let cpos = self.pos;
        let mut last = Object::Robot;
        // set current position to empty as robot is moving away
        self.map[cpos as usize] = Object::Air;
        self.pos = cpos + vel;
        for i in 1..lim {
            let idx = cpos + vel * i;
            let c = self.map.get(idx as usize).unwrap().to_owned();
            
            match c {
                Object::Air => {
                    self.map[idx as usize] = last;
                    return;
                },
                Object::Box => {
                    self.map[idx as usize] = last;
                    last = Object::Box;
                },
                Object::Wall => return,
                Object::Robot => panic!(),
            }
        }
    }

    fn to_coord(&self, pos: i32) -> (i32, i32) {
        let w: i32 = pos % self.width as i32;
        let h: i32 = pos / self.height as i32;
        (h, w)
    }

    pub fn get_sum(&self) -> usize {
        let as_gps = |h, w| 100 * h + w;
        let mut sum = 0;
        for (i, o) in self.map.iter().enumerate() {
            let (h, w) = self.to_coord(i as i32);
            match o {
                Object::Box => {
                    sum += as_gps(h, w);
                },
                _ => continue,
            }
        }
        sum as usize
    }

    pub fn write_to_terminal(&self) -> io::Result<()>  {
        let mut stdout = io::stdout();
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;

        for (i, o) in self.map.iter().enumerate() {
            let (h, w) = self.to_coord(i as i32);
            stdout.queue(cursor::MoveTo(w as u16, h as u16))?;
            let c = match o {
                &Object::Robot => "@",
                &Object::Box => "O",
                &Object::Wall => "#",
                &Object::Air => ".",
            };
            stdout.queue(style::Print(c))?;
        }
        stdout.flush()?;
        thread::sleep(Duration::from_millis(10));
        Ok(())
    }
}

impl Debug for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, o) in self.map.iter().enumerate() {
            if i % self.width as usize == 0 {
                writeln!(f)?
            }
            match o {
                &Object::Robot => write!(f, "@")?,
                &Object::Box => write!(f, "O")?,
                &Object::Wall => write!(f, "#")?,
                &Object::Air => write!(f, ".")?,
            }
        }
        writeln!(f, "")
    }
}