

extern crate turtle;

use turtle::Turtle;

fn apply_rules(i: String) -> String{

    let mut r = "".to_string();

    if i == "A"{
      r = "B".to_string();
    }else if i == "B"{
      r = "AB".to_string();
    }else {
      r = i.to_string();
    }

    return r;

}

fn process_string(old_string: String) -> String{
    let mut new_string = "".to_string();

    for c in old_string.chars() {
        let rule = apply_rules(c.to_string());
        new_string = [new_string, rule].join("");
    }

    return new_string;
}

fn create_l_system(n: i32, axiom: String) -> String{

    let mut start_string = String::from(axiom);
    let mut end_string = "".to_string();

    for _i in 0..n {
        end_string = process_string(start_string);
        start_string = end_string.clone();

    }

    return end_string;
}

fn main() {
    let result = create_l_system(4, "A".to_string());

    println!("{}", result);
//    let mut turtle = Turtle::new();

//    for _ in 0..4 {
//        turtle.forward(200.0);
//        turtle.right(90.0);
//    }
//
//


}
