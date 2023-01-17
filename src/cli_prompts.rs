
//mod vehicle;
use crate::vehicle::*;

//mod suspension_calculations;
use crate::suspension_calculations::*;

use std::io;


pub fn welcome_menu() {
    {
    println!("________________________________________________________________");
    println!("________________________________________________________________\n\n");
    println!("Welcome to the main menu for my suspension program!\n");
    println!("To begin, select from the options below:\n");
    println!("1 -> Create a default car and run calculations");
    println!("2 -> Create a custom vehicle profile and run calculations"); 
    println!("3 -> Learn about what all these values mean\n");
    println!("Type your selection, or type exit, and press Enter");
   }
}

pub fn print_instructions() {
    println!("________________________________________________________________");
    println!("________________________________________________________________\n\n");
    println!("Welcome back to the main menu for my suspension program!\nIf you generated information it is displayed above ^\n");
    println!("Select again from the options below:\n");
    println!("1 -> Create a default car and run calculations");
    println!("2 -> Create a custom vehicle profile and run calculations"); 
    println!("3 -> Learn about what all these values mean\n");
    println!("Type your selection, or type exit, and press Enter");
   }

pub fn run_program (input: String) {
    
    //menu #1 = generate default car
    
    if input == ("1").to_string() {
           run_input_1()
    }
    else if input == ("2").to_string() {
        run_input_2()
    }
    else if input == ("3").to_string() {
        run_input_3()
    }    


    //menu #2 = build custom car profile    
        else {println!("Houston we have a problem: {}.", input)}
}
fn run_input_1() {
    let car = default_car();
    calc_full_car(&car);
    print_930()
}
fn run_input_2 () {
    let mut question: i32 = 0;
    let mut user_car: Car = default_car();

    loop{          
    match question {
        0 =>  {
            println!("enter vechicle Name (Year Make Model:");
            let mut inner_input = String::new();

            io::stdin()
                .read_line(&mut inner_input)
                .expect("Failed question 0");
            
            if inner_input.trim() == "exit".to_string() {break}
            else{
            user_car.modify_car_name(&inner_input);
            question += 1;}

            continue
            } 
        1 =>  {
            println!("enter front motion ratio with decimal (i.e. 0.85 ):");
            let mut inner_input = String::new();

            io::stdin()
                .read_line(&mut inner_input)
                .expect("Failed question 2");

            if inner_input.trim() == "exit".to_string() {break}
            else{
                let inner_input_f32 = inner_input.trim().parse::<f32>().unwrap();
                user_car.motion_ratio_front = inner_input_f32 ;
                question += 1;
                }

            continue
        } 
        2 =>  {
            println!("enter rear motion ratio with decimal (i.e. 0.85 ):");
            let mut inner_input = String::new();

            io::stdin()
                .read_line(&mut inner_input)
                .expect("Failed question 3");

            if inner_input.trim() == "exit".to_string() {break}
            else{    
                let inner_input_f32 = inner_input.trim().parse::<f32>().unwrap();
                user_car.motion_ratio_rear = inner_input_f32 ;
                question += 1;
                }

            continue 
            }
        3 =>  {
            println!("enter front shock angle in degrees (i.e. 80.0 ):");
            let mut inner_input = String::new();

            io::stdin()
                .read_line(&mut inner_input)
                .expect("Failed question 4");
            
            if inner_input.trim() == "exit".to_string() {break}
            else{
                let inner_input_f32 = inner_input.trim().parse::<f32>().unwrap();
                user_car.front_shock_angle = inner_input_f32 ;
                question += 1;
                }
            continue 
            } 
        4 =>  {
            println!("enter rear shock angle in degrees (i.e. 80.0 ):");
            let mut inner_input = String::new();

            io::stdin()
                .read_line(&mut inner_input)
                .expect("Failed question 5");
            if inner_input.trim() == "exit".to_string() {break}
            else{
                let inner_input_f32 = inner_input.trim().parse::<f32>().unwrap();
                user_car.rear_shock_angle = inner_input_f32 ;
                question += 1;
                }
            continue
            } 
        5 =>  {
            println!("enter front spring rate in Kg (i.e. 8000 ):");
            let mut inner_input = String::new();

            io::stdin()
                .read_line(&mut inner_input)
                .expect("Failed question 6");
    
            if inner_input.trim() == "exit".to_string() {break}
            else{
                let inner_input_i32 = inner_input.trim().parse::<i32>().unwrap();
                user_car.modify_spring_rate_front(inner_input_i32);
                question += 1;
                }

            continue
            }
        6 =>  {
            println!("enter rear spring rate in Kg (i.e. 6000 ):");
            let mut inner_input = String::new();

            io::stdin()
                .read_line(&mut inner_input)
                .expect("Failed question 7");
            
            if inner_input.trim() == "exit".to_string() {break}
            
            else{
                let inner_input_i32 = inner_input.trim().parse::<i32>().unwrap();
                user_car.modify_spring_rate_rear(inner_input_i32);
                question += 1;
                }
            continue
            } 
        7 =>  {
            println!("enter right front (passenger side) corner weight in Kg (i.e. 325 ):");
            let mut inner_input = String::new();

            io::stdin()
                .read_line(&mut inner_input)
                .expect("Failed question 8");
    
            if inner_input.trim() == "exit".to_string() {break}
            else{
                let inner_input_i32 = inner_input.trim().parse::<i32>().unwrap();
                user_car.modify_rf_corner_weight(inner_input_i32) ;
                question += 1;
                }
            continue
            }
        8 =>  {
            println!("enter left front (driver side) corner weight in Kg (i.e. 325 ):");
            let mut inner_input = String::new();

            io::stdin()
                .read_line(&mut inner_input)
                .expect("Failed question 9");
            
            if inner_input.trim() == "exit".to_string() {break}
            else{
                let inner_input_i32 = inner_input.trim().parse::<i32>().unwrap();
                user_car.modify_lf_corner_weight(inner_input_i32) ;
                question += 1;
                }

            continue
            }
        9 =>  {
            println!("enter left rear (passenger side) corner weight in Kg (i.e. 325 ):");
            let mut inner_input = String::new();

            io::stdin()
                .read_line(&mut inner_input)
                .expect("Failed question 10");
    
            if inner_input.trim() == "exit".to_string() {break}
            else{
                let inner_input_i32 = inner_input.trim().parse::<i32>().unwrap();
                user_car.modify_lr_corner_weight(inner_input_i32) ;
                question += 1;
                }
            continue
            }
        10 => {
            println!("enter right rear (driver side) corner weight in Kg (i.e. 325 ):");
            let mut inner_input = String::new();

            io::stdin()
                .read_line(&mut inner_input)
                .expect("Failed question 8");
            if inner_input.trim() == "exit".to_string() {break}
            else{
                let inner_input_i32 = inner_input.trim().parse::<i32>().unwrap();
                user_car.modify_rr_corner_weight(inner_input_i32) ;
                question += 1;
                }
            continue
            }
        11 => {
            println!("enter front corner unsprung weight in Kg (i.e. 45 ):");
            let mut inner_input = String::new();

            io::stdin()
                .read_line(&mut inner_input)
                .expect("Failed question 12");
    
            if inner_input.trim() == "exit".to_string() {break}
            else{
                let inner_input_i32 = inner_input.trim().parse::<i32>().unwrap();
                user_car.front_unsprung_weight = inner_input_i32 ;
                question += 1;
                }
            continue
            }
        12 => {
            println!("enter rear corner unsprung weight in Kg (i.e. 45 ):");
            let mut inner_input = String::new();

            io::stdin()
                .read_line(&mut inner_input)
                .expect("Failed question 13");
    
            if inner_input.trim() == "exit".to_string() {break}
            else{
                let inner_input_i32 = inner_input.trim().parse::<i32>().unwrap();
                user_car.rear_unsprung_weight =inner_input_i32 ;
                question += 1;
            }
            continue
            }
        13 => {
            println!("enter low speed front critical damping value if desired (i.e. 0.7 )\n for default recommendations enter default:");
            let mut inner_input = String::new();

            io::stdin()
                .read_line(&mut inner_input)
                .expect("Failed question 14");
            
            if inner_input.trim() == "default".to_string() {
                question += 1;
                continue
            }
            else if inner_input.trim() == "exit".to_string() {break}
            else {    
                let inner_input_f32 = inner_input.trim().parse::<f32>().unwrap();
                user_car.modify_front_critical_damp_low(inner_input_f32);
                question += 1;
            }

            continue
            }
        14 => {
            println!("enter low speed rear critical damping value if desired (i.e. 0.7 )\n for default recommendations enter default:");
            let mut inner_input = String::new();

            io::stdin()
                .read_line(&mut inner_input)
                .expect("Failed question 15");
            
            if inner_input.trim() == "default".to_string() {
                question += 1;
                continue
            }
            else {    
                let inner_input_f32 = inner_input.trim().parse::<f32>().unwrap();
                user_car.modify_rear_critical_damp_low(inner_input_f32) ;
                question += 1;
            }

            continue
            }
        15 => {
            calc_full_car(&user_car);
            print_930();
            break
        }
        _ => {
            println!("Question Counter is broken in cli_promps index 2")
        }
    }
    } 
    
}
fn run_input_3 () {

    println!(r"
    
    This program is a simple suspension value calculator, working with
    suspension variables meant to return enough parameters to select
    spring rates and shock settings. These calculations can also give
    a baseline understanding of what handling characteristics to expect.

    The two most important values to consider from these outputs are the
    suspension's natural frequency (Hz) and the damper critical damping
    values in the low speed range.

    The natural frequency describes how many cycles per second the wheel
    would bounce without any damper acting on it. Generally a lower
    frequency would make a more comfortable road car and a higher
    value would indicate a race car that is able to recover after a
    bump. These are some general guidelines for selecting a natural
    frequency.

        1.0 Hz - passenger cars
        1.25 to 1.75 Hz - sports cars
        2.0 to 2.5 Hz - autocross and racecars with low downforce
        2.5+ Hz - high downforce racecars
        1.45 Hz - Subaru BRZ (front and rear)

    The natural frequncy is impacted by factors that impact the speed
    that the suspension arm can move through it's range of motion:

        Sprung Weight (wheels, hub, brakes, suspension arms)
        Corner Weight (the load on each corner over the Sprung Weight)
        Spring Rate
        Motion Ratio

    Critical Damping Rate decribes the behavior of the suspension
    bounce with a damper acting on it. A value of 1.0 (Critically Damped)
    describes a susupension that returns to level with no overshoot
    or 'bounce'. A Critical Damping Rate of 0.5 describes a suspension
    that compresses, then rebounds (bounces) beyond level with roughly half the
    amplitude of the compression. While a value of 1.0 sounds ideal, the
    goal is to return to level as quickly as possible, which happens closer
    to the value 0.7. This is because the damping force required to acheive
    a value of 1.0 slows the suspension more than the 0.7 force that allows
    a small over-run.

    In this program, we are only adjusting values for the Low Speed damping.
    The Low Speed zone impacts how responsive a car feels, and in many situations
    what the handling balance will acheive (Understeer, Neutral, Oversteer) based
    on the differences between front and rear values. These are excellent figures
    which partially consider the natural frequency and spring choices. The values
    presented are in lbs/inch per second. These are typical values shown in shock
    force value curves generated from testing dampers on a shock dynamometer. In
    and ideal setting, the dampers would be valved to specification to match the
    values presented in this program.

    The High Speed damping values deal with suspension compression forces greater
    than steering, accelerating, and braking create; this is why we are less
    interested in changing and fine tuning these values in this program. High
    Speed damping deals with large bumps and jolts (imagine hitting a tall curb
    at 100mph). In most low/mid priced adjustable shocks, the high speed values
    are not adjustable for safety. This is also where more expensive shocks show
    their value: digressive valving. Digressive valving describes a shock that
    gradually applies less resistance to increased force. It is the basis for the
    separation of Low and High speed that you see presented here, and has many
    advantages. If working correctly it provides good body control with higher
    low speed force per inch per second, and then allows suspension movement to
    occur to absorb harsh bumps.");
}
