mod exercices;
mod resume_exercices;
use ansi_term::{Colour, Style};
use resume_exercices::{exo1, exo2, exo3, exo4, exo5, exo6, exo7, exo8, exo9, exo10, exo11};

// Main de test
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
}
