fn main() {
    println!("Hello, world!");

    can_vote(16);

    list_items_with_index(&[1,2,3,4,5]);
}

fn can_vote(age: i32) {
    if age >= 18 {
        println!("Can vote");
    } else if age == 17 {
        println!("They are close to voting age");
    }else {
        println!("Cannot vote");
    }
}

#[allow(dead_code)]
fn count_to(to: i32) {
    let mut counter = 0;

    loop {
        counter += 1;

        println!("{}", counter);

        if counter == to {
            break
        }
    }
}

#[allow(dead_code)]
fn while_count_to(to: i32) {
    let mut counter = 0;

    while counter != to {
        counter += 1;

        println!("{}", counter)
    }
}

#[allow(dead_code)]
fn range_count_to(to: i32) {
    for number in 1..=to {
        println!("{}", number);
    }
}

#[allow(dead_code)]
fn list_items(array: &[i32]) {
    for element in array.iter() {
        println!("{}", element);
    }
}


fn list_items_with_index(array: &[i32]) {
    for (index, element) in array.iter().enumerate() {
        println!("Index {} has value {}", index, element);
    }
}