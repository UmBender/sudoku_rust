use rand::thread_rng;
use rand::seq::SliceRandom;

struct Sudoku {
    map: Vec<Vec<usize>>,
    set: usize,
}

impl Sudoku {
    fn new(set: usize) -> Self {
        Sudoku {
            map: (0..(set * set))
                .map(|_| vec![0; set * set])
                .collect::<Vec<Vec<usize>>>(),
            set,
        }
    }

    fn print(&self) -> () {
        for i in self.map.iter() {
            for &j in i.iter() {
                print!("{:02} ", j);
            }
            println!();
        }
        println!();
    }

    fn next_element(&self) -> Option<(usize, usize)> {
        for (j, i) in self.map.iter().enumerate() {
            for (l, k) in i.iter().enumerate() {
                if *k == 0 {
                    return Some((l, j));
                }
            }
        }
        return None;
    }

    fn elements_in_set(&self, x: usize, y: usize) -> Vec<usize> {
        let init_x = (x / self.set) * self.set;
        let init_y = (y / self.set) * self.set;
        let mut elements = vec![];
        for i in 0..self.set {
            for j in 0..self.set {
                if self.map[i + init_y][j + init_x] != 0 {
                    elements.push(self.map[i + init_y][j + init_x]);
                }
            }
        }
        return elements;
    }
    fn elements_in_line(&self, y: usize) -> Vec<usize> {
        let mut elements = vec![];
        for i in 0..(self.set.pow(2)) {
            if self.map[y][i] != 0 {
                elements.push(self.map[y][i]);
            }
        }
        return elements;
    }
    fn elements_in_col(&self, x: usize) -> Vec<usize> {
        let mut elements = vec![];
        for i in self.map.iter() {
            if i[x] != 0 {
                elements.push(i[x]);
            }
        }
        return elements;
    }

    fn elements_not_in(&self, x: usize, y: usize) -> Vec<usize> {
        let col_ele = self.elements_in_col(x);
        let set_ele = self.elements_in_set(x, y);
        let line_ele = self.elements_in_line(y);
        let mut values = vec![];
        for i in 1..=self.set.pow(2) {
            let mut stats = false;
            stats |= col_ele.iter().any(|&x| i == x);
            stats |= set_ele.iter().any(|&x| i == x);
            stats |= line_ele.iter().any(|&x| i == x);
            if !stats {
                values.push(i);
            }
        }
        return values;
    }

    fn solve(&mut self) -> bool {
        let x;
        let y;
        match self.next_element() {
            Some(i) => (x,y) = i,
            None => return true,
        }
        let mut elements = self.elements_not_in(x, y);
        elements.shuffle(&mut thread_rng());
        for i in elements.iter() {
            self.map[y][x] = *i;
            if self.solve() {
                return true;
            }

            self.map[y][x] = 0;
        }
        return false;
    }
}

fn main() {
    let mut map = Sudoku::new(3);
    map.solve();
    map.print();
}
