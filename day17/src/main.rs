const INPUT: usize = 349;

extern crate day17;
use day17::{Spinner, FastSpinner, TWENTY_SEVENTEEN, FIFTY_MILLION};

fn main() {
    let mut spinner = Spinner::new();
    spinner.insert_until(TWENTY_SEVENTEEN, INPUT);
    println!(
        "Item after 2017 inserts: {}",
        spinner.get_items()[spinner.get_index() + 1]
    );
    let mut fspinner = FastSpinner::new();
    fspinner.insert_until(FIFTY_MILLION, INPUT);
    // we know that 0 is in the list
    println!(
        "Item after 0 after 50 million inserts: {}",
        fspinner.get_after_zero()
    );
}
