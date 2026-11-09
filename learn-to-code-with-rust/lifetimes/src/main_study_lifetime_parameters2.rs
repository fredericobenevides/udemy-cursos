struct DentistAppointment {
    doctor: String,
}

impl DentistAppointment {
    fn book(&self, check_in_time: &str, check_out_time: &str) -> &str {
        println!(
            "You are booked from {} to {} with doctor {}",
            check_in_time, check_out_time, self.doctor
        );
        &self.doctor
    }

    fn book2<'a>(&self, check_in_time: &'a str, check_out_time: &str) -> &'a str {
        println!(
            "You are booked from {} to {} with doctor {}",
            check_in_time, check_out_time, self.doctor
        );
        check_in_time
    }
}

fn main() {
    let appt = DentistAppointment {
        doctor: String::from("David"),
    };
    let result = appt.book("03:00PM", "04:00PM");
    println!("{}", result);

    let result = appt.book2("03:00PM", "04:00PM");
    drop(appt);
    println!("{}", result);
}
