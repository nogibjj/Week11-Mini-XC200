fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;
    
    while low <= high {
        let mid = (low + high) / 2;
        
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    
    None
}

fn main() {
    let arr = [1, 3, 5, 7, 9];
    let target = 5;
    
    match binary_search(&arr, target) {
        Some(index) => println!("{} found at index {}", target, index),
        None => println!("{} not found", target),
    }
}
