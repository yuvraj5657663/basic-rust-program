function calc(){
    let fd = document.getElementById('fd').value;
    fd = Number(fd);
    let ld = document.getElementById('ld').value;
    ld = Number(ld);
    let oop = document.getElementById('operator').value;
    switch (oop) {
        case "+":{
            document.write(fd + ld);
            break;
            }
        case "-":{
            document.write(fd - ld);
            break;
            }
        case "*":{
            document.write(fd * ld);
            break;
            }
        case "/":{
            document.write(fd / ld);
            break;
            }
        case "%":{
            document.write(fd % ld);
            break;
            }
        default:
            document.write('The entered operator is not an Arithnatic operator');
            break;
    }
};