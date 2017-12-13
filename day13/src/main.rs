extern crate day13;
use day13::Firewall;

extern crate util;
use util::file_as_by;

fn main() {
    let mut firewall = Firewall::new();

    for rule in file_as_by::<usize, _>("input.txt", |line| line.split(": ").collect())
        .expect("problem reading input")
    {
        if rule.len() == 0 {
            continue;
        }
        assert_eq!(rule.len(), 2, "rule must have 2 parts");
        firewall.add_layer(rule[0], rule[1]);
    }

    #[cfg(debug_assertions)]
    println!("Determining severity of 0-delay start...");
    println!(
        "Severity of starting without delay: {}",
        firewall.traversal_severity(0)
    );
    #[cfg(debug_assertions)]
    {
        println!("");
        println!("Finding undetected delay...");
    }
    println!(
        "First delay which sneaks through undetected: {}",
        firewall.find_first_uncaught_delay()
    );
}
