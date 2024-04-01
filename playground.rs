// 1. Check if a string is a palindrome
fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

// 2. Find the index of the first occurrence of a number in a sorted array
fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target)
}

// 3. Find the shortest word in a string
fn shortest_word(s: &str) -> &str {
    s.split_whitespace().min_by_key(|word| word.len()).unwrap_or("")
}

// 4. Check if a number is prime
fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=n / 2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// 5. Find the median of a sorted array
fn median(arr: &[i32]) -> f64 {
    let mid = arr.len() / 2;
    if arr.len() % 2 == 0 {
        (arr[mid - 1] + arr[mid]) as f64 / 2.0
    } else {
        arr[mid] as f64
    }
}

// 6. Find the longest common prefix of a set of strings
fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return String::new();
    }
    let mut prefix = strs[0].clone();
    for s in strs.iter().skip(1) {
        while !s.starts_with(&prefix) {
            prefix.pop();
        }
    }
    prefix
}

// 7. Find the kth smallest element in an array
fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();
    sorted_arr.get(k - 1).copied()
}

// 8. Find the maximum depth of a binary tree
use std::cmp;

struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn max_depth(root: Option<&Box<TreeNode>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let left_depth = TreeNode::max_depth(node.left.as_ref().map(|x| x));
                let right_depth = TreeNode::max_depth(node.right.as_ref().map(|x| x));
                cmp::max(left_depth, right_depth) + 1
            }
        }
    }
}

// 9. Reverse a string
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

// 10. Check if a number is prime
fn is_prime_rust(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// 11. Merge two sorted arrays
fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }
    merged.extend_from_slice(&arr1[i..]);
    merged.extend_from_slice(&arr2[j..]);
    merged
}

// 12. Find the maximum subarray sum
fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_so_far = arr[0];
    let mut max_ending_here = arr[0];
    for &num in arr.iter().skip(1) {
        max_ending_here = cmp::max(num, max_ending_here + num);
        max_so_far = cmp::max(max_so_far, max_ending_here);
    }
    max_so_far
}

fn main() {
    println!("Is 'radar' a palindrome? {}", is_palindrome("radar"));
    let sorted_arr = vec![1, 2, 3, 4, 5];
    println!("Index of first occurrence of 3: {:?}", first_occurrence(&sorted_arr, 3));
    let sentence = "This is a test sentence";
    println!("Shortest word in sentence: {}", shortest_word(sentence));
    println!("Is 7 prime? {}", is_prime(7));
    println!("Median of [1, 2, 3, 4, 5]: {}", median(&sorted_arr));
    let strings = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    println!("Longest common prefix: {}", longest_common_prefix(&strings));
    println!("3rd smallest element: {:?}", kth_smallest(&sorted_arr, 3));
    let tree = Some(Box::new(TreeNode {
        val: 1,
        left: Some(Box::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })),
    }));
    println!("Maximum depth of tree: {}", TreeNode::max_depth(tree.as_ref()));
    println!("Reverse of 'hello': {}", reverse_string("hello"));
    println!("Is 13 prime? {}", is_prime_rust(13));
    let arr1 = vec![1, 3, 5];
    let arr2 = vec![2, 4, 6];
    println!("Merged arrays: {:?}", merge_sorted_arrays(&arr1, &arr2));
    let arr3 = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("Maximum subarray sum: {}", max_subarray_sum(&arr3));
}
