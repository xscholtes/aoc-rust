pub(crate) const INPUT:&'static str = include_str!("data/input.txt");

 
fn input_to_rows(input:&str) -> Vec<String>{
    return input.split("\r\n").map(|x| x.to_string()).collect();
}
fn main() {
    let result1:i32 = solution1(input_to_rows(INPUT));
    println!("RESULT 1: {}",result1);
    let result2:i32 = solution2(input_to_rows(INPUT));
    println!("RESULT 2: {}",result2);
}

fn solution1(rows:Vec<String>) -> i32 {
    let mut sum:i32 = 0;
    rows.into_iter().for_each(|r: String| {
        let n:Vec<char> = r.chars()
        .filter(|x: &char| x.is_numeric())
        .collect();
        if n.first().is_some() && n.last().is_some(){
         let num:i32 = format!("{}{}",n.first().unwrap(),n.last().unwrap()).parse().unwrap();
         sum += num;
        }
    });
    sum
}

fn solution2(rows:Vec<String>) -> i32 {
    let numbers = ["one","two","three","four","five","six","seven","eight","nine"];
    let rows2 = rows.into_iter().map(|original: String| {
        let mut left:String = original.clone();
        for (x,n) in numbers.into_iter().enumerate() {
            left = left.replace(n, format!("{}{}",n,x+1).as_str());
        }
        let mut right:String = original.clone();
        for (x,n) in numbers.into_iter().enumerate() {
            right = right.replace(n, format!("{}{}",x+1,n).as_str());
        }
        return format!("{left}{right}")
    }).collect();
    solution1(rows2)
}


#[cfg(test)]
mod tests {
    use super::solution1;
    use super::solution2;
    use super::input_to_rows;
    pub(crate) const SAMPLE1:&'static str = include_str!("data/sample1.txt");
    pub(crate) const SAMPLE2:&'static str = include_str!("data/sample2.txt");
    #[test]
    fn day1() {
        let sample1:i32 = solution1(input_to_rows(SAMPLE1));
        assert_eq!(sample1, 142);
        let sample2:i32 = solution2(input_to_rows(SAMPLE2));
        assert_eq!(sample2, 281);
    }
}