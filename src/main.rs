use std::collections::HashMap;

#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}

fn main() {
    let mut order = 1;
    let mut car: Car;
    let mut orders: HashMap<i32, Car> = HashMap::new();
    let mut miles = 0;

    while order <= 11 {
        car = car_factory(order, miles);
        orders.insert(order, car);
        println!("Car order {}: {:?}", order, orders.get(&order));

        if miles == 2100 {
            miles = 0;
        } else {
            miles += 700;
        }
        order += 1;
    }

    let mut engine = Transmission::Manual;
}

fn car_quality(miles: u32) -> (Age, u32) {
    // let quality = (Age::New, miles);
    let quality = if miles > 0 {
        (Age::Used, miles)
    } else {
        (Age::New, miles)
    };
    quality
}

fn car_factory(order: i32, miles: u32) -> Car {
    let colors = ["Blue", "Green", "Red", "Silver"];

    let mut color = order as usize;
    while color > 4 {
        color -= 4;
    }

    let mut motor = Transmission::Manual;
    let mut roof = true;
    if order % 3 == 0 {
        motor = Transmission::Automatic;
    } else if order % 2 == 0 {
        motor = Transmission::SemiAuto;
        roof = false;
    }

    let age = car_quality(miles);
    Car {
        color: String::from(colors[(color - 1) as usize]),
        motor,
        roof,
        age,
    }
}
