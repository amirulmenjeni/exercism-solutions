use std::fmt::{Debug, self};

#[derive(Debug)]
enum Tile {
    Mine,
    Annotation(Option<u8>),
}

struct Board {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

trait TileMap {
    fn from_strs(strs: &[&str]) -> Self;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn get_tile_at<'a>(&'a self, row: usize, col: usize) -> Option<&'a Tile>;
    fn set_tile_at<'a>(&'a mut self, row: usize, col: usize, tile: Tile) -> Result<&'a Tile, &'static str>;
    fn to_vec_string(&self) -> Vec<String>;
}

trait TileMapStats {
    fn count_mines_at(&self, row: usize, col: usize) -> u8;
}

impl Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:?}", self.tiles.iter().flatten())
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        self.to_vec_string().join("\n")
    }
}

impl TileMap for Board {

    fn from_strs(strs: &[&str]) -> Self {
        let mut rows = Vec::<Vec::<Tile>>::new();

        for str in strs {
            let mut cols = Vec::<Tile>::new();
            for c in str.chars() {
                cols.push(match c {
                    '*' => Tile::Mine,
                    _ => Tile::Annotation(None),
                });
            }
            rows.push(cols);
        }

        Board { 
            width: rows.first().unwrap_or(&Vec::<Tile>::new()).len(),
            height: rows.len(),
            tiles: rows,
        }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get_tile_at<'a>(&'a self, row: usize, col: usize) -> Option<&'a Tile> {
        if let Some(tiles) = self.tiles.get(row) {
            tiles.get(col)
        } else {
            None
        }
    }

    fn set_tile_at<'a>(&'a mut self, row: usize, col: usize, tile: Tile) -> Result<&'a Tile, &'static str>{
        if let Some (tiles) = self.tiles.get_mut(row) {
            if let Some (t) = tiles.get_mut(col) {
                *t = tile; 
                Ok(t)
            } else {
                Err("Can't find col")
            }
        } else {
            Err("Can't find row")
        }
    }

    fn to_vec_string(&self) -> Vec<String> {
        let mut v = Vec::<String>::new();
        self.tiles.iter().for_each(|rows| {
            let mut s = String::new();
            rows.iter().for_each(|tile| {
                match tile {
                    Tile::Mine => s.push('*'),
                    Tile::Annotation(Some(count)) => s.push(char::from_digit((*count).into(), 10).unwrap_or(' ')),
                    _ => s.push(' '),
                }
            });
            v.push(s);
        });
        v
    }
}

impl TileMapStats for Board {

    fn count_mines_at(&self, row: usize, col: usize) -> u8 {
        let mut c = 0;
        for y in -1..2_i32 {
            if (row as i32 + y) < 0 || (row as i32 + y) > self.height() as i32 {
                continue;
            }
            for x in -1..2_i32 {
                if (col as i32 + x) < 0 || (col as i32 + x) > self.width() as i32 {
                    continue;
                }
                if let Some(Tile::Mine) = self.get_tile_at((row as i32 + y) as usize, (col as i32 + x) as usize) {
                    c = c + 1;
                }
            }
        }
        c
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {

    let mut board = Board::from_strs(minefield);

    for y in 0..board.height() {
        for x in 0..board.width() {
            if let Some(Tile::Annotation(None)) = board.get_tile_at(y, x).as_mut() {
                let ann = match board.count_mines_at(y, x) {
                    0 => Tile::Annotation(None),
                    c => Tile::Annotation(Some(c)),
                };
                if let Err(err) = board.set_tile_at(y, x, ann) {
                    eprint!("{}", err);
                }
            }
        }
    }

    board.to_vec_string()
}