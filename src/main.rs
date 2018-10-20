extern crate turtle;
use turtle::Turtle;

fn apply_rules(i: String) -> String{

    let mut r = "".to_string();

    if i == "1"{
        r = "11".to_string();
    }else if i == "0"{
        r = "1[0]0".to_string();
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
    turtle.set_speed(25);
//    turtle.wait_for_click();
    turtle.set_pen_color("white");
    turtle.drawing_mut().set_background_color("black");

    turtle.drawing_mut().set_center([0.0, -300.0]);



    let mut positions = Vec::new();
    let mut angles = Vec::new();

    for cmd in instructions.chars(){
        if cmd == '0'{

            turtle.forward(distance);
 


        }else if cmd == '1'{
            turtle.forward(distance);

        } else if cmd == '['{

            positions.push(turtle.position());
            angles.push(turtle.heading());

            turtle.left(angle);

        } else if cmd == ']' {

            let point = positions.pop().unwrap_or( [0.0, 0.0].into() );
            let heading = angles.pop().unwrap_or(45.0);

            turtle.pen_up();
            turtle.go_to(point);
            turtle.set_heading(heading);
            turtle.right(angle);
            turtle.pen_down();
        }

    }
}


fn main() {
    let result = create_l_system(4, "0".to_string());

    //let r = result.clone();

    //println!("{}", r);
    draw_l_system(result, 45.0, 20.0);
}
