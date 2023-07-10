//pattern 1 :

let x = 4;
for (let i = 1; i <= x; i++) {
    for (j = 1; j <= i; j++) {
        document.write("*")
    }
    document.write("</br>");
}

// pattern 2:

document.write("</br>");
 
let a,b,c;
 
for(a=1;a<=6;a++){

    for(b=1;b<=6-a;b++){
        document.write(" ","&nbsp");
    }
    for(c=1;c<=a-1;c++){
        document.write("* ");
    }
    document.write("</br>");
}


   