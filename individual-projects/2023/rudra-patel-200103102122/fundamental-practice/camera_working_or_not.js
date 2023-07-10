// parking camera working or not


function main(){
    let c = "chek_the_camera";

    if (c > 1) {
        console.log("error");
    }
    else if (c < 2) {
        console.log("the camera is not working");
    }
    else{
        console.log("the camera is working");
    }
}



// function camera(){
//     let num_of_cameras_not_working =0;

//     let camera1 = true;
//     let camera2 = false;
//     let camera3 = false;

//     if (camera1) {
//         num_of_cameras_not_working += 1;
//     }
//     if (camera2) {
//         num_of_cameras_not_working += 1;
//     }
//     if (camera3) {
//         num_of_cameras_not_working += 1;
//     }
    
//     console.log("not working camera is : {}" , num_of_cameras_not_working);
//     if (num_of_cameras_not_working == 1) {
//         console.log("warnig");
//     }
//     else if (num_of_cameras_not_working > 1) {
//         console.log("error");
//     }
// }

// function main(){
//     let basement = 3;

//     for (let i = 1; i <= 3; i++) {
//             camera();
//             console.log("this basement is completed");
//     }
// }
