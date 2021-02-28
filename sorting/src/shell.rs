use std::cmp::PartialOrd;

pub fn sort<T: PartialOrd>(arr: &mut [T]) {
    let mut sublist_count = arr.len() / 2;
    while sublist_count > 0 {
        for pos in 0..sublist_count {
            insertion_sort(arr, pos, sublist_count);
        }
        sublist_count /= 2;
    }
}

pub fn insertion_sort<T: PartialOrd>(arr: &mut [T], start: usize, gap: usize) {
    let indexes = (start..arr.len()).step_by(gap).collect::<Vec<usize>>();
    for i in 1..indexes.len() {
        for j in (1..=i).rev() {
            let left_index = indexes[j - 1];
            let right_index = indexes[j];

            if arr[left_index] > arr[right_index] {
                arr.swap(j - 1, j);
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sort;
    crate::base_cases!(sort);
}
