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
mod tests_lomuto {
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

pub fn partition_hoare<T: PartialOrd>(arr: &mut [T], low: usize, high: usize) -> usize {
    let mut pivot = low;
    let mut i = low;
    let mut j = high;

    loop {
        while arr[i] < arr[pivot] {
            i += 1;
        }

        while arr[j] > arr[pivot] {
            j -= 1;
        }

        if i >= j {
            return j;
        }

        if pivot == i {
            pivot = j;
        } else if pivot == j {
            pivot = i;
        }

        arr.swap(i, j);

        i += 1;
        j -= 1;
    }
}

pub fn quicksort_hoare<T: PartialOrd>(arr: &mut [T], low: usize, high: usize) {
    if low < high {
        let p = partition_hoare(arr, low, high);
        quicksort_hoare(arr, low, p);
        quicksort_hoare(arr, p + 1, high);
    }
}

#[cfg(test)]
mod tests_hoare {
    use super::*;

    #[test]
    fn partition_hoare_test() {
        let mut arr = [8, 0, 3, 9, 2, 14, 10, 27, 1, 5, 8, -1, 26];

        let low: usize = 0;
        let high = arr.len() - 1;
        partition_hoare(&mut arr, low, high);

        let res = [-1, 0, 3, 8, 2, 5, 1, 27, 10, 14, 9, 8, 26];
        assert_eq!(arr, res);
    }

    fn test_quicksort_hoare<T: PartialOrd>(arr: &mut [T]) {
        if arr.len() > 1 {
            quicksort_hoare(arr, 0, arr.len() - 1);
        }
    }

    crate::base_cases!(test_quicksort_hoare);
}
