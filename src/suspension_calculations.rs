

use std::f32::consts::PI;
use crate::vehicle::*;



pub fn calc_motion_ratio(pivot_to_shock:f32, pivot_to_hub:f32) -> f32 { 
    let motion_ratio = pivot_to_shock /pivot_to_hub ;
    motion_ratio as f32
}

pub fn calc_full_car(car: &Car) {
    let curb = car.calc_curb_weight() as f32;
    //paragraph information
    println!("-----------------------------------------------------------\n");
    println!("Target Damper Testing Values for {}:\n", car.car_name);
    match car.front_critical_damp_low {
        None => {
    
            println!("Standard Front Low Speed Body Control: {} to {} lbs/inch per sec", car.calc_critical_damping_coil_front() * 0.6, car.calc_critical_damping_coil_front() * 0.7);
            println!("Front High Speed Bump Control: {} to {} lbs/inch per sec\n", car.calc_critical_damping_coil_front() * 0.3, car.calc_critical_damping_coil_front() * 0.4);
        }
        Some(value) => {
            println!("Defined Front Low Speed Body Control {} CDR: {} lbs/inch per sec", value, car.calc_critical_damping_coil_front() * value);
            println!("Front High Speed Bump Control: {} to {} lbs/inch per sec\n", car.calc_critical_damping_coil_front() * 0.3, car.calc_critical_damping_coil_front() * 0.4);
                }
        }

    match car.rear_critical_damp_low {
        None => {
            println!("Standard Rear Low Speed Body Control: {:.2} to {:.2} lbs/inch per sec", car.calc_critical_damping_coil_rear() * 0.6, car.calc_critical_damping_coil_rear() * 0.7);
            println!("Rear High Speed Bump Control: {:.2} to {:.2} lbs/inch per sec\n", car.calc_critical_damping_coil_rear() * 0.3, car.calc_critical_damping_coil_rear() * 0.4);
        }
        Some(value) => {
            println!("Defined Rear Low Speed Body Control {:.2} CDR: {:.2} lbs/inch per sec", value, car.calc_critical_damping_coil_rear() * value);
            println!("Rear High Speed Bump Control: {:.2} to {:.2} lbs/inch per sec\n", car.calc_critical_damping_coil_rear() * 0.3, car.calc_critical_damping_coil_rear() * 0.4);
        }
    }

    println!("Current Frequencies: Front {:.2}Hz, Rear {:.2}Hz", car.calc_nat_freq_front(),  car.calc_nat_freq_rear());
    println!("Ride Flat Theory Rear Frequency: {:.2}Hz based on current front", car.calc_nat_freq_front() * 1.15  );
    

    //chassis diagram
    println!("\n-----------------------------------------------------------\n");
    print!(" LF: {:.2}%, {:.2} Kg         ", ((car.lf_corner_weight as f32 / curb) * 100.0), car.lf_corner_weight );
    println!(" RF: {:.2}%, {:.2} Kg", ((car.rf_corner_weight as f32 / curb) * 100.0), car.rf_corner_weight);
    println!("\n\n");
    
    println!("              Front: {:.2}%, {:.2} Kg", (car.lf_corner_weight as f32 + car.rf_corner_weight as f32) / curb * 100.0, car.lf_corner_weight + car.rf_corner_weight);
    println!("       Total Weight {:.2} Kg, or {:.2} lbs\n", curb, curb * 2.2);

    println!("");
    print!(r"      LF|RR Cross \ : {:.2}%      ", (car.lf_corner_weight as f32 + car.rr_corner_weight as f32)/curb *100.0);
    println!("RF|LR Cross / : {:.2}%", (car.rf_corner_weight as f32 + car.lr_corner_weight as f32)/curb * 100.0);

    println!("\n               Rear: {:.2}%, {}Kg", (car.rr_corner_weight as f32 + car.lr_corner_weight as f32)/curb * 100.0,  car.rr_corner_weight + car.lr_corner_weight);
    println!("\n\n");
    print!(" LR: {:.2}%, {} Kg         ", ((car.lr_corner_weight as f32 / curb) * 100.0), car.lr_corner_weight );
    println!(" RR: {:.2}%, {} Kg", ((car.rr_corner_weight as f32 / curb) * 100.0), car.rr_corner_weight);

    //car art
    //print_930();
}



