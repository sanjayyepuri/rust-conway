use core::ops::Range;
use std::cmp;
use std::fmt;

pub struct ConwayBoard<const H: usize, const W: usize> {
    population: [[bool; W]; H],
}

impl<const H: usize, const W: usize> ConwayBoard<H, W> {
    pub fn new(init: [[bool; W]; H]) -> ConwayBoard<H, W> {
        ConwayBoard { population: init }
    }

    pub fn next(&mut self) {
        let mut next_gen = self.population;
        _compute_next_gen(&self.population, &mut next_gen);
        self.population = next_gen;
    }
}

impl<const H: usize, const W: usize> fmt::Display for ConwayBoard<H, W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        _print_population(&self.population, f)
    }
}

pub struct ConwayBoardDoubleBuf<const H: usize, const W: usize> {
    buffers: [[[bool; W]; H]; 2],
    curr: usize,
}

impl<const H: usize, const W: usize> ConwayBoardDoubleBuf<H, W> {
    pub fn new(init: [[bool; W]; H]) -> ConwayBoardDoubleBuf<H, W> {
        let mut buffers = [[[false; W]; H]; 2];
        buffers[0] = init;

        ConwayBoardDoubleBuf { buffers, curr: 0 }
    }

    pub fn next(&mut self) {
        // there must be a better of doing this...
        let next = (self.curr + 1) % 2;
        let bufs = self.buffers.split_at_mut(1);
        
        if (next == 1) {
            _compute_next_gen(&mut bufs.0[0], &mut bufs.1[0]);
        } else {
            _compute_next_gen(&mut bufs.1[0], &mut bufs.0[0]);
        }

        self.curr = next;
    }
}

impl<const H: usize, const W: usize> fmt::Display for ConwayBoardDoubleBuf<H, W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        _print_population(&self.buffers[self.curr], f)
    }
}

fn _compute_next_gen<const H: usize, const W: usize>(
    prev_gen: &[[bool; W]; H],
    next_gen: &mut [[bool; W]; H],
) {
    for y in 0..H {
        for x in 0..W {
            let n = _sum_neighbors(x, y, prev_gen);
            if n < 2 || n > 3 {
                next_gen[y][x] = false
            } else if n == 3 {
                next_gen[y][x] = true;
            } else {
                next_gen[y][x] = prev_gen[y][x];
            }
        }
    }
}

fn _sum_neighbors<const H: usize, const W: usize>(x: usize, y: usize, gen: &[[bool; W]; H]) -> u8 {
    let mut count: u8 = 0;
    for i in _neighborhood::<H>(y) {
        for j in _neighborhood::<W>(x) {
            if i == y && j == x {
                continue;
            }
            count += gen[i][j] as u8;
        }
    }
    count
}

fn _neighborhood<const B: usize>(center: usize) -> Range<usize> {
    let lb = if center > 0 { center - 1 } else { center };
    let ub = cmp::min(center + 2, B);
    lb..ub
}

fn _print_population<const H: usize, const W: usize>(
    pop: &[[bool; W]; H],
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    for i in 0..H {
        for j in 0..W {
            if pop[i][j] {
                write!(f, "■ ").unwrap();
            } else {
                write!(f, "  ").unwrap();
            }
        }
        writeln!(f).unwrap();
    }
    writeln!(f)
}

static _BLINKER: [[bool; 5]; 5] = to_boolean([
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 1, 1, 1, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
]);

static _TOAD: [[bool; 6]; 6] = to_boolean([
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 1, 1, 1, 0, 0],
    [0, 0, 1, 1, 1, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
]);

pub static _GLIDER_GUN: [[bool; 36]; 21] = to_boolean([
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 1, 1,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 1, 1,
    ],
    [
        1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ],
    [
        1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ],
]);

const fn to_boolean<const H: usize, const W: usize>(arr: [[i32; W]; H]) -> [[bool; W]; H] {
    let mut bool_arr = [[false; W]; H];
    let mut i = 0;
    let mut j = 0;
    while i < H {
        while j < W {
            bool_arr[i][j] = arr[i][j] == 1;
            j += 1;
        }
        j = 0;
        i += 1;
    }
    return bool_arr;
}
