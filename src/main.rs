use std::thread;
use std::time::Duration;

fn main() {
    // Cria uma nova thread e executa o fechamento dentro dela
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1)); // Pausa por 1 milissegundo
        }
    });

    // Código na thread principal continua a executar em paralelo
    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1)); // Pausa por 1 milissegundo
    }

    // Aguarda a thread criada terminar
    handle.join().unwrap();
}

