use std::sync::{Arc, Mutex};
use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let sillas = 3;

    // Semáforo que cuenta clientes esperando
    let clientes = Arc::new(Semaphore::new(0));

    // Semáforo que indica que el barbero está listo
    let barbero = Arc::new(Semaphore::new(0));

    // Mutex para proteger la cantidad de sillas ocupadas
    let sillas_ocupadas = Arc::new(Mutex::new(0));

    // ---------------- BARBERO ----------------
    {
        let clientes = clientes.clone();
        let barbero = barbero.clone();
        let sillas_ocupadas = sillas_ocupadas.clone();

        tokio::spawn(async move {
            loop {
                // Espera a que haya un cliente
                clientes.acquire().await.unwrap().forget();

                {
                    let mut ocupadas = sillas_ocupadas.lock().unwrap();
                    *ocupadas -= 1;
                    println!("Barbero llama al siguiente cliente. Sillas ocupadas: {}", *ocupadas);
                }

                // Barbero listo para atender
                barbero.add_permits(1);

                println!("Barbero está cortando el pelo...");
                sleep(Duration::from_millis(800)).await;
                println!("Barbero terminó un corte.");
            }
        });
    }

    // ---------------- CLIENTES ----------------
    for id in 0..10 {
        let clientes = clientes.clone();
        let barbero = barbero.clone();
        let sillas_ocupadas = sillas_ocupadas.clone();

        tokio::spawn(async move {
            loop {
                sleep(Duration::from_millis(300 * id as u64)).await;

                let mut ocupadas = sillas_ocupadas.lock().unwrap();

                if *ocupadas < sillas {
                    *ocupadas += 1;
                    println!("Cliente {id} se sienta. Sillas ocupadas: {}", *ocupadas);

                    // Señala que hay un cliente esperando
                    clientes.add_permits(1);

                    drop(ocupadas);

                    // Espera a que el barbero lo llame
                    barbero.acquire().await.unwrap().forget();

                    println!("Cliente {id} está siendo atendido.");
                    sleep(Duration::from_millis(500)).await;
                } else {
                    println!("Cliente {id} se va: no hay sillas.");
                }
            }
        });
    }

    loop {
        sleep(Duration::from_secs(1)).await;
    }
}
