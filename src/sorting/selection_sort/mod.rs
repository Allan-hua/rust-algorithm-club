/// Selection sort
pub fn selection_sort(arr: &mut [i32]) {
    // Rust would skip iteration if lower bound >= upper bound.
    // Hence, `arr.len() - 1` is only a skip of last iteration.
    for i in 0..(arr.len() - 1) {
        let mut temp = i;
        for j in (i + 1)..arr.len() {
            if arr[temp] > arr[j] {
                temp = j;
            }
        }
        arr.swap(i, temp);
    }
}