pub struct RailFence {
    rails: u32
}

impl RailFence {
    pub fn generate_zig_zag(n: u32, length: u32) -> Vec<(u32, u32)> {
        let mut col_down = (0..n).collect::<Vec<u32>>();
        let mut col_up = (1..n-1).collect::<Vec<u32>>();
        let mut counter = 0;
        let mut output: Vec<(u32, u32)> = Vec::new();

        col_up.reverse();
        col_down.extend(col_up);

        let mut citer_row = col_down.iter().cycle();

        while counter < length {
            output.push((*citer_row.next().unwrap(), counter));

            counter += 1;
        }

        output
    }

    pub fn new(rails: u32) -> RailFence {
        RailFence {
            rails: rails
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let cipher_chars: Vec<char> = text.chars().collect();
        let height = self.rails;
        let string_length = cipher_chars.len() as usize;

        let matrix = RailFence::generate_zig_zag(height, string_length as u32);
        let mut matrix_sorted = matrix.clone();

        let mut output: Vec<Vec<char>> = Vec::new();
        let mut encoded_string: String = String::new();

        matrix_sorted.sort_by_key(|x| x.0);

        for _x in 0..height {
            output.push(vec!['-'; string_length as usize]);
        }

        for (_index, item) in matrix.iter().enumerate() {
            output[item.0 as usize][item.1 as usize] = cipher_chars[_index];

        }

        for (_index, item) in matrix_sorted.iter().enumerate() {
            encoded_string.push(output[item.0 as usize][item.1 as usize]);
        }

        encoded_string
    }

    pub fn decode(&self, cipher: &str) -> String {
        let cipher_chars: Vec<char> = cipher.chars().collect();
        let height = self.rails;
        let string_length = cipher_chars.len() as usize;

        let matrix = RailFence::generate_zig_zag(height, string_length as u32);
        let mut matrix_sorted = matrix.clone();

        let mut output: Vec<Vec<char>> = Vec::new();
        let mut decoded_string: String = String::new();

        matrix_sorted.sort_by_key(|x| x.0);

        for _x in 0..height { 
            output.push(vec!['-'; string_length as usize]);
        }

        for (index, item) in matrix_sorted.iter().enumerate() {
            output[item.0 as usize][item.1 as usize] = cipher_chars[index];
        }

        for (_index, item) in matrix.iter().enumerate() {
            decoded_string.push(output[item.0 as usize][item.1 as usize]);
        }

        decoded_string
    }
}
