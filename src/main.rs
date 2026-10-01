fn main() {
    let name = "Dev Kant Kumar";
    let email = "hello@devkantkumar.com";
    let mut age = 21;
    let is_active = true;

    let mut marks = vec![98, 98, 97, 96, 99];

    println!("Name : {name}");
    println!("Email :  {email}");
    println!("Age : {age}");
    println!("Active : {is_active}");
    println!("Marks : {:?}", marks);

    age = 22;
    println!("Age : {age}");

    marks.push(90);

    println!("\nUpdated marks : {:?}", marks);

    let mut total = 0;

    for m in marks {
        total += m;
    }

    println!("Total Marks : {}", total);
}
