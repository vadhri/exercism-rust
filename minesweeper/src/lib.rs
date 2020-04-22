pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let generate_grid = |x: i32, y: i32, w: i32, h: i32| -> Vec<(i32, i32)>{
        let left_start_x = if x - 1 < 0 { 0 } else { x - 1 };
        let left_start_y = if y - 1 < 0 { 0 } else { y - 1 };

        let left_end_x = if x + 1 >= h - 1 { h - 1 } else { x + 1 };
        let left_end_y = if y + 1 >= w - 1 { w - 1 } else { y + 1 };

        let mut grid_coordinates = Vec::new();

        for x_coord in left_start_x..=left_end_x {
            for y_coord in left_start_y..=left_end_y {
                if (x_coord, y_coord) != (x, y) {
                    grid_coordinates.push((x_coord, y_coord));
                }
            }
        }

        grid_coordinates
    };

    let mut output = Vec::new();

    for row in minefield {
        output.push(vec![0; row.len()]);
    }

    for (rowindex, string) in minefield.iter().enumerate() {
        for (colindex, cell) in string.chars().enumerate() {
            if cell == '*' {
                output[rowindex][colindex] = -1;
                let grid = generate_grid(rowindex as i32, colindex as i32, string.len() as i32, minefield.len() as i32);
                for point in grid {
                    if output[point.0 as usize][point.1 as usize] != -1 {
                        output[point.0 as usize][point.1 as usize] += 1;
                    }
                }
            }
        }
    }

    let mut output_string = Vec::new();

    for line in output.clone() {
        let temp: String = line.into_iter().map(|i| i.to_string()).collect::<String>();
        output_string.push(temp.replace("-1", "*").replace("0", " "));
    }

    output_string
}
