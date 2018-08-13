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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_ints() {
        let mut arr = vec![9, 7, 6, 4, 2, 1, 3, 5, 8];
        let sorted = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        quicksort(&mut arr);
        assert_eq!(arr, sorted);
    }
}
