use std::io;

fn main() {
    let mut buff= String::new();
    std::io::stdin().read_line(&mut buff).unwrap();
    let parameter_1= buff.trim();
    
    // let result= solution(parameter_1, String::from("MFMFMF"));
    print!("{}", result);
}

fn solution(n: usize, list :String) -> String{
    let char_list: Vec<char>= list.chars().collect();

    if char_list.len() <= 1 {
        return String::from("Yes");
    }

    for i in 0..n-1{
        let pre= &char_list[i];
        let next= &char_list[i+1];
        if pre == next {
            return String::from("No");
        }
    }


    return String::from("Yes");
}
