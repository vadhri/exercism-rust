pub fn get_diamond(c: char) -> Vec<String> {
    let diamond_letter = c;
    let height_of_diamond = 2 * (diamond_letter as u32 - 'A' as u32) + 1;
    let mut out = Vec::new();

    let mut output_vector = Vec::new();

    for _row in 0..height_of_diamond {
        output_vector.push(vec![' '; height_of_diamond as usize]);
    }

    let vector_midpoint = (height_of_diamond / 2) as i32;

    for (index, r) in output_vector.iter_mut().enumerate() {
        let mut edge_left = index as i32;

        edge_left = match edge_left {
            value if value > vector_midpoint => {
            if index as i32 % vector_midpoint != 0 {
                vector_midpoint - (index as i32 % vector_midpoint)
            } else {
                0
            }},
            rest => rest
        };

        if let value = r.get_mut(vector_midpoint as usize + edge_left as usize).unwrap() {
            *value = ('A' as u8 + edge_left as u8) as char;
        };

        if let value = r.get_mut(vector_midpoint as usize - edge_left as usize).unwrap() {
            *value = ('A' as u8 + edge_left as u8) as char;
        };

        out.push(r.iter().collect::<String>());
    }
    out
}
