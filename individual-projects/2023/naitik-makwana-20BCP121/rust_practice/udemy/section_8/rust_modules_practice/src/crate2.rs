mod car_type {
    pub enum Car {
        Suv,
        Sedan,
        Hatchback
    }
}

pub fn some_car() {
    let car1 = car_type::Car::Suv;
    let car2 = car_type::Car::Sedan;
    let car3 = car_type::Car::Hatchback;
}