fn main(){
    let mut num_of_cameras_not_working =0;

    let camera1 = true;
    let camera2 = false;
    let camera3 = false;

    if !camera1 {
        num_of_cameras_not_working += 1;
    }
    if !camera2 {
        num_of_cameras_not_working += 1;
    }
    if !camera3 {
        num_of_cameras_not_working += 1;
    }
    
    println!("not working camera is : {}" , num_of_cameras_not_working);
    if num_of_cameras_not_working == 1 {
        println!("warnig");
    }
    else if num_of_cameras_not_working > 1 {
        println!("error");
    }
}