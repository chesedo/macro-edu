use macro_copy::copy;

struct Parent {
    name: String,
    age: u32,
    married: bool,
}

#[derive(Debug)]
struct Human {
    first_name: String,
    years_old: u32,
}

fn main() {
    let john = Parent {
        name: "John".to_string(),
        age: 34,
        married: true,
    };

    copy!(
        john -> human: Parent -> Human {
            name -> first_name,
            age -> years_old,
        }
    );

    // let human = Human {
    //     first_name: john.name,
    //     years_old: john.age,
    // };

    println!("{human:?}");
}
