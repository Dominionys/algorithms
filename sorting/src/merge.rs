use std::cmp::PartialOrd;

pub fn top_down_sort<T: PartialOrd>(arr: &mut [T]) {
    let middle_index = arr.len() / 2;

    let (left, right) = arr.split_at_mut(middle_index);

    if left.len() > 1 {
        top_down_sort(left);
    }
    if right.len() > 1 {
        top_down_sort(right);
    }
    top_down_merge(arr);
}

pub fn top_down_merge<T: PartialOrd>(arr: &mut [T]) {
    if arr.len() == 2 {
        if arr[0] > arr[1] {
            arr.swap(0, 1);
        }
    } else if arr.len() > 2 {
        let mut left: usize = 0;
        let mut mid = arr.len() / 2;
        let mut right = mid;
        let end = arr.len();

        if arr[mid - 1] <= arr[right] {
            return;
        }

        while left < mid && right < end {
            if arr[left] <= arr[right] {
                left += 1;
            } else {
                arr.swap(left, right);

                let mut index = left + 1;

                while index != right {
                    arr.swap(index, right);
                    index += 1;
                }

                left += 1;
                mid += 1;
                right += 1;
            }
        }
    }
}

#[cfg(test)]
mod top_down_sort_tests {
    use super::top_down_sort;

    crate::base_cases!(top_down_sort);
}
