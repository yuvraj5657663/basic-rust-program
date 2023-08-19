// fastival oriented cloths


// fastivals with selected dress
// const fastivals = {January : "utrayan, to-wear_casual_cloths", March : "holi, to-wear_white_cloths", August : "rakshabandhan, to-wear_kurta", Septmeber : "ganesh_chaturthi, to-wear_red_kurta", November : "navratri, to-wear_kediyu", December : "diwali, to-wear_designer_kurta"};
// for (const [key, value] of Object.entries(fastivals)) {
//     console.log(`${key}: ${value}`);
//   }

// fn main(){
//     let fastival_date = user_input;

//     if fastival true {
//         fastival_date = fastival_date-1;
//         println!(fastival_date, "wear a festival oriented cloths");
//     }
// }
  

function main(){
    let one_day_befor_fastival_date = 13-january;
    switch(one_day_befor_fastival_date){
        case 13-january:
            console.log("Tomorrow will be utrayan wear a casual cloths");
            break;
        case 10-march:
            console.log("Tomorrow will be holi wear a white cloths");
            break;
        case 15-august:
            console.log("Tomorrow will be rakshabandhan wear a kurta");
            break;
        case 10-september:
            console.log("Tomorrow will be ganesh_chaturthi wear a red kurta");
            break;
        case 10-november:
            console.log("Tomorrow will be navratri wear a kediyu");
            break;
        case 10-december:
            console.log("Tomorrow will be diwali wear a designer kurta");
            break;
        default:
            console.log("Tomorrow will be normal day wear a casual cloths");
            break;
    }
}