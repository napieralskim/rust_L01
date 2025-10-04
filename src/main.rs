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

    //--------------------------------------------------------------------------

    let board = [
        [ 0, 1, 2 ,3],
        [ 4, 5, 6, 7],
        [ 8, 9,10,11],
        [12,13,14,15],
    ];
    
    'outer:
    for y in 0..board.len() {
        let row = &board[y];
        for x in 0..row.len() {
            let elem = row[x];

            println!("pole {elem}");
            if elem == 9 {
                break 'outer;
            }            
        }
    }
    println!("znaleziono dziewiątkę");

    let coords = make_tuple();
    println!("wygenerowana tupla: (x: {}, y: {})", coords.0, coords.1);
    println!("wartość pod tymi współrzędnymi: {}", board[coords.1 as usize][coords.0 as usize])
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

// naciągany przykład, no ale różne typy są
fn make_tuple() -> (i8, u16) {
    (
        rand::rng().random_range(0..4),
        rand::rng().random_range(0..4),
    )
}
