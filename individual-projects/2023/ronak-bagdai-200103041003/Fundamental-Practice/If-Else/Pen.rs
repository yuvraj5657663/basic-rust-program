fn remove_pen(){
    remove faulty_pens;
}

fn main() {
    let n = 10;
    let working_pens = 0;
    let faulty_pens = 0;

    for i in 1..=n {
        if i is working {
            working_pens +=1;
        }
        else {
            faulty_pens +=1;
        }
    }
    remove_pen();
    print!("Number of Working Pens are {}", working_pens);
}