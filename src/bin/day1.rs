/* From https://adventofcode.com/2020/day/1
Find two entries in a list of numbers which sum to 2020 and multiply them together 
*/
fn main() {
    let input: Vec<i32> = vec![1721,979,366,299,675,1456];
    let sum = 2020;
    let ans = factor_sum(&input,sum);
    println!("{} + {} = {}",ans.0,ans.1,sum)
}

fn factor_sum (possible_factors: &Vec<i32>, sum: i32) -> (i32,i32) {
    for x in possible_factors.iter() {
        for y in possible_factors.iter() {
            if x + y == sum {
                return (*x,*y)        
            }
        }
    }
    return (0,0);
}
