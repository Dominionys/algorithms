use std::cmp::PartialOrd;

pub fn sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 1..arr.len() {
        for j in (1..=i).rev() {
            if arr[j - 1] > arr[j] {
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
