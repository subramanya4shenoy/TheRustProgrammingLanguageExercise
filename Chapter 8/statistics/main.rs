use std::collections::HashMap;

fn main() {
    let array_num = vec![12.3, 12.3, 12.3, 12.3, 12.3, 12.3, 12.3, 1.45, 1.45, 1.45, 2.0, 1.45, 1.45, 19.4, 19.4, 19.4];
    println!("mean: {}, median: {}", mean_array(&array_num), mode_array(&array_num));
}

fn mean_array(arr: &[f64]) -> f64 {
    let arr_length: usize = arr.len();
    let mut sum: f64 = 0.0;
    for val in arr {
        sum += val;
    }
    sum/arr_length as f64
}

fn mode_array(arr: &Vec<f64>) -> u8 {
    let mut arr_median: HashMap<String, u8> = HashMap::new();
    for val in arr {
        let key:String = match val {
            val if !val.is_nan() => val.to_string(),
            _ => String::from("0")
        };
        let count = arr_median.entry(key).or_insert(0);
        *count += 1;
    }
    
    let mut max: u8 = 0;
    let mut max_key = String::new();
    for (k, v) in arr_median {
        if max == 0 {
            max = v;
            max_key = k;
        } else if v >= max {
             max = v;
             max_key = k;
        }
    }
    println!("{max_key}");
    max
}