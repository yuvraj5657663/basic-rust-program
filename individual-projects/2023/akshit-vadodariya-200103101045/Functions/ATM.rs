let notes = [5,5,5];

fn deposit(a,b,c) {
    return [notes[0]+a, notes[1]+b, notes[2]+c];
}

fn withdraw(x,y,z) {
    return [notes[0]-x, notes[1]-y, notes[2]-z]
}

fn main() {
    notes = deposit(2,4,5);
    notes = withdraw(1,0,5);
}