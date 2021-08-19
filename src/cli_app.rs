#![allow(unused_must_use)]
use std::io;

fn calculate_weight_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

// fn grab_normal_weight_value_here(weight: &mut String){ }

pub fn run() {
    // let normal_person_weight = 100.0;
    println!("Please enter weight in kg: ");
    let mut normal_person_weight = String::new();

    // grab_normal_weight_value_here(&mut normal_person_weight);

    io::stdin().read_line(&mut normal_person_weight).unwrap();

    let weight: f32 = normal_person_weight.trim().parse().unwrap();

    let person_weight = calculate_weight_mars(weight);
    println!(
        "Weight of a normal person on EARTH is {}kg and on MARS is {} kg.",
        normal_person_weight, person_weight
    );
}
