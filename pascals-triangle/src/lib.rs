pub struct PascalsTriangle {
    data: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut pascal = PascalsTriangle { data: vec![] };
        for _ in 0..row_count {
            pascal.add_row();
        }
        pascal
    }

    pub fn add_row(&mut self) {
        let last_row = self.data.last();
        match last_row {
            None => self.data.push(vec![1]),
            Some(row) => {
                let mut new_row: Vec<u32> = vec![1];
                for i in 0..row.len() - 1 {
                    new_row.push(row[i] + row[i + 1]);
                }
                new_row.push(1);
                self.data.push(new_row);
            }
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.data.clone()
    }
}
