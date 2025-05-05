mod box_pointers;
mod fnmut_pointers;
mod rc_pointers;
mod resume_ex;
use ansi_term::{Colour, Style};
use resume_ex::{exo1, exo2, exo3, exo4, exo5};

// Main de test
fn main() {
    println!(
        "{}",
        Style::new()
            .bold()
            .underline()
            .fg(Colour::Green)
            .paint("Exercice 1")
    );
    exo1();

    println!("");
    println!(
        "{}",
        Style::new()
            .bold()
            .underline()
            .fg(Colour::Green)
            .paint("Exercice 2")
    );
    exo2();

    println!("");
    println!(
        "{}",
        Style::new()
            .bold()
            .underline()
            .fg(Colour::Green)
            .paint("Exercice 3")
    );
    exo3();

    println!("");
    println!(
        "{}",
        Style::new()
            .bold()
            .underline()
            .fg(Colour::Green)
            .paint("Exercice 4")
    );
    exo4();

    println!("");
    println!(
        "{}",
        Style::new()
            .bold()
            .underline()
            .fg(Colour::Green)
            .paint("Exercice 5")
    );
    exo5();
}
