use itertools::Itertools;
use factorial::Factorial;
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let vvec: Vec<i128> = args.iter().map(|x| x.parse::<i128>().expect("Please provide integer volume traded.")).collect();
    println!("The size of the problem is {}.", vvec.len());
    for v in &vvec {
        let c = commission(*v);
        println!("Volume is {}, Commission is {}", v, c);
    }
    println!("Result is {:?}", shapley_value_mc(&vvec));
    println!("Result is {:?}", shapley_value(&vvec));
}

fn shapley_value_mc(vvec: &Vec<i128>) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();
    let total_vol = vvec.iter().sum();
    let c_t_vol = commission(total_vol);
    for v in vvec {
        result.push(c_t_vol - commission(total_vol - *v));
    }
    let norm_factor = c_t_vol / result.iter().sum::<f64>() ;
    for v in &mut result {
        *v = *v * norm_factor;
    }
    result
}


fn shapley_value(vvec: &Vec<i128>) -> Vec<f64> {
    let n = vvec.len();
    let perms = (0..n).permutations(n);
    let mut result = vec![0.0; n];
    for p in perms {
        let mut s: f64 = 0.0;
        let mut vs: i128 = 0;
        for i in p {
            vs += *vvec.get(i).unwrap();
            let tmpc = commission(vs);
            result[i] += tmpc - s;
            s = tmpc;
        }
    }
    for v in &mut result{
        *v = *v / n.factorial() as f64;
    }
    result
}

fn commission(volume: i128) -> f64 {
    assert!(volume > 0, "Please input a positive number");
    let volume = volume as f64;
    if volume <= 300001.0 {
        return 0.0035 * volume;
    }
    else if volume <= 3000000.0 {
        return 0.0020 * volume;
    }
    else if volume <= 20000000.0 {
        return 0.0015 * volume;
    }
    else if volume <= 100000000.0 {
        return 0.001 * volume;
    }
    else {
        return 0.0005 * volume;
    }
}

