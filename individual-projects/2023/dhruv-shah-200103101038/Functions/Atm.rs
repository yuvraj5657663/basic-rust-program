let note = [2,5,1];
fn deposit (d,two_thousand){
    let note = two_thousand + d;
    return note;
}
fn withdraw (t,f,o) {
    let two_note = two_thousand - t;
    let five_note = five_hundred - f;
    let one_note = one_hundred - o;
    return [two_note, five_note, one_note];
}
fn main(){
    two_thousand = deposit(5,two_thousand);
    let note = withdraw(2,3,5);
    two_thousand = note[0];
    five_hundred = note[1];
    one_hundred = note[2];
}
