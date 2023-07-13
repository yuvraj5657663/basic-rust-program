fn main() {
    let mut s = String::from("Hello");
    {
        // let s1 = &s[0..4];
    }
    change( &mut s);

    let s2 = &s;

    println!("{}", &s2);
}

fn change(comp: &mut String) {
    comp.push_str(" world");
}