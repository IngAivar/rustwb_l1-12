use std::collections::HashSet;

fn intersection(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    set1.intersection(set2).cloned().collect()
}

fn main() {
    let set1: HashSet<i32> = vec![1, 2, 3, 4].into_iter().collect();
    let set2: HashSet<i32> = vec![3, 4, 5, 6].into_iter().collect();

    let intersection_set = intersection(&set1, &set2);
    println!("Пересечение: {:?}", intersection_set);
}