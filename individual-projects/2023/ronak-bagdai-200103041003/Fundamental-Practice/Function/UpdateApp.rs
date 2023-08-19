fn update_app(name: &str) {
    println!("Updating {}...",name)
}
fn main() {
    let apps = ["FaceBook", "Twitter", "Instagram", "Whatsapp"];

    for app in apps {
        update_app(app);
    }

    println!("All apps are up to date!");
}