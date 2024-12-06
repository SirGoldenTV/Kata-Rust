fn main() {
    let s1 : String = String::from("abcdef");
    let s2 : String = String::from("abc");

    let res1 : Vec<String> = solution(&s1);
    print!("{:?}",res1);
    let res2 : Vec<String> = solution(&s2);
    print!("{:?}",res2);
}


fn solution(s: &str) -> Vec<String> {
    let mut res = Vec::new();
    let string = String::from(s.to_owned() + "_");
    for i in 0..string.len() {
        if i % 2 == 0 {
            continue;
        }
        res.push((&string[(i-1)..(i+1)]).to_string())
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn basic() {
        assert_eq!(solution("abcdef"), ["ab", "cd", "ef"]);
        assert_eq!(solution("abcdefg"), ["ab", "cd", "ef", "g_"]);
        assert_eq!(solution(""), [] as [&str; 0]);
    }
}