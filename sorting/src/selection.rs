use std::cmp::PartialOrd;

pub fn sort<T: PartialOrd>(arr: &mut [T]) {
    if arr.len() < 2 {
        return;
    }

    for i in 0..arr.len() - 1 {
        let mut lowest = i;

        for j in (i + 1)..arr.len() {
            if arr[j] < arr[lowest] {
                lowest = j;
            }
        }

        if lowest != i {
            arr.swap(lowest, i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sort;
    crate::base_cases!(sort);
}
