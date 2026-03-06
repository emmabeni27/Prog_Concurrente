use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

fn main()  -> std::io::Result<()>{

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
    for line in lines.lines(){
        let line = line.unwrap();
        if line.is_empty(){
            break; //paro de leer porque empiezan los headers
        }
    }

    //parseo la primera línea
    let mut slices = first_line.split(" "); //[0] GET [1] ruta [2]versión
}

//servidor de socket tcp escuchando conexiones
// & referencia sin tomar ownership