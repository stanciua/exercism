pub struct PascalsTriangle {
    rows: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { rows: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut v = Vec::new();
        match self.rows {
            0 => return v,
            1 => {
                v.push(vec![1]);
            }
            2 => {
                v.push(vec![1]);
                v.push(vec![1, 1]);
            }
            _ => {
                v.push(vec![1]);
                v.push(vec![1, 1]);
                for _ in 0..self.rows - 2 {
                    let current_row: Vec<u32>;
                    {
                        let lgth = v.last().unwrap().len();
                        let last_row = v.last().unwrap();
                        current_row = vec![1]
                            .into_iter()
                            .chain((0..lgth - 1)
                                .into_iter()
                                .zip((1..lgth).into_iter())
                                .map(|(x, y)| last_row[x] + last_row[y]))
                            .chain(vec![1].into_iter())
                            .collect::<Vec<_>>();
                    }

                    v.push(current_row);
                }
            }
        }
        v
    }
}
