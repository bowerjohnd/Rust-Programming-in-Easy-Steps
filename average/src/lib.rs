use std::collections::HashMap;

pub fn mean<T> (v: &Vec<T>) -> f64
    where T: std::convert::Into<f64> + Clone {

    let mut total: f64 = 0.0;
    let size = v.len() as f64;
    for i in v {
        total += i.clone().into();
    }
    let mean: f64 = total / size;
    mean
}

pub fn median<T> (v: &Vec<T>) -> T 
    where T: Ord + Clone {

    let mut v = v.clone();
    v.sort();
    v[v.len() / 2].clone()
}

pub fn mode<T> (v: &Vec<T>) -> T
    where T: Eq + std::hash::Hash + Clone {

    let mut h = HashMap::new();

    for i in v {
        if let Some(entry) = h.get_mut(&i) {
            *entry += 1;
        }
        else {
            h.insert(i, 0);
        }
    }

    let mut key_max = v[0].clone();
    let mut max_count = 0;

    for (key, count) in h {
        if count > max_count {
            max_count = count;
            key_max = key.clone();
        }
    }
    key_max
}

pub trait Average<T> {
    fn average(&self) -> T;
}