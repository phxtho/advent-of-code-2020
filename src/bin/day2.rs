/*From https://adventofcode.com/2020/day/2 */
use regex::Regex;
use std::convert::TryFrom;

fn main() {
    let input = vec!["1-3 a: abcde","1-3 b: cdefg","2-9 c: ccccccccc"]; 
    let re = Regex::new(r"^([0-9]+)-([0-9]+)\s([A-z]):\s([A-z]+)$").unwrap();
    let mut count = 0; 
    for policy in input.iter(){
        let cap = re.captures(policy).unwrap();
        if validate_policy(*&cap[1].parse::<u32>().unwrap(), *&cap[2].parse::<u32>().unwrap(), &cap[3], &cap[3]) {
            count += 1;
        }
    }
    println! ("There are {} valid passwords", count);
}

fn validate_policy(min:u32,max:u32,character:&str,password:&str) -> bool {
    let count = u32::try_from(password.matches(character).count()).unwrap() ;
    if count >=  min && count <= max {
        return true
    }

    return false;
    
}