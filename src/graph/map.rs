pub struct MapCell {
    pub height: isize,
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl MapCell {
    pub fn position(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

pub struct Map {
    map: Vec<Vec<MapCell>>,
}

impl Map {
    pub fn with_matrix(matrix: &Vec<Vec<isize>>) -> Self {
        let mut res = Map { map: vec![] };
        for i in 0..matrix.len() {
            res.map.push(vec![]);
            for j in 0..matrix.len() {
                let cell = MapCell { height: matrix[i][j], x: i, y: j };
                res.map[i].push(cell);
            }
        }

        res
    }

    pub fn get(&self, x: usize, y: usize) -> &MapCell {
        &self.map[x][y]
    }
    pub fn get_by_tuple(&self, (x, y): (usize, usize)) -> &MapCell {
        &self.map[x][y]
    }

    pub fn neighbors(&self, cell: &MapCell) -> Vec<&MapCell> {
        let mut res = vec![];
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (x, y) = (cell.x as isize + dx, cell.y as isize + dy); // Считаем индексы соседних вершин

            if x >= 0 && x < self.map.len() as isize && y >= 0 && y < self.map[0].len() as isize {
                res.push(self.get(x as usize, y as usize));
            }
        }

        res
    }

    pub fn width(&self) -> usize {
        self.map[0].len()
    }

    pub fn height(&self) -> usize {
        self.map.len()
    }
}