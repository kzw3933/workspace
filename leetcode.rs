impl Solution {
  pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
      let mut nums = nums;
      let length = nums.len();
      nums.sort();
      let mut result = vec![];
      let mut i = 0;
      while i < length {
          let mut j = i+1;

          while j < length {
              let (mut l, mut r) = (j+1, length-1);
              while l < r && l < length-1 {
                  let sum4: i64 = nums[i] as i64 + nums[j] as i64 + nums[l] as i64 + nums[r] as i64;
                  if sum4 < target as i64 {
                      l += 1;
                  } else if sum4 > target as i64 {
                      r -= 1;
                  } else {
                      result.push(vec![nums[i], nums[j], nums[l], nums[r]]);
                      let old_num = nums[l];
                      while l < length-1 && nums[l+1] == old_num {
                          l += 1;
                      }
                      l += 1;
                  }
              }

              let old_num = nums[j];
              while j < length-1 && nums[j+1] == old_num {
                  j += 1;
              }
              j += 1;
          }

          let old_num = nums[i];
          while i < length-1 && nums[i+1] == old_num {
              i += 1;
          }
          i += 1;
      }

      result
  }
}