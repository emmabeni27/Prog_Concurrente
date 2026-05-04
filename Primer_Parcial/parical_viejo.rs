use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;

pub struct Runway { 
id: u32, 
is_occupied: bool, 
}

impl Runway {
    fn new(id: u32)-> Runway { 
    Runway { id, is_occupied: false, related: Vec<Runway> } 
        
    }
}
pub struct Plane { 
    id: u32 
    
}

impl Plane {
    fn new(id: u32)-> Plane { Plane { id } }
    }
    
pub struct Airport {
    runways: Mutex<Vec<Runway>>, //las pistas son el recurso compartido y van en un mutex
    cond: Condvar,
    }
    
impl Airport {
    fn request_runway(&self) -> i32{ //ocupar
        let mut runways = self.runways.lock().unwrap();

        loop{
        
            for r in runways.iter_mut(){
            
                if !r.is_occupied{
                    r.is_occupied = true;
                    for related in r.realted.iter_mut(){
                        related.is_occupied = true;
                    }
                    return r.id; //va a hacer solo el drop al salir del loop
                }
                
                runways = self.cond.wait(runways).unwrap()
            }
        }

    }
    }
    
    fn release_runway(&self, id:i32){
        let mut runways = self.runways.lock().unwrap();

        for r in runways.iter_mut(){
            if r.id == id {
                r.is_occupied = false;
                for related in r.related.iter_mut(){
                    related.is_occupied = false;
                }
                break; //arriba uso return, aca break! Pero cortar de alguna manera
            }

        drop(runways);
        self.cond.notify_all();
    }
    }
fn main() {

    let runways = (0..3).map(|i| Runway::new(i)).collect();
    let planes = (0..10).map(|i| Plane::new(i));
    let arc_airport = Arc::new(Airport::new(runways)); //mi monitor

    thread::scope(|s| {
        for plane in planes {
            let airport = arc_airport.clone();
            let plane_id = plane.id;
            s.spawn(move || {
            println!("Plane {}, requesting landing", plane_id);
            let runway_id = airport.request_runway();
            println!("Plane {}, landing on runway {}", plane_id, runway_id);
            thread::sleep(std::time::Duration::from_secs(1));
            println!("Plane {}, landed", plane_id);
            airport.release_runway(runway_id);
            });
        }
    });
}


--------------------------------------------------------------------------------------------------------------------------------------------------------------
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;

// ---------------- Runway ----------------

pub struct Runway {
    id: u32,
    is_occupied: bool,
    related: Vec<u32>,
}

impl Runway {
    fn new(id: u32) -> Runway {
        Runway {
            id,
            is_occupied: false,
            related: Vec::new(), // después podés cargar relaciones
        }
    }
}

// ---------------- Plane ----------------

pub struct Plane {
    id: u32,
}

impl Plane {
    fn new(id: u32) -> Plane {
        Plane { id }
    }
}

// ---------------- Airport ----------------

pub struct Airport {
    runways: Mutex<Vec<Runway>>,
    cond: Condvar,
}

impl Airport {
    fn new(runways: Vec<Runway>) -> Self {
        Airport {
            runways: Mutex::new(runways),
            cond: Condvar::new(),
        }
    }

    // -------- request --------
    fn request_runway(&self) -> u32 {
        let mut runways = self.runways.lock().unwrap();

        loop {
            for i in 0..runways.len() {
                if !runways[i].is_occupied {
                    // ocupar principal
                    runways[i].is_occupied = true;

                    // ocupar relacionadas (usando ids)
                    let related_ids = runways[i].related.clone();
                    for rel_id in related_ids {
                        if let Some(r) = runways.iter_mut().find(|r| r.id == rel_id) {
                            r.is_occupied = true;
                        }
                    }

                    return runways[i].id;
                }
            }

            runways = self.cond.wait(runways).unwrap();
        }
    }

    // -------- release --------
    fn release_runway(&self, id: u32) {
        let mut runways = self.runways.lock().unwrap();

        for i in 0..runways.len() {
            if runways[i].id == id {
                runways[i].is_occupied = false;

                let related_ids = runways[i].related.clone();
                for rel_id in related_ids {
                    if let Some(r) = runways.iter_mut().find(|r| r.id == rel_id) {
                        r.is_occupied = false;
                    }
                }

                break;
            }
        }

        drop(runways);
        self.cond.notify_all();
    }
}

// ---------------- main ----------------

fn main() {
    let runways: Vec<Runway> = (0..3).map(Runway::new).collect();
    let planes: Vec<Plane> = (0..10).map(Plane::new).collect();

    let airport = Arc::new(Airport::new(runways));

    thread::scope(|s| {
        for plane in planes {
            let airport = Arc::clone(&airport);
            let plane_id = plane.id;

            s.spawn(move || {
                println!("Plane {}, requesting landing", plane_id);

                let runway_id = airport.request_runway();

                println!("Plane {}, landing on runway {}", plane_id, runway_id);

                thread::sleep(Duration::from_secs(1));

                println!("Plane {}, landed", plane_id);

                airport.release_runway(runway_id);
            });
        }
    });
}
