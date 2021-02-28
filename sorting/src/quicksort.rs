use std::cmp::PartialOrd;

pub fn partition_lomuto<T: PartialOrd>(arr: &mut [T], low: usize, high: usize) -> usize {
    let mut i = low;

    for j in low..high {
        if arr[j] <= arr[high] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, high);
    i
}

pub fn quicksort_lomuto<T: PartialOrd>(arr: &mut [T], low: usize, high: usize) {
    if low < high {
        let p = partition_lomuto(arr, low, high);
        if p > 0 {
            quicksort_lomuto(arr, low, p - 1);
        }
        quicksort_lomuto(arr, p + 1, high);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partition_lomuto_test() {
        let mut arr = [10, 0, 3, 9, 2, 14, 26, 27, 1, 5, 8, -1, 8];

        let low: usize = 0;
        let high = arr.len() - 1;
        partition_lomuto(&mut arr, low, high);

        let res = [0, 3, 2, 1, 5, 8, -1, 8, 9, 10, 14, 26, 27];
        assert_eq!(arr, res);
    }

    #[test]
    fn partition_lomuto_test_1() {
        let mut arr = [10, 0, 3, 9, 2, 8];

        let low: usize = 0;
        let high = arr.len() - 1;
        partition_lomuto(&mut arr, low, high);

        let res = [0, 3, 2, 8, 10, 9];
        assert_eq!(arr, res);
    }

    fn test_quicksort_lomuto<T: PartialOrd>(arr: &mut [T]) {
        if arr.len() > 1 {
            quicksort_lomuto(arr, 0, arr.len() - 1);
        }
    }
    crate::base_cases!(test_quicksort_lomuto);

}
