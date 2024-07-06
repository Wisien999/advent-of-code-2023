use std::boxed::Box;


#[derive(Debug, Clone)]
struct Map {
    rows: Vec<Vec<char>>,
    cached_transpose: Option<Vec<Vec<char>>>,
}
impl Map {
    fn new(rows: Vec<Vec<char>>) -> Self {
        Map { rows, cached_transpose: None }
    }

    fn are_rows_the_same(&self, row1: usize, row2: usize, allow_smudge: &mut bool) -> bool {
        let row1 = &self.rows[row1];
        let row2 = &self.rows[row2];

        for i in 0..row1.len() {
            if row1[i] != row2[i] {
                if *allow_smudge {
                    *allow_smudge = false;
                    continue;
                }
                return false;
            }
        }

        return true;
    }

    fn find_row_symmetry(&self) -> u32 {
        for last_row_before_symmetry_line in 0..self.rows.len() - 1 {

            let mut is_symmetric = true;
            let mut i = 0;
            let mut allow_smudge = true;
            while last_row_before_symmetry_line >= i && last_row_before_symmetry_line + i + 1 < self.rows.len() {
                let row_before = last_row_before_symmetry_line - i;
                let row_after = last_row_before_symmetry_line + i + 1;

                if !self.are_rows_the_same(row_before, row_after, &mut allow_smudge) {
                    is_symmetric = false;
                    break;
                }
                i += 1;
            }

            if is_symmetric && !allow_smudge {
                return (last_row_before_symmetry_line + 1) as u32;
            }

        }

        return 0;
    }

    fn transpose(&mut self) -> Box<Map> {
        if let Some(ref transposed) = self.cached_transpose {
            return Box::new(Map {
                rows: transposed.clone(),
                cached_transpose: Some(self.rows.clone()),
            });
        }
        
        if self.rows.is_empty() {
            return Box::new(Map {
                rows: Vec::new(),
                cached_transpose: None,
            });
        }

        let row_count = self.rows.len();
        let col_count = self.rows[0].len();

        let mut transposed: Vec<Vec<char>> = vec![vec![' '; row_count]; col_count];

        for i in 0..row_count {
            for j in 0..col_count {
                transposed[j][i] = self.rows[i][j];
            }
        }

        self.cached_transpose = Some(transposed.clone());

        Box::new(Map {
            rows: transposed,
            cached_transpose: None,
        })
    }

    fn find_column_symmetry(&mut self) -> u32 {
        let transposed = self.transpose();
        transposed.find_row_symmetry()
    }
}


fn summarise_map(map: &mut Map) -> u32 {
    let row_symmetry = map.find_row_symmetry();
    let column_symmetry = map.find_column_symmetry();

    return row_symmetry * 100 + column_symmetry;
}

fn summarise_maps(maps: &mut Vec<Map>) -> u32 {
    maps.iter_mut().map(|map| summarise_map(map)).sum()
}


fn main() {
    let mut maps = include_str!("day13_data.txt")
        .split("\n\n")
        .map(|part| part.split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
        )
        .map(|map| Map::new(map))
        .collect::<Vec<Map>>();

    let summary = summarise_maps(&mut maps);
    println!("Summary: {:?}", summary);
}

