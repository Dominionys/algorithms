use std::cmp::PartialOrd;

pub fn sort<T: PartialOrd>(arr: &mut [T]) {
    if arr.len() > 0 {
        slow_sort(0, arr.len() - 1, arr);
    }
}

fn slow_sort<T: PartialOrd>(i: usize, j: usize, arr: &mut [T]) {
    if i >= j {
        return;
    }
    let m = (i + j) / 2;

    slow_sort(i, m, arr);
    slow_sort(m + 1, j, arr);

    if arr[j] < arr[m] {
        arr.swap(j, m);
    }
    slow_sort(i, j - 1, arr);
}

#[cfg(test)]
mod tests {
    use super::sort;
    crate::base_cases!(sort);
}
