use std::fmt::{Debug, Display};

pub fn bubble_sort<'a, T>(nums: &'a mut Vec<T>) -> &'a Vec<T>
where
    T: PartialEq + PartialOrd + Display + Debug,
{
    let n = nums.len();
    for i in 0..n {
        for j in 0..(n - i - 1) {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
            }
        }
    }
    return nums;
}

pub fn selection_sort<'a, T>(nums: &'a mut Vec<T>) -> &'a Vec<T>
where
    T: PartialEq + PartialOrd + Display + Debug,
{
    let n = nums.len();
    for i in 0..n {
        let mut idx = i;
        for j in i..n {
            if nums[j] < nums[idx] {
                idx = j;
            }
        }
        nums.swap(i, idx);
    }
    nums
}

pub fn insertion_sort<'a, T>(nums: &'a mut Vec<T>) -> &'a Vec<T>
where
    T: PartialEq + PartialOrd + Display + Debug,
{
    let n = nums.len();
    for i in 1..n {
        let mut j = i;
        while j > 0 && nums[j] < nums[j - 1] {
            nums.swap(j, j - 1);
            j -= 1;
        }
    }
    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_bubble_sort() {
        let mut nums = vec![5, 3, 2, 6, 0];
        bubble_sort(&mut nums);
        assert_eq!(nums, vec![0, 2, 3, 5, 6]);
    }

    #[test]
    fn mock_selection_sort() {
        let mut nums = vec![5, 3, 2, 6, 0];
        selection_sort(&mut nums);
        assert_eq!(nums, vec![0, 2, 3, 5, 6]);
    }
    #[test]
    fn mock_insertion_sort() {
        let mut nums = vec![5, 3, 2, 6, 0];
        insertion_sort(&mut nums);
        assert_eq!(nums, vec![0, 2, 3, 5, 6]);
    }
}
