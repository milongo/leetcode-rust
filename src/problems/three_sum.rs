pub fn three_sum(nums: Vec<i32>, target: i32) -> bool {
    let mut nums = nums;
    nums.sort();

    let n = nums.len();

    for i in 0..n - 2 {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = n - 1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];

            if sum == target {
                return true;
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    return false
}
