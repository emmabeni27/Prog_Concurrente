use std::sync::{Arc, Mutex};
use tokio::sync::{Semaphore, Notify};
use tokio::time::{sleep, Duration};

//no alcanza con un if decorativo. 
#[tokio::main]
async fn main(){
    
    let salon = Arc::new(Semaphore::new(5));

    let prioritarios = Arc::new(Mutex::new(0));
    
    let notify = Arc::new(Notify::new());
    
    let mut handles = vec![];
    
    for i in 1..=10 {
    
        //comunes
        if i%2==1{
            let s = salon.clone();
            let p = prioritarios.clone();
            let n = notify.clone();
            //sgte equivale a thread spawn
            let handle = tokio::spawn(async move{ 
                loop{
                    let hay_prioritarios = {
                        let p = p.lock().unwrap();
                        *p>0 //sin ; para que retorno
                    };
                    
                    if ! hay_prioritarios {
                        let permit = s.acquire().await.unwrap();
                        println!("Cliente {i} entró");
                        sleep(Duration::from_secs(1)).await;
                        drop(permit);
                        println!("Cleinte {i} atendido");
                        break;
                    }
                    n.notified().await;
                }
            });
            
            handles.push(handle);
        }
        
        //prioritarios
        if i%2==0{
            let s = salon.clone();
            let p = prioritarios.clone();
            let n = notify.clone();
            let handle = tokio::spawn(async move{

                {
                let mut p = p.lock().unwrap();
                *p += 1;
                }
                
                let permit = s.acquire().await.unwrap();
                
                println!("Cliente {i} entró en espera");
                sleep(Duration::from_secs(1)).await; //duerme a la tarea, no al hilo
                
                drop(permit);
                
                {
                let mut p = p.lock().unwrap();
                *p -= 1;
                }
                
                println!("Cliente {i} atendido");
                n.notify_waiters();
            });
            
            handles.push(handle);
        }
        
    }
    for h in handles {
            h.await.unwrap();
        }     
}
