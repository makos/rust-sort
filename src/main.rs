fn main() {
    let mut random = vec![9, 3, 2, 6, 1, 4, 8, 5, 7];
    println!("{:?}", random);
    quicksort(&mut random);
    println!("{:?}", random);
}

fn quicksort(arr: &mut [i32]) {
    if arr.len() > 1 {
        let len = arr.len();
        let pivot = arr[len - 1];
        let mut left: usize = 0;
        let mut right: usize = len - 1;

        while left <= right {
            while arr[left] < pivot {
                left += 1;
            }
            while arr[right] > pivot {
                right -= 1;
            }
            if left <= right {
                arr.swap(left, right);
                left += 1;
                right -= 1;
            }
        }
        quicksort(&mut arr[0..right + 1]);
        quicksort(&mut arr[left..len]);
    }
}