impl Car {
    pub fn calc_curb_weight(&self) -> i32 {
        let total = 
        self.rf_corner_weight 
        + self.lf_corner_weight 
        + self.rr_corner_weight
        + self.lr_corner_weight;
        total
    }

    pub fn calc_front_sprung_weight(&self) -> i32 {
        let rf_sprung_weight = self.rf_corner_weight - self.front_unsprung_weight;
        let lf_sprung_weight = self.lf_corner_weight - self.front_unsprung_weight;
        let front_sprung_weight: f32 = ((rf_sprung_weight + lf_sprung_weight)/2) as f32;
        front_sprung_weight as i32
    }

    pub fn calc_rear_sprung_weight(&self) -> i32 {
        let rr_sprung_weight = self.rr_corner_weight - self.rear_unsprung_weight;
        let lr_sprung_weight = self.lr_corner_weight - self.rear_unsprung_weight;
        let rear_sprung_weight: f32 = ((rr_sprung_weight + lr_sprung_weight)/2) as f32;
        rear_sprung_weight as i32
    }

    pub fn calc_nat_freq_front(&self) -> f32 {
        //Common suspension frequencies:

        // 1.0 Hz - passenger cars
        // 1.25 to 1.75 Hz - sports cars
        // 2.0 to 2.5 Hz - autocross and racecars with low downforce 
        // 2.5+ Hz - high downforce racecars
        // 1.45 Hz - Subaru BRZ (front and rear)

        //from Kg/m spring rate to Nm
        let front_spring_newton_meters =  self.spring_rate_front as f32 * 9.80665;
        //println!("front_spring_newton_meters: {}", front_spring_newton_meters);

        let front_motion_ratio_sq = self.motion_ratio_front * self.motion_ratio_front;
        let wheel_rate_front = front_spring_newton_meters as f32 * front_motion_ratio_sq;
        
        
        let half_pi = 1.0 / (2.0 * PI);
        let keff_div_by_sprung_front =  wheel_rate_front / self.calc_front_sprung_weight() as f32;
        let shock_angle_correction = self.front_shock_angle.to_radians().sin();
        //println!("front sin value {}", shock_angle_correction);
        let nat_freq_front = half_pi * (keff_div_by_sprung_front.sqrt()) * shock_angle_correction;
        

        nat_freq_front

        //to add coil angle correction multiply result by sin(angle of shock), should be 75* to 90*
        //10 degrees from verticle is sin(80), 1.5% change to freq
    }

    pub fn calc_nat_freq_rear(&self) -> f32 {
         //Common suspension frequencies:

        // 1.0 Hz - passenger cars
        // 1.25 to 1.75 Hz - sports cars
        // 2.0 to 2.5 Hz - autocross and racecars with low downforce 
        // 2.5+ Hz - high downforce racecars
        // 1.45 Hz - Subaru BRZ (front and rear)

        //from Kg/m spring rate to Nm
        let rear_spring_newton_meters = self.spring_rate_rear as f32 * 9.80665;
        //println!("rear_spring_newton_meters: {}", rear_spring_newton_meters);

        let rear_motion_ratio_sq = self.motion_ratio_rear * self.motion_ratio_rear;
        let wheel_rate_rear = rear_spring_newton_meters as f32 * rear_motion_ratio_sq;

        let half_pi = 1.0 / (2.0 * PI);
        let keff_div_by_sprung_rear =  wheel_rate_rear / self.calc_rear_sprung_weight() as f32  ;
        let shock_angle_correction = self.rear_shock_angle.to_radians().sin();
        //println!("rear sin value {}", shock_angle_correction);
        let nat_freq_rear = half_pi * (keff_div_by_sprung_rear.sqrt()) * shock_angle_correction;
        
        nat_freq_rear
        //to add coil angle correction multiply result by sin(angle of shock), should be 75* to 90*
        //10 degrees from verticle is sin(80), 1.5% change to freq
    }    

