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

    car = car_factory(order, 1000);
    orders.insert(order, car);
    println!("car order {}: {:?}", order, orders.get(&order));

    order += 1;
    car = car_factory(order, 2000);
    orders.insert(order, car);
    println!("car order {}: {:?}", order, orders.get(&order));

    order += 1;
    car = car_factory(order, 0);
    orders.insert(order, car);
    println!("car order {}: {:?}", order, orders.get(&order));

    order += 1;
    car = car_factory(order, 0);
    orders.insert(order, car);
    println!("car order {}: {:?}", order, orders.get(&order));

    // let mut car = Car {
    //     color: "Blue".to_string(),
    //     motor: Transmission::Manual,
    //     roof: false,
    //     age: (Age::New, 0),
    // };
    let mut engine = Transmission::Manual;

    // car = car_factory("Ornage".to_string(), engine, true, 0);

    // engine = Transmission::SemiAuto;
    // car = car_factory("Red".to_string(), engine, false, 565);

    // engine = Transmission::Automatic;
    // car = car_factory("White".to_string(), engine, true, 3000);
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
    if color > 4 {
        color = color - 4;
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
