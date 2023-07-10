
// Write the code which asks for a login with prompt.

let name = 'Jay Parejiya';
let password = '5070';

let enterName = prompt("Enter the username");

if (enterName == '') {
    alert('cancled');
}
else if (enterName == name) {
    var enterPassword = prompt("Enter the password");
    if (enterPassword == '') {
        alert('Wrong Password');
    }
    else if (enterPassword == password) {
        alert('Welcome!');
    }
    else {
        alert('cancled');
    }

}
else {
    alert(`i don't know you`);
}



