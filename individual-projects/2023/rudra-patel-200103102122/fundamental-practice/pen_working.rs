// pen is working or not

fn pen_count_working(){
    pen_working = pen_working + 1;
}
fn pen_count_not_working(){
    pen_working = pen_working + 1;
}

fn main(){
    let pen_working = yes;
    for pen in 0..50 {
    if pen_working == yes {
        println!("the pen is working");
        return pen_count_working();
        pen ++;
    }
    else if pen_working == no{
        println!("the pen is not working");
        continue;
        return pen_count_not_working();
        pen ++;
    }
    else{

    }
}
}