pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows = Vec::new();
        for i in 0..row_count {
            let mut row = Vec::new();
            row.push(1);

            for item in 0..i {
                row.push(row[item as usize] * ( i - item ) / (item + 1));
            }
            rows.push(row);
        }

        PascalsTriangle {
            rows: rows
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
