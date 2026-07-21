use std::thread;
use std::time::Duration;
use std::sync::mpsc;

struct Move {
    from: usize,
    to: usize
}

// towers of hanoi algorithm

fn hanoi(tx: &mpsc::Sender<Move>,
    from: usize,
    to: usize,
    n: usize) {

    if n == 1 {
        thread::sleep(Duration::from_secs(3));
        tx.send(Move{to, from})
            .expect("Failed to send move");
    }
    else {
        let other = 3 - (from + to);
        hanoi(&tx, from, other, n - 1);
        hanoi(&tx, from, to, 1);
        hanoi(&tx, other, to, n -1);
    }
}

fn print_pegs(pegs: &[Vec<&str>; 3]) {
    println!("\x1B[H\x1b[2J");

    for peg in 0..pegs.len() {
        print!("\x1B[32m|-");                           // added color for fun

        for disk in 0..pegs[peg].len() {
            print!("\x1B[33m{} ", pegs[peg][disk]);     // added color for fun
        }
        println!("\x1B[0m");                            // reset colors
    }
}

fn main() {
    
    let (tx, rx) = mpsc::channel();

    let mut pegs = [Vec::new(), Vec::new(), Vec::new()];

    let disk_labels = ["A", "B", "C", "D"];
    let n = disk_labels.len();

    for label in disk_labels {
        pegs[0].push(label);
    }

    print_pegs(&pegs);

    let handle = thread::spawn(move || hanoi(&tx, 0, 2, n));

    loop {

        if let Ok(msg) = rx.recv() {

            let disk = pegs[msg.from].pop()
                .expect(&format!(
                    "Can't take disk from peg {}", msg.from));
                
            pegs[msg.to].push(disk);

            print_pegs(&pegs);
        }
        else {
            break;
        }
    }
    handle.join().unwrap();
}