const VARIABLES: &[(&str, &str)] = &[("x0", "x"), ("x1", "y"), ("x2", "z"), ("x3", "t"), ("x4", "w"), ("x5", "a")];

fn main() {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("Bit size: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    let bit_size: u8 = s.parse().expect("Expected number between 0 and 255");
    s.clear();
    print!("Number of truth values: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    let n_truth_values: u8 = s.parse().expect("Expected number between 0 and 255");
    s.clear();
    let mut truth_values = Vec::with_capacity(n_truth_values as usize);
    for i in 0..n_truth_values {
        loop {
            s.clear();
            print!("{}: ", i);
            let _ = stdout().flush();
            stdin()
                .read_line(&mut s)
                .expect("Did not enter a correct string");
            if let Some('\n') = s.chars().next_back() {
                s.pop();
            }
            if let Some('\r') = s.chars().next_back() {
                s.pop();
            }
            if truth_values.iter().any(|(s2, _, _)| s2 == &s) {
                println!("String already in list; Try again");
            } else if s.len() == bit_size as usize {
                break;
            } else {
                println!("Len: {}; Expected: {}; Try again", s.len(), bit_size);
            }
        }
        truth_values.push((s.clone(), s.chars().filter(|c| *c == '1').count(), 0u8));
    }
    truth_values.sort_by_key(|(_, c, _)| *c);
    println!("{:?}", truth_values);
    let mut truth_values_evolution = vec![truth_values];
    loop {
        let new_truth_values = optimize(truth_values_evolution.last_mut().unwrap());
        if new_truth_values.is_empty() {
            break;
        }
        truth_values_evolution.push(new_truth_values);
    }
    println!();
    print(&truth_values_evolution);
    println!();

    println!(
        "{0:<width$}{1}",
        "",
        truth_values_evolution[0]
            .iter()
            .fold(String::new(), |s, (s2, _, _)| format!("{}|{}", s, s2)),
        width = truth_values_evolution[0][0].0.len()
    );
    let mut grid: Vec<Vec<bool>> = Vec::new();
    let mut optimized: Vec<(&String, &usize, u8)> = truth_values_evolution
        .iter()
        .flatten()
        .filter(|(_, _, x)| *x == 0)
        .map(|(s, c, _)| (s, c, 0))
        .collect();
    for (i, (s, _, _)) in optimized.iter().enumerate() {
        println!(
            "{}{}",
            s,
            truth_values_evolution[0].iter().enumerate().fold(
                String::new(),
                |s1, (j, (s2, _, _))| format!(
                    "{}|{:^width$}",
                    s1,
                    if s.chars()
                        .zip(s2.chars())
                        .filter(|(c, _)| *c != '-')
                        .all(|(c1, c2)| c1 == c2)
                    {
                        get_or_default(&mut grid, j, |v| get_or_default(v, i, |b| *b = true));
                        'X'
                    } else {
                        get_or_default(&mut grid, j, |v| get_or_default(v, i, |b| *b = false));
                        ' '
                    },
                    width = truth_values_evolution[0][0].0.len()
                )
            )
        );
    }
    println!("{:?}", grid);
    loop {
        let mut taken = None;
        let min = grid
            .iter()
            .fold(usize::MAX, |n, s| s.iter().filter(|x| **x).count().min(n));
        if let Some(v) = grid
            .iter()
            .find(|s| s.iter().filter(|x| **x).count() == min)
        {
            let (t, _) = v.iter().enumerate().find(|(_, b)| **b).unwrap();
            optimized[t].2 = 1;
            taken = Some(t);
        }

        if let Some(taken) = taken {
            grid = grid.into_iter().filter(|v| !v[taken]).collect();
        } else if grid.is_empty() {
            break;
        }
    }
    let optimized = optimized
        .iter()
        .filter(|(_, _, a)| *a == 1)
        .map(|(s, _, _)| *s)
        .collect::<Vec<&String>>();
    println!("{:?}", optimized);
    let res = optimized
        .iter()
        .map(|s| {
            let s = s.chars()
                .enumerate()
                .filter(|(_, c)| *c != '-')
                .map(|(i, c)| format!("x{}{}", i, if c == '0' { "'" } else { "" }))
                .collect::<String>();
                VARIABLES.iter().fold(s, |s, (from, to)| s.replace(from, *to))
        })
        .collect::<Vec<String>>();
    
    println!("RES: {}", res.join(" + "));
}

fn get_or_default<T: Default, F: Fn(&mut T)>(vec: &mut Vec<T>, i: usize, f: F) {
    if let Some(v) = vec.get_mut(i) {
        f(v)
    } else {
        vec.push(T::default());
        f(vec.last_mut().unwrap())
    }
}

fn optimize(truth_values: &mut [(String, usize, u8)]) -> Vec<(String, usize, u8)> {
    let mut new_truth_values = Vec::new();
    for i in 0..truth_values.len() {
        for j in i..truth_values.len() {
            let diff = truth_values[j].1 - truth_values[i].1;
            if diff == 0 || diff > 1 {
                continue;
            }
            match different_characters(&truth_values[i].0, &truth_values[j].0) {
                DifferentCharacters::One(place) => {
                    let mut s = truth_values[i].0.clone();
                    s.replace_range(place..=place, "-");
                    if !new_truth_values.iter().any(|(s2, _, _)| s2 == &s) {
                        new_truth_values.push((s, truth_values[i].1.min(truth_values[j].1), 0u8));
                    }
                    truth_values[i].2 += 1;
                    truth_values[j].2 += 1;
                }
                DifferentCharacters::More => continue,
                x => unreachable!("Unexpect unhandled different character condition: {:?}", x),
            }
        }
    }
    new_truth_values
}

fn print(truth_values: &[Vec<(String, usize, u8)>]) {
    let columns: Vec<Vec<String>> = truth_values
        .iter()
        .map(|v| {
            v.iter()
                .fold(Vec::new(), |mut v, (s, c, u)| {
                    if let Some(c2) = v.last().map(|(_, c2, _)| *c2) {
                        if &c2 < c {
                            v.push((
                                format!("=={:=<width$}==\t", "", width = s.len()),
                                0usize,
                                0u8,
                            ));
                        }
                    }
                    v.push((
                        format!("{} {}  \t", if *u == 0 { ' ' } else { 'X' }, s),
                        *c,
                        *u,
                    ));
                    v
                })
                .into_iter()
                .map(|(s, _, _)| s)
                .collect()
        })
        .collect();
    let mut i = 0;
    loop {
        let b = columns
            .iter()
            .map(|c| c.get(i))
            .flatten()
            .fold(false, |_, s| {
                print!("{}", s);
                true
            });
        println!();
        i += 1;
        if !b {
            break;
        }
    }
}

#[derive(Debug)]
enum DifferentCharacters {
    None,
    One(usize),
    More,
    DiffLength,
}

fn different_characters(s1: &str, s2: &str) -> DifferentCharacters {
    if s1.len() != s2.len() {
        return DifferentCharacters::DiffLength;
    }
    s1.chars()
        .zip(s2.chars())
        .enumerate()
        .map(|(i, (c1, c2))| (i, c1, c2))
        .filter(|(_, c1, c2)| c1 != c2)
        .fold(DifferentCharacters::None, |d, (i, _, _)| match d {
            DifferentCharacters::None => DifferentCharacters::One(i),
            DifferentCharacters::One(_) => DifferentCharacters::More,
            x => x,
        })
}
