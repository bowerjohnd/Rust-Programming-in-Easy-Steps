fn total(nums: &[i32]) -> i32 {
    let mut total = 0;
    for num in nums {
        total += num;
    }
    total
}

fn main() {
    let values = [1,2,3,4,5,6,7,8,9,10];
    let others = [100,200];
    println!("The totals are {0} and {1}", total(&values), total(&others));
}
