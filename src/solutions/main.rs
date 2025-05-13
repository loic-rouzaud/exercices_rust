pub mod box_pointers;
pub mod error_handling;
pub mod fn_pointers;
pub mod hashmaps;
pub mod loops;
pub mod mutex_pointers;
pub mod rc_pointers;
pub mod refcell_pointer;
pub mod traits;

mod resume_solution;
use ansi_term::{Colour, Style};
use resume_solution::{
    exo1, exo10, exo11, exo12, exo13, exo2, exo3, exo4, exo5, exo6, exo7, exo8, exo9,
};

// main pour les exercices ⚠️ va changer ⚠️
fn main() {
    println!(
        "\n{}",
        Style::new()
            .bold()
            .underline()
            .fg(Colour::Green)
            .paint("Exercice 1")
    );
    exo1();

    println!(
        "\n{}",
        Style::new()
            .bold()
            .underline()
            .fg(Colour::Green)
            .paint("Exercice 2")
    );
    exo2();

    println!(
        "\n{}",
        Style::new()
            .bold()
            .underline()
            .fg(Colour::Green)
            .paint("Exercice 3")
    );
    exo3();

    println!(
        "\n{}",
        Style::new()
            .bold()
            .underline()
            .fg(Colour::Green)
            .paint("Exercice 4")
    );
    exo4();

    println!(
        "\n{}",
        Style::new()
            .bold()
            .underline()
            .fg(Colour::Green)
            .paint("Exercice 5")
    );
    exo5();

    println!(
        "\n{}",
        Style::new()
            .bold()
            .underline()
            .fg(Colour::Green)
            .paint("Exercice 6")
    );
    exo6();

    println!(
        "\n{}",
        Style::new()
            .bold()
            .underline()
            .fg(Colour::Green)
            .paint("Exercice 7")
    );
    exo7();

    println!(
        "\n{}",
        Style::new()
            .bold()
            .underline()
            .fg(Colour::Green)
            .paint("Exercice 8")
    );
    exo8();

    println!(
        "\n{}",
        Style::new()
            .bold()
            .underline()
            .fg(Colour::Green)
            .paint("Exercice 9")
    );
    exo9();

    println!(
        "\n{}",
        Style::new()
            .bold()
            .underline()
            .fg(Colour::Green)
            .paint("Exercice 10")
    );
    exo10();

    println!(
        "\n{}",
        Style::new()
            .bold()
            .underline()
            .fg(Colour::Green)
            .paint("Exercice 11")
    );
    exo11();

    println!(
        "\n{}",
        Style::new()
            .bold()
            .underline()
            .fg(Colour::Green)
            .paint("Exercice 12")
    );
    exo12();

    println!(
        "\n{}",
        Style::new()
            .bold()
            .underline()
            .fg(Colour::Green)
            .paint("Exercice 13")
    );
    exo13();
}
