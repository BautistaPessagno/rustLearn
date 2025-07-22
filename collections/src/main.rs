use std::{array, collections::HashMap, i32};

fn main() {
    //Given a list of integers, use a vector and return the median
    //(when sorted, the value in the middle position)
    //and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    let v = vec![2, 3, 3, 3, 4, 6, 10, 12, 13, 13, 16];
    //en este caso la mediana seria 6 y la moda 3
    let v2 = vec![2, 3, 3, 4, 4, 6, 10, 12, 13, 13, 13, 17];
    let median1 = get_median(&v);
    let median2 = get_median(&v2);

    println!("vec1 {v:?}");
    println!("vec2 {v2:?}");

    println!("la mediana es {}", median1);
    println!("la segunda mediana es {}", median2);

    let mode1 = get_mode(&v);
    let mode2 = get_mode(&v2);

    println!("la primera moda es: {}", mode1);
    println!("la segunda moda es: {}", mode2);

    println!("--------------------EJ2--------------------------------");
    // pig-latin ----------------------------------------
    let first: String = String::from("first");
    let apple = String::from("apple");
    let bautista = String::from("Bautista");
    let airplane = String::from("airplane");

    let first2 = pig_latin(&first);
    let apple2 = pig_latin(&apple);
    let bautista2 = pig_latin(&bautista);
    let airplane2 = pig_latin(&airplane);

    println!("{} -> {}", first, first2);
    println!("{} -> {}", apple, apple2);
    println!("{} -> {}", airplane, airplane2);
    println!("{} -> {}", bautista, bautista2);

    println!("------------------EJ3---------------------------");
    let mut employees = HashMap::new();

    let deparments = ["Engineering", "Sales"];
    let add_sally = String::from("Add sally to Engineering");
    let add_amir = String::from("Add Amir to Sales");

    add_employee(&deparments, &add_sally, &mut employees);
    add_employee(&deparments, &add_amir, &mut employees);
    add_employee(&deparments, "add Bautista to Engineeringg", &mut employees);

    for (k, v) in employees {
        println!("{} : ", k);
        for i in v {
            print!("{i} ");
        }
        println!("");
    }
}

fn add_employee(arr: &[&str], s: &str, map: &mut HashMap<String, Vec<String>>) {
    let mut it = s.split_whitespace();
    let add = it.next();
    let employee = match it.next() {
        Some(e) => String::from(e),
        None => {
            eprintln!("texto incompleto");
            return;
        }
    };
    for &i in arr {
        if s.contains(i) {
            let employe_dep = map.entry(i.clone().to_string()).or_insert(Vec::new());
            employe_dep.push(employee.clone());
        }
    }
}

fn get_median(vec: &Vec<i32>) -> i32 {
    let length = vec.len();

    if (length % 2) == 1 {
        let m = vec[length / 2];
        m
    } else {
        let m = (vec[length / 2] + vec[(length / 2) - 1]) / 2;
        m
    }
}

fn get_mode(v: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let mut mode = v[0];
    let mut mode_frec = 0;
    for &i in v {
        let count = map.entry(i).or_insert(0);
        *count += 1;
        if *count > mode_frec {
            mode_frec = *count;
            mode = i;
        }
    }
    mode
}

fn pig_latin(s: &str) -> String {
    let first = &s[..1];

    let vowels = ["a", "e", "i", "o", "u"];

    let mut ans: String = String::new();
    if vowels.contains(&first) {
        ans = String::from(s) + "-hay";
    } else {
        ans = String::from(&s[1..]) + "-" + first + "ay";
    }
    ans
}
