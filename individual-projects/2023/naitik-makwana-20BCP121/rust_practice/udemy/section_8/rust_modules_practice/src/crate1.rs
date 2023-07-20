mod person {
    pub struct Person_info {
        pub name:String,
        pub age:u32
        //without pub fields are private
    }

    impl Person_info {
        pub fn new(name:&str,x:u32) -> Self {
            Self{
                name:String::from(name),
                age:x
            }
        } 
    }
}

pub fn some_person() {
    let p1 = person::Person_info::new("Naitik",21);
    let p2 = person::Person_info{
        name: String::from("Rajan"),
        age: 21
    };
}