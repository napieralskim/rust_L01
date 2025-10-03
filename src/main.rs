use rand::Rng;

fn main() {

    let loop_status = loop {

        print!("Podaj liczbę: ");
        std::io::stdout().flush().unwrap();

        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Nie udało się pobrać liczby!");

        let mut x = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            _ => break true,
        };

        if x == 0 {
            break false;
        }

        x += rand::thread_rng().gen_range(0..=5);
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
        use std::io::prelude::*;
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

fn tuple_make() -> (u32, f32) {
    (5, 3.8)
}