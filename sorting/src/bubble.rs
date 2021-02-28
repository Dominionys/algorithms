use std::cmp::PartialOrd;

pub fn sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut swapped = false;

        for j in 0..arr.len() - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sort;
    crate::base_cases!(sort);
}
