use std::libc::funcs::posix88::unistd::sleep;

use std::int;

enum Cell {
    dead = 0,
    live = 1
}

struct Grid  {
    cells: ~[Cell],
    width: uint,
    height: uint
}

fn uint_to_cell(u: uint) -> Cell {
    match u {
        0 => { dead }
        1 => { live }
        _ => { fail!("Invalid Cell value"); }
    }
}

fn Grid(c: ~[Cell], w:uint, h:uint) -> ~Grid {
    ~Grid {
        cells: c,
        width: w,
        height: h
    }
}

type Point = (int, int);
type WrappedPoint = (uint, uint);

fn main() {
    let mut grid = Grid(
     [0,0,1,0,0,
      0,0,0,1,0,
      0,1,1,1,0,
      0,0,0,0,0,
      0,0,0,0,0].map(|c| uint_to_cell(*c)), 5, 5);

    print_grid(grid);

    loop {
        unsafe { sleep(1); }
        grid = evolve(grid);
        print_grid(grid);
    }
}

fn neighbours(c: &Point) -> ~[Point] {
    match *c {
        (x, y) => {
           ~[(x-1,y-1), (x,y-1), (x+1,y-1),
             (x-1,y  ),          (x+1,y  ),
             (x-1,y+1), (x,y+1), (x+1,y+1),
            ]}
    }
}

fn at(g: &Grid, c: &Point) -> Cell {
    match *wrap(c, g) {
        (x, y) => {
            g.cells[y * g.width + x]
        }
    }
}

fn wrap(c: &Point, g: &Grid) -> ~WrappedPoint {
    match *c {
        (x, y) => { ~((nwrap(x, g.width)),
                     (nwrap(y, g.height)))
        }
    }
}

fn nwrap(x: int, w: uint) -> uint {
    let r = x % (w as int);
    return if r < 0 {
        (r + (w as int)) as uint
    } else {
        r as uint
    }
}


fn density(g: &Grid, c: &Point) -> uint {
    neighbours(c).map(|n| at(g, n)).foldl(0, |a, i| {*a + (*i as uint)})
}

fn survives(g: &Grid, c:&Point) -> Cell {
    let d = density(g, c);
    match at(g, c) {
        dead => {
            match d {
                3 => { live }
                _ => { dead }
            }
        }
        live => {
            match d {
                2 | 3 => { live }
                _     => { dead }
            }
        }
    }
}

fn all_points(g: &Grid) -> ~[Point]
{
    let mut coords: ~[Point] = ~[];

    for int::range(0, g.height as int) |y| {
        for int::range(0, g.width as int) |x| {
            coords += [(x, y)];
        }
    }

    return coords;
}

fn evolve(g: &Grid) -> ~Grid {
    ~Grid {
        cells: all_points(g).map( |c| survives(g, c) ),
        width: g.width,
        height: g.height
    }
}

fn print_grid(g: &Grid)
{
    for all_points(g).each |c| {
        match *c {
            (0, _) => {println("")}
            _      => {}
        }

        match at(g, c) {
            dead => {print("-")}
            live => {print("*")}
        }
    }
}
