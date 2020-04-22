#[derive(PartialOrd, PartialEq, Debug, Clone)]
pub struct Point {
        x: usize,
        y: usize,
}

pub fn spiral_matrix(input_size: u32) -> Vec<Vec<u32>> {
    let size: usize = input_size as usize;
    let no_of_layers: usize = (match size {
        x if x % 2 == 0 => x / 2,
        _ => size / 2 + 1
    }) as usize;

    let mut output:Vec<Vec<u32>> = vec![vec![0 as u32; size];  size];
    let mut counter = 1;

    for layer in 0..no_of_layers as usize {
        let top_left = Point { x: layer, y: layer };
        let top_right = Point { x: layer, y: size-layer-1 };
        let bottom_right = Point { x: size-layer-1, y: size-layer-1 };
        let bottom_left = Point { x: size-layer-1, y: layer };

        let mut layer_values = Vec::new();

        for point in 0..(size- (2 * layer)) {
            layer_values.push((top_left.y + point, top_left.x));
            output[top_left.x][top_left.y + point] = counter;
            counter += 1;
        }


        for point in 1..(size- (2 * layer)) {
            layer_values.push((top_right.y, top_right.x + point));
            output[top_right.x + point][top_right.y] = counter;
            counter += 1;
        }

        for point in 1..(size- (2 * layer)) {
            layer_values.push((bottom_right.x - point, bottom_right.y));
            output[bottom_right.y][bottom_right.x - point] = counter;
            counter += 1;
        }

        for point in 1..(size- (2 * layer) - 1) {
            layer_values.push((bottom_left.y, bottom_left.x - point));
            output[bottom_left.x - point][bottom_left.y] = counter;
            counter += 1;
        }
    }
    output
}
