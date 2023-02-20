use std::collections::HashMap;

pub fn median(numbers: &[i32]) -> f32 {
    let sorted = {
        let mut sorted = numbers.to_vec();
        sorted.sort();
        sorted
    };
    let middle = sorted.len() / 2;
    if sorted.len() % 2 == 0 { (sorted[middle] + sorted[middle - 1]) as f32 / 2.0 }
    else { sorted[middle] as f32 }
}

pub fn mode(numbers: &[i32]) -> HashMap<i32, i32> {
    let mut map = HashMap::new();
    for &number in numbers {
        *map.entry(number).or_insert(0) += 1;
    }
    map
}
