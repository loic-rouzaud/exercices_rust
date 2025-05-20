mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use ansi_term::{Colour, Style};

fn main() {
    println!(
        "{}",
        Style::new().bold().underline().paint("\nBienvenue !\n")
    );
    println!("Usage :\n\tCompiler chaque fichier avec la commande rustc file_name && ./filename");
    println!("\tPour les fonctions qui ont des tests : rustc filename --test && ./filename");

    println!(
        "{}",
        Style::new()
            .bold()
            .underline()
            .fg(Colour::Green)
            .paint("\nExercices made by Loïc Rouzaud { Epitech } TOULOUSE\n")
    );
}
