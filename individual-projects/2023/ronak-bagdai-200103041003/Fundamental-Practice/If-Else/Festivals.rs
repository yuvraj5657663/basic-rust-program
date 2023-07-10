fn main() {
    let festivals = ["Holi", "Diwali", "Christmas"];
    let mut festival_count = 0;

    for festival in festivals.iter() {
        if festival == &"Holi" {
            println!("Wear white clothes on day before Holi");
            festival_count += 1;
        } else if festival == &"Diwali" {
            println!("Wear traditional Indian clothes on day before Diwali");
            festival_count += 1;
        } else if festival == &"Christmas" {
            println!("Wear red and green clothes on day before Christmas");
            festival_count += 1;
        }
    }

    println!("Number of festivals in the year: {}", festival_count);
}
