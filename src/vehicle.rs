
use crate::*;

pub struct Car {
    pub car_name: String,           //Year Make Model
    pub motion_ratio_front: f32,    //calc_motion_ratio available
    pub motion_ratio_rear: f32,     //calc_motion_ratio available
    pub front_shock_angle: f32,     //90.0 is verticle, use degrees, typical 75-90
    pub rear_shock_angle: f32,      //90.0 is verticle, use degrees
    pub spring_rate_front: i32,     //use Kg, ex 12000
    pub spring_rate_rear: i32,      //use Kg, ex 6000
    pub rf_corner_weight: i32,      //Kg
    pub lf_corner_weight: i32,      //Kg
    pub rr_corner_weight: i32,      //Kg
    pub lr_corner_weight: i32,      //Kg
    pub front_unsprung_weight: i32, //Kg
    pub rear_unsprung_weight: i32,  //Kg
    pub front_critical_damp_low: Option<f32>,   //0-3 inch per second range
    pub rear_critical_damp_low: Option<f32>,    //0-3 inch per second range
}

pub fn default_car() -> Car {
    let car =  Car {
        car_name: "2022 Default Car".to_string(),
        motion_ratio_front: calc_motion_ratio(8.2, 13.5),
        motion_ratio_rear: calc_motion_ratio(10.0, 13.5),
        front_shock_angle: 80.0,  
        rear_shock_angle: 75.0,    
        spring_rate_front:12000 , 
        spring_rate_rear: 5000,  
        rf_corner_weight: 319, 
        lf_corner_weight: 319,
        rr_corner_weight: 295,
        lr_corner_weight: 295,
        front_unsprung_weight: 45, 
        rear_unsprung_weight: 45,
        front_critical_damp_low: None,
        rear_critical_damp_low: None, 
        };
        car
}


pub fn print_930() {
    println!("");
    println!("");
   println!(r#" ____----------- _____                                     "#);
   println!(r#" \~~~~~~~~~~/~_--~~~------~~~~~     \                      "#);
   println!(r#"  `---`\  _-~      |                   \                   "#);
   println!(r#"    _-~  <_         |                    \[]               "#);
   println!(r#"  / ___     ~~--[""] |      ________-------'_              "#);
   println!(r#" > /~` \    |-.   `\~~.~~~~~                _ ~ - _        "#);
   println!(r#"  ~|  ||\%  |       |    ~  ._                ~ _   ~ ,   "#);
   println!(r#"    `_//|_%  \      |          ~  ,             ~-_   /\   "#);
   println!(r#"           `--__     |    _-____  /\               ~-_\/. "#);
   println!(r#"               ~--_ /  ,/ -~-_ \  \/         _______---~/   "#);
   println!(r#"                   ~~-/._<   \ \`~~~~~~~~~~~       ##--~/  "#);
   println!(r#"                          \    ) |`((---~~~---~~~~-~  ) )  "#);
   println!(r#"                           ~-_/_/                  ~~ ~~   "#);
   println!("");

}