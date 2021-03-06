extern crate turtle;
use turtle::Turtle;

fn apply_rules(i: String) -> String{

    let mut r = "".to_string();

    if i == "A"{
        r = "-BF+AFA+FB-".to_string();
    } else if i == "B"{
        r = "+AF-BFB-FA+".to_string();
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

fn draw_l_system(instructions: String, angle: f64, distance: f64){

    let mut turtle = Turtle::new();
    turtle.set_speed(24);
    turtle.wait_for_click();
    turtle.set_pen_color("white");
    turtle.drawing_mut().set_background_color("black");

    turtle.drawing_mut().set_center([200.0, -300.0]);

    for cmd in instructions.chars(){
        if cmd == 'A'{
            //turtle.forward(distance);
        }else if cmd == 'B'{
            //turtle.forward(distance);
        } else if cmd == 'F'{
            turtle.forward(distance);
            //turtle.left(angle);
        } else if cmd == '-' {
            turtle.left(angle);
        } else if cmd == '+'{
            turtle.right(angle);
        }
    }
}


fn main() {
    let result = create_l_system(6, "B".to_string());
    //let r = result.clone();
    //println!("{}", r);
    draw_l_system(result, 90.0, 10.0);
}
