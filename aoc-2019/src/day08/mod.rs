use std::collections::HashMap;

use utils::read_to_string_in_module;

pub fn run() {
    let contents = read_to_string_in_module!("input.txt");
    let mut layers = read_layers(contents);

    println!(
        "Space Image Format part1 Solution: {:?}",
        calculate_layer_value(&mut layers)
    );

    let final_image = calculate_visible_pixel_values(&mut layers);
    println!("Space Image Format part2 Solution: \n",);
    print_image(final_image);
}

#[derive(Clone, Debug)]
struct Layer {
    values: Vec<Vec<char>>,
    value_count: HashMap<char, i32>,
}
impl Layer {
    pub fn new() -> Layer {
        Layer {
            values: vec![vec![' '; 6]; 25],
            value_count: HashMap::new(),
        }
    }
}

fn calculate_visible_pixel_values(layers: &mut [Layer]) -> Vec<Vec<char>> {
    let mut values = vec![vec![' '; 6]; 25];
    for (i, row) in values.iter_mut().take(25).enumerate() {
        for (j, val) in row.iter_mut().take(6).enumerate() {
            let mut layer_index = 0;
            let mut pixel_color = layers[layer_index].values[i][j];
            while pixel_color == '2' {
                layer_index += 1;
                pixel_color = layers[layer_index].values[i][j];
            }
            *val = pixel_color;
        }
    }
    values
}

fn calculate_layer_value(layers: &mut Vec<Layer>) -> i32 {
    let mut layer_with_min0 = Layer::new();
    let mut minimum = i32::MAX;

    for layer in layers {
        let zero_count = layer.value_count.get(&'0').unwrap();
        if minimum > *zero_count {
            minimum = *zero_count;
            layer_with_min0 = layer.clone();
        }
    }
    layer_with_min0.value_count.get(&'1').unwrap() * layer_with_min0.value_count.get(&'2').unwrap()
}

fn read_layers(input: String) -> Vec<Layer> {
    let mut layers = Vec::new();
    let mut current_layer = 0;
    let mut i = 0;
    let mut j = 0;
    for ch in input.chars() {
        let curr = current_layer;
        if i == 0 && j == 0 {
            layers.push(Layer::new());
        }
        if i == 24 {
            i = 0;
            if j == 5 {
                j = 0;
                current_layer += 1;
            } else {
                j += 1;
            }
        } else {
            i += 1;
        }
        layers[curr].values[i][j] = ch;
        if let Some(&value_count) = layers[curr].value_count.get(&ch) {
            layers[curr].value_count.insert(ch, value_count + 1);
        } else {
            layers[curr].value_count.insert(ch, 1);
        }
    }
    layers
}

fn print_image(image: Vec<Vec<char>>) {
    let mut sb = String::new();
    for i in 0..6 {
        for row in image.iter().take(25) {
            match row[i] {
                '0' => sb.push(' '),
                _ => sb.push('|'),
            }
        }
        sb.push('\n');
    }
    println!("{}", sb)
}
