// ====================== loop ======================
// attendance

function attendace(){
    let count = 50;
    let student_call1 = "yes";
    // let student_call2 = no;

    if (count == 1) {
        if (student_call1 == "yes") {
            console.log("the student is present");
        }
        else {
            console.log("the student is absent");
        }
    count ++;
    }
    else if (count >= 51) {
       console.log("the attendance is completed");
    }
    else{

   }
}

function main(){

    for(let i = 1; i <= 25; i++) {
    //    let class = 25;
     
    if (i >= 1) {
        attendace();
        console.log("this class attendance is completed");
        // class ++;
   }
   else if (i >= 26) {
         console.log("all the class attendance is completed");
         break;
        }
        else{

        }
   }
}
main();