    pub fn modify_car_name(&mut self, new_name:&str) {
        self.car_name = new_name.to_string()
    }

    pub fn modify_spring_rate_front(&mut self, new_rate:i32) {
        self.spring_rate_front = new_rate
    }

    pub fn modify_spring_rate_rear(&mut self, new_rate:i32) {
        self.spring_rate_rear = new_rate
    }

    pub fn modify_rf_corner_weight(&mut self, new_weight:i32) {
        self.rf_corner_weight = new_weight
    }

    pub fn modify_lf_corner_weight(&mut self, new_weight:i32) {
        self.lf_corner_weight = new_weight
    }

    pub fn modify_rr_corner_weight(&mut self, new_weight:i32) {
        self.rr_corner_weight = new_weight
    }

    pub fn modify_lr_corner_weight(&mut self, new_weight:i32) {
        self.lr_corner_weight = new_weight
    }

    pub fn modify_front_critical_damp_low(&mut self, new_value:f32) {
        self.front_critical_damp_low = Some(new_value)
    }

    pub fn modify_rear_critical_damp_low(&mut self, new_value:f32) {
        self.rear_critical_damp_low = Some(new_value)
    }

    pub fn calc_critical_damping_wheel_front(&self) -> f32 {
        //returns in newtons/m per sec
        //2*sqrt(mass*Kw)
        
        let front_spring_newton_meters =  self.spring_rate_front as f32 * 9.80665;
        //println!("front_spring_newton_meters: {}", front_spring_newton_meters);

        let front_motion_ratio_sq = self.motion_ratio_front * self.motion_ratio_front;
        let wheel_rate_front = front_spring_newton_meters as f32 * front_motion_ratio_sq;

        let result = 2.0 * ((wheel_rate_front * self.calc_front_sprung_weight() as f32).sqrt());
        //println!("Front critical damping wheel rate in Nm: {}", result);
        result as f32
    }

    pub fn calc_critical_damping_wheel_rear(&self) -> f32 {
        //2*sqrt(mass*Kw)
        let rear_spring_newton_meters = self.spring_rate_rear as f32 * 9.80665;
        //println!("rear_spring_newton_meters: {}", rear_spring_newton_meters);

        let rear_motion_ratio_sq = self.motion_ratio_rear * self.motion_ratio_rear;
        let wheel_rate_rear = rear_spring_newton_meters as f32 * rear_motion_ratio_sq;

        let result_nm = 2.0 * ((wheel_rate_rear * self.calc_rear_sprung_weight() as f32).sqrt());
        //println!("Rear critical damping wheel rate in Nm: {}", result_nm);
        result_nm as f32
        //result as f32 
        //convert to lbs per inch per second
        
        

    }

    pub fn calc_critical_damping_coil_front(&self) -> f32 {
        let result_nm = self.calc_critical_damping_wheel_front() / (self.motion_ratio_front * self.motion_ratio_front);
       let result_lbs = result_nm * (2.2/9.806) * (1.0/39.37);
       //println!("Front critical damping rate for the coil in lbs / inch per second: {}", result_lbs);
       result_lbs as f32
        
    }

    pub fn calc_critical_damping_coil_rear(&self) -> f32 {
       let result_nm = self.calc_critical_damping_wheel_rear() / (self.motion_ratio_rear * self.motion_ratio_rear);
       let result_lbs = result_nm * (2.2/9.806) * (1.0/39.37);
       //println!("Rear critical damping rate for the coil in lbs / inch per second: {}", result_lbs);
       result_lbs as f32
        //returns in newtons/m per sec
    }
    
}
 


