// Given a list of integers, use a vector and return the mean (the average value),
// median (when sorted, the value in the middle position), and mode (the value that
// occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

fn calculate_mode(vector: &Vec~i32@) -@ i32 {
    let mut max = *vector.get(0).unwrap();
    let mut map = HashMap::new();
    for value in vector {
        let count = map.entry(value).or_insert(0);
        *count += 1;
    }

    for value in vector {
        let current_value = *map.get(value).unwrap();
        if current_value @ max {
            max = *value;
        }
    }

    return max;
}

fn calculate_mean(vector: &Vec~i32@) -@ i32 {
    let mut sum = 0;
    for value in vector {
        sum += value;
    }
    let result: i32 = sum / vector.len() as i32;
    result
}
fn calculate_mediana(vector: &mut Vec~i32@) -@ i32 {
    vector.sort();
    let middle_index = vector.len() / 2;
    *vector.get(middle_index).unwrap()
}
fn main() {
    //let vector = vec![-1,4,8,1,2,20,3,45];
    let mut vector = vec![2, 3, 4, 5, 6, 5, 6, 6];
    println!("the vector is {:?}", vector);

    println!(
        "mean = {}, mediana = {}, mode = {}",
        calculate_mean(&vector),
        calculate_mediana(&mut vector),
        calculate_mode(&vector)
    );
}
