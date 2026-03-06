use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::{SystemTime};

fn main() -> std::io::Result<()>{

    //creo un TcpListener escuchando en el puerto 3000
    let listener = TcpListener::bind("127.0.0.1:3000")?;

    //aceptar conexiones continuas y procesarlas
    for stream in listener.incoming(){
        handle_client(stream?);
    }

    Ok(())
}

//lee header del request http del tcpStream
fn handle_client(p0: TcpStream) {

    let mut lines = BufReader::new(p0); //para poder leerlo línea a línea

    let mut first_line = String::new(); //tomo la primera línea, me interesa pro el contenido
    lines.read_line(&mut first_line).unwrap(); // guardo el contenido

    //debo consumir el resto del contendio para que el stream no quede atascado y el cliente esperando
    for line in lines.by_ref().lines(){ //toma prestado el reader y luego lo devuelve
        let line = line.unwrap();
        if line.is_empty(){
            break; //paro de leer porque empiezan los headers
        }
    }

    //parseo la primera línea
    let mut slices: Vec<&str> = first_line.split(' ').collect(); //[0] GET [1] ruta [2]versión
    //acalro el tipo para que no sea unkown para el exterior. .collect() me permite acceder por índice

    //construyo la rta
    let route = &slices[1];
    let mut slash: Vec<&str> = route.split("/").collect();

    let response;
    if slash[1] == "pi" {
        match slash[2].parse::<u64>() {
            Ok(numero) => {
                let start = SystemTime::now();
                let result = liebniz(numero);
                let time = start.elapsed().unwrap();

                let body = format!(
                    "Valor de Pi para el termino {}: {} (Tiempo: {:?})",
                    slash[2],
                    result,
                    time,
                );

                response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
                    body.len(),
                    body
                );
            },
            Err(_) => {
                response = "HTTP/1.1 400 BAD REQUEST\r\nContent-Length: 0\r\n\r\n".to_string();
            }
        }
    } else{
        response = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\n\r\n".to_string();
    }

    //enviar rta
    lines.get_mut().write_all(response.as_bytes()).unwrap();
}

//servidor de socket tcp escuchando conexiones
// & referencia sin tomar ownership

fn liebniz(i: u64) -> f64{ //unsigned --> ntero y positivo

    let mut total :f64 = 0.0;

    for i in 0..=i{ //rango inclusivo
        total += (-1.0f64).powi(i as i32) / (2.0*i as f64 + 1.0);
    }
    4.0*total //no necesito el return
}

//liebniz es matemático, no teine sentido que pueda fallar