use std::io::{self, Write};
use rand::Rng;

fn main() {

    let loop_status = loop {

        print!("Podaj liczbę: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Nie udało się pobrać liczby!");

        let mut x = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            _ => break true,
        };

        if x == 0 {
            break false;
        }

        x += rand::rng().random_range(0..=5);
        println!("Nowa wartość: {x}");

        let pows = pows_of_2();
        for i in 0..pows.len() {

            let res = collatz_check(pows[i]);
            let msg =
                if res {"spełnia hipotezę"}
                else {"nie spełnia hipotezy"};
            println!("Liczba {} {} Collatza.", pows[i], msg);
        
        }

        use std::fs::File;
        let mut file = File::create("xyz.txt")
            .expect("Nie udało się stworzyć pliku!");
        for i in 0..pows.len() {
            writeln!(file, "{}", pows[i])
                .expect("Nie udało się zapisać do pliku!");
        }
    };

    println!("Pętla zakończona {}.",
        match loop_status {
            false => "z woli użytkownika",
            true => "z powodu błędu",
        }
    );

    tuple_thingy();
}

const SIZE: usize = 10;
fn pows_of_2() -> [u32; SIZE] {

    let mut arr: [u32; SIZE] = [0; SIZE];

    let mut tmp = 1;
    for i in 0..SIZE {
        arr[i] = tmp;
        tmp *= 2;
    }

    return arr;
}

fn collatz_next(num: u32) -> u32 {
    
    match num % 2 {
        0 => num / 2,
        1 => 3 * num + 1,
        _ => unreachable!(),
    }

}

fn collatz_check(mut num: u32) -> bool {

    const ITER_LIMIT: u32 = 100;
    for _ in 0..ITER_LIMIT {

        if num == 1 {
            return true
        }
        num = collatz_next(num);

    }
    false
}

fn tuple_thingy() {

    'my_label: loop {

        // różne typy
        let tuple: (u8, i16) = (
            rand::rng().random_range(0..u8::MAX),
            rand::rng().random_range(0..i16::MAX),
        );

        // podział krotki
        let (a, b) = tuple;

        // break with label
        if (a as i16) > b {
            println!("Stop, bo a > b.");
            break 'my_label;
        } else {
            println!("({a}, {b})");
        }
    }

}
