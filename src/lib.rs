pub fn quicksort<T: PartialOrd>(arr: &mut [T]) {
    if arr.len() > 1 {
        let len = arr.len();
        let pivot = len - 1;
        let mut left: usize = 0;
        let mut right: usize = len - 1;

        while left <= right {
            while arr[left] < arr[pivot] {
                left += 1;
            }
            while arr[right] > arr[pivot] {
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

pub fn bubblesort<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n - 1 {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubblesort_i32() {
        let mut arr: Vec<i32> = vec![9, 7, 6, 4, 2, 1, 3, 5, 8];
        let sorted: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        bubblesort(&mut arr);
        assert_eq!(arr, sorted);
    }

    #[test]
    fn bubblesort_u8() {
        let mut arr: Vec<u8> = vec![2, 8, 1, 4, 7];
        let sorted: Vec<u8> = vec![1, 2, 4, 7, 8];
        bubblesort(&mut arr);
        assert_eq!(arr, sorted);
    }

    #[test]
    fn bubblesort_f64() {
        let mut arr: Vec<f64> = vec![4.0, 9.1, 5.2, 2.5, 2.0];
        let sorted: Vec<f64> = vec![2.0, 2.5, 4.0, 5.2, 9.1];
        bubblesort(&mut arr);
        assert_eq!(arr, sorted);
    }

    #[test]
    fn bubblesort_str() {
        let mut arr: Vec<&str> = vec!["ghi", "abc", "def", "jkl"];
        let sorted: Vec<&str> = vec!["abc", "def", "ghi", "jkl"];
        bubblesort(&mut arr);
        assert_eq!(arr, sorted);
    }

    #[test]
    fn bubblesort_string() {
        let mut arr: Vec<String> = vec![
            String::from("ghi"),
            String::from("abc"),
            String::from("def"),
            String::from("jkl"),
        ];
        let sorted: Vec<String> = vec![
            String::from("abc"),
            String::from("def"),
            String::from("ghi"),
            String::from("jkl"),
        ];
        bubblesort(&mut arr);
        assert_eq!(arr, sorted);
    }

    #[test]
    fn quicksort_i32() {
        let mut arr: Vec<i32> = vec![9, 7, 6, 4, 2, 1, 3, 5, 8];
        let sorted: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        quicksort(&mut arr);
        assert_eq!(arr, sorted);
    }

    #[test]
    fn quicksort_u8() {
        let mut arr: Vec<u8> = vec![2, 8, 1, 4, 7];
        let sorted: Vec<u8> = vec![1, 2, 4, 7, 8];
        quicksort(&mut arr);
        assert_eq!(arr, sorted);
    }

    #[test]
    fn quicksort_f64() {
        let mut arr: Vec<f64> = vec![4.0, 9.1, 5.2, 2.5, 2.0];
        let sorted: Vec<f64> = vec![2.0, 2.5, 4.0, 5.2, 9.1];
        quicksort(&mut arr);
        assert_eq!(arr, sorted);
    }

    #[test]
    fn quicksort_str() {
        let mut arr: Vec<&str> = vec!["ghi", "abc", "def", "jkl"];
        let sorted: Vec<&str> = vec!["abc", "def", "ghi", "jkl"];
        quicksort(&mut arr);
        assert_eq!(arr, sorted);
    }

    #[test]
    fn quicksort_string() {
        let mut arr: Vec<String> = vec![
            String::from("ghi"),
            String::from("abc"),
            String::from("def"),
            String::from("jkl"),
        ];
        let sorted: Vec<String> = vec![
            String::from("abc"),
            String::from("def"),
            String::from("ghi"),
            String::from("jkl"),
        ];
        quicksort(&mut arr);
        assert_eq!(arr, sorted);
    }
}
