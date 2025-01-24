pub mod sorting {
    use std::fmt::{Debug, Display};
    pub fn bubble_sort<T>(nums: &mut Vec<T>)
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
    }

    pub fn selection_sort<T>(nums: &mut Vec<T>)
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
    }

    pub fn insertion_sort<T>(nums: &mut Vec<T>)
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
    }

    fn merge<T>(nums: &mut Vec<T>, start: usize, mid: usize, end: usize)
    where
        T: PartialEq + PartialOrd + Display + Debug + Copy,
    {
        let mut i: usize = start;
        let mut j: usize = mid + 1;
        println!("{i}, {j}, {start}, {mid}, {end}, {:?}", nums);
        let mut sorted: Vec<T> = Vec::new();
        while i <= mid && j <= end {
            println!("i: {i}, j: {j}");
            if nums[i] <= nums[j] {
                sorted.push(nums[i]);
                i += 1;
            } else {
                sorted.push(nums[j]);
                j += 1;
            }
        }
        while i <= mid {
            sorted.push(nums[i]);
            i += 1;
        }
        while j <= end {
            sorted.push(nums[j]);
            j += 1;
        }
        println!("sorted: {:?}", sorted);
        j = start;
        for i in sorted {
            nums[j] = i;
            j += 1;
        }
    }

    pub fn merge_sort<T>(nums: &mut Vec<T>, start: usize, end: usize)
    where
        T: PartialEq + PartialOrd + Display + Debug + Copy,
    {
        if start >= end {
            return;
        }

        let mid = start + (end - start) / 2;

        merge_sort(nums, start, mid);
        merge_sort(nums, mid + 1, end);
        merge(nums, start, mid, end);
    }

    pub fn quick_sort<T>(nums: &mut Vec<T>, start: usize, end: usize)
    where
        T: PartialEq + PartialOrd + Display + Debug + Copy,
    {
        if start >= end {
            return;
        }
        let pivot = nums[start];
        let mut p = start + 1;
        let mut q = end;
        while p <= q {
            if nums[p] <= pivot {
                p += 1;
            }
            if nums[q] >= pivot {
                q -= 1;
            }
            if nums[p] > pivot && nums[q] < pivot && p < q {
                nums.swap(p, q);
                p += 1;
                q -= 1;
            }
        }
        if pivot > nums[q] {
            nums.swap(start, q);
        }
        if q != 0 {
            quick_sort(nums, start, q - 1);
        }
        quick_sort(nums, q + 1, end);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_bubble_sort() {
        let mut nums = vec![5, 3, 2, 6, 0];
        sorting::bubble_sort(&mut nums);
        assert_eq!(nums, vec![0, 2, 3, 5, 6]);
    }

    #[test]
    fn mock_selection_sort() {
        let mut nums = vec![5, 3, 2, 6, 0];
        sorting::selection_sort(&mut nums);
        assert_eq!(nums, vec![0, 2, 3, 5, 6]);
    }

    #[test]
    fn mock_insertion_sort() {
        let mut nums = vec![5, 3, 2, 6, 0];
        sorting::insertion_sort(&mut nums);
        assert_eq!(nums, vec![0, 2, 3, 5, 6]);
    }

    #[test]
    fn mock_merge_sort() {
        let mut nums = vec![5, 3, 2, 6, 0];
        let n = nums.len();
        sorting::merge_sort(&mut nums, 0, n - 1);
        assert_eq!(nums, vec![0, 2, 3, 5, 6]);
    }
    #[test]
    fn mock_quick_sort() {
        let mut nums = vec![5, 3, 2, 6, 0];
        let n = nums.len();
        sorting::quick_sort(&mut nums, 0, n - 1);
        assert_eq!(nums, vec![0, 2, 3, 5, 6]);
    }
}
