mod grid {
    pub struct Grid {
        rows: usize,
        cols: usize,
        grid: Vec<Vec<i32>>
    }

    impl Grid {
        pub fn new(rows: usize, cols: usize) -> Grid {
            Grid { rows: rows, cols: cols, grid: vec![vec![0; rows]; cols] }
        }

        pub fn rows(&self) -> usize {
            self.rows
        }

        pub fn cols(&self) -> usize {
            self.cols
        }

        pub fn get(&self, x: usize, y: usize) -> i32 {
            self.grid[x][y]
        }
    }
}

fn main() {
    let g = grid::Grid::new(10, 5);
    println!("{} {} ::: {}", g.rows(), g.cols(), g.get(2,2));
}
