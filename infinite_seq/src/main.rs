#[macro_use]

use std::io;

//1) Input: }](){ Output: (){} 2) Input: sh(dh)} Output: sh(dh) 3) Input: ]h({hdb}b)[ Output: Infinite

#[allow(unused_macros)]
macro_rules! read_str {
    ($out:ident) => {
        let mut inner = String::new();
        io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim();
    };
}

fn valid(xs: &mut String) -> bool {
    let mut st: String = String::new();
    for c in xs.chars() {
        match c {
            '(' => st.push('('),
            '[' => st.push('['),
            '{' => st.push('{'),
            ')' => match st.pop() { Some('(') => continue, _ => return false },
            ']' => match st.pop() { Some('[') => continue, _ => return false },
            '}' => match st.pop() { Some('{') => continue, _ => return false },
            _   => continue,
            
        }
    }
    if st.is_empty() {
        return true
    }
    else {
        return false
    }
}

fn sol(xs: &String) -> String {
    let mut res: String = String::new();
    let mut flag: usize = 0;
    for (i,c) in xs.chars().enumerate() {
        if flag > 0 { flag = flag - 1;  continue }
        match c {
            '(' => match xs.find(')') { Some(j) =>
                                        if j>i && valid(&mut xs[i..=j].to_string()) {
                                            res = res + &xs[i..=j];
                                            flag = j - i;
                                        }
                                        else if j<i && valid(&mut (xs[i..xs.len()].to_string() + &xs[0..=j])) {
                                            res = res + &xs[i..xs.len()] + &xs[0..=j];
                                        }
                                        else { continue },
                                        None    => continue },
            '[' => match xs.find(']') { Some(j) =>
                                        if j>i && valid(&mut xs[i..=j].to_string()) {
                                            res = res + &xs[i..=j];
                                            flag = j - i;
                                        }
                                        else if j<i && valid(&mut (xs[i..xs.len()].to_string() + &xs[0..=j])) {
                                            res = res + &xs[i..xs.len()] + &xs[0..=j];
                                        }
                                        else { continue },
                                        None    => continue },
            '{' => match xs.find('}') { Some(j) =>
                                        if j>i && valid(&mut xs[i..=j].to_string()) {
                                            res = res + &xs[i..=j];
                                            flag = j - i;
                                        }
                                        else if j<i && valid(&mut (xs[i..xs.len()].to_string() + &xs[0..=j])) {
                                            res = res + &xs[i..xs.len()] + &xs[0..=j];
                                        }
                                        else { continue },
                                        None    => continue },
            ')' => continue,
            ']' => continue,
            '}' => continue,
            _   => res.push(c),
        }
    }
    if res.len() >= xs.len() {
        String::from("Infinite")
    }
    else { return res }
}

            
fn main() {
    read_str!(xs);
    println!("{}", sol(&mut xs.to_string()))
}


//1) Input: }](){ Output: (){} 2) Input: sh(dh)} Output: sh(dh) 3) Input: ]h({hdb}b)[ Output: Infinite

#[test]
fn t1() {
    let xs = String::from("}](){");
    assert_eq!("(){}", sol(&xs))
}

#[test]
fn t2() {
    let xs = String::from("sh(dh)}");
    assert_eq!("sh(dh)", sol(&xs))
}

#[test]
fn t3() {
    let xs = String::from("]h({hdb}b)[");
    assert_eq!("Infinite", sol(&xs))
}

#[test]
fn t4() {
    let xs = String::from("()");
    assert_eq!("Infinite", sol(&xs))
}

#[test]
fn t5() {
    let xs = String::from("([](");
    assert_eq!("[]", sol(&xs))
}

#[test]
fn t6() {
    let xs = String::from(")(");
    assert_eq!("Infinite", sol(&xs))
}

#[test]
fn t7() {
    let xs = String::from("])[");
    assert_eq!("[]", sol(&xs))
}

#[test]
fn t8() {
    let xs = String::from("a");
    assert_eq!("Infinite", sol(&xs))
}

#[test]
fn t9() {
    let xs = String::from("[");
    assert_eq!("", sol(&xs))
}

#[test]
fn t10() {
    let xs = String::from("}");
    assert_eq!("", sol(&xs))
}
