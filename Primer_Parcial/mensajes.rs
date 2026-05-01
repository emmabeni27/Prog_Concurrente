use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // 1) Crear el canal
    let (sender, receiver) = mpsc::channel::<String>();

    // 2) Crear muchos productores
    for tid in 0..10 { //en cada iteración crea un hilo y trabajan concurrentemente
        let s = sender.clone(); // cada hilo necesita su propio sender

        thread::spawn(move || {
            let msg = format!("Hola desde el hilo {tid}");
            println!("Enviando: {}", msg);
            s.send(msg).unwrap();
        });
    }

    // 3) Consumidor: recibe mensajes hasta que pase 1 segundo sin recibir nada
    loop {
        match receiver.recv_timeout(Duration::from_secs(1)) {
            Ok(msg) => println!("Recibido: {}", msg),
            Err(_) => {
                println!("No llegaron más mensajes. Terminando.");
                break;
            }
        }
    }
}
