use std::cmp;

fn main() {
    let height = vec![0,1,0,2,1,0,1,3,2,1,2,1];
    println!("{}", trapping_rain_water(height));
}

fn trapping_rain_water(height: Vec<i32>) -> i32 {
    let len = height.len();
    let mut water = 0;
    let mut left = vec![0; len];
    let mut right = vec![0; len];

    left[0] = height[0];
    right[len - 1] = height[len - 1];

    for i in 1..len {
        left[i] = cmp::max(height[i], left[i - 1]);
    }

    for i in (0..len - 1).rev() {
        right[i] = cmp::max(height[i], right[i + 1]);
    }

    for i in 0..len {
        water += cmp::min(left[i], right[i]) - height[i];
    }

    water
}
