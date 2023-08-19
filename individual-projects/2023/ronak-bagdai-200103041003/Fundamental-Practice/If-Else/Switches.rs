fn main() {
    let total_switches = 6;
    let working_switches = 4; 
    let not_working_switches = total_switches - working_switches;

    if working_switches == total_switches {
        println!("All switches are working.");
    } else if working_switches == 0 {
        println!("All switches are not working.");
    } else {
        println!("There are {} working switches and {} not working switches.", working_switches, not_working_switches);
    }
}
