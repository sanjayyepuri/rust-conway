use std::cmp;
use std::fmt;
use core::ops::Range;

pub struct ConwayBoard<const H: usize, const W: usize> {
    board: [[bool; W]; H],
}

impl<const H: usize, const W: usize> ConwayBoard<H, W> {
    pub fn new(board: [[bool; W]; H]) -> ConwayBoard<H, W> {
        ConwayBoard { board }
    }

    fn num_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count: u8 = 0;
        for i in neighborhood::<H>(y) {
            for j in neighborhood::<W>(x) {
                if i == y && j == x { continue; }
                count += self.board[i][j] as u8;
            }
        }

        return count;
    }

    pub fn transition(&mut self) {
        let mut next_gen = self.board;
        for y in 0..H {
            for x in 0..W {
                let n = self.num_neighbors(x, y);
                if n < 2 || n > 3 {
                    next_gen[y][x] = false
                } else if n == 3 {
                    next_gen[y][x] = true;
                }
            }
        }

        self.board = next_gen;
    }
}

fn neighborhood<const B: usize>(center: usize) -> Range<usize> {
    let lb = if center > 0  { center - 1 } else { center };
    let ub = cmp::min(center + 2, B);
    lb..ub
}

impl<const H: usize, const W: usize> fmt::Display for ConwayBoard<H, W>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        for i in 0..H {
            for j in 0..W {
                if self.board[i][j] {
                    write!(f, "■ ").unwrap();
                } else {
                    write!(f, "  ").unwrap();
                }
            }
            writeln!(f).unwrap();
        }
        writeln!(f)
    }
}


static _BLINKER: [[bool; 5]; 5] = to_boolean([
    [0,0,0,0,0], 
    [0,0,0,0,0],
    [0,1,1,1,0],
    [0,0,0,0,0],
    [0,0,0,0,0],
]);

static _TOAD: [[bool; 6]; 6] = to_boolean([
    [0,0,0,0,0,0],
    [0,0,0,0,0,0],
    [0,1,1,1,0,0],
    [0,0,1,1,1,0],
    [0,0,0,0,0,0],
    [0,0,0,0,0,0],
]);

pub static _GLIDER_GUN: [[bool; 36]; 21] = to_boolean([
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,1,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,1,1],
    [0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,1,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,1,1],
    [1,1,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [1,1,0,0,0,0,0,0,0,0,1,0,0,0,1,0,1,1,0,0,0,0,1,0,1,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
]);

const fn to_boolean<const H: usize, const W: usize>(arr: [[i32; W]; H]) -> [[bool; W]; H] {
    let mut bool_arr =  [[false; W]; H];
    let mut i = 0;
    let mut j = 0;
    while i < H {
        while j < W {
            bool_arr[i][j] = arr[i][j] == 1;   
            j += 1;
        }
        i += 1;
    }
    return bool_arr;
}
