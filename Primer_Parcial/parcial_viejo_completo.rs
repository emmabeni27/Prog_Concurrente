use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::{Duration, Instant};

// ---------------- Runway ----------------

pub struct Runway {
    id: u32,
    is_occupied: bool,
}

impl Runway {
    fn new(id: u32) -> Runway {
        Runway {
            id,
            is_occupied: false,
        }
    }
}

// ---------------- Plane ----------------

pub struct Plane {
    id: u32,
    priority: u32,
}

impl Plane {
    fn new(id: u32, priority: u32) -> Plane {
        Plane { id, priority }
    }
}

// ---------------- Waiting Plane ----------------

struct WaitingPlane {
    id: u32,
    priority: u32,
    arrival: Instant,
}

// ---------------- Estado (Monitor) ----------------

struct Estado {
    runways: Vec<Runway>,
    waiting: Vec<WaitingPlane>,
}

// ---------------- Airport ----------------

pub struct Airport {
    estado: Mutex<Estado>,
    cond: Condvar,
}

impl Airport {
    fn new(runways: Vec<Runway>) -> Self {
        Airport {
            estado: Mutex::new(Estado {
                runways,
                waiting: Vec::new(),
            }),
            cond: Condvar::new(),
        }
    }

    // -------- request runway --------

    fn request_runway(&self, plane_id: u32, priority: u32) -> u32 {
        let mut st = self.estado.lock().unwrap();

        // me agrego a la cola
        st.waiting.push(WaitingPlane {
            id: plane_id,
            priority,
            arrival: Instant::now(),
        });

        loop {
            // 1. hay pista libre?
            let free_index = st.runways.iter().position(|r| !r.is_occupied);

            // 2. soy el mejor candidato?
            let best_plane = st.waiting.iter().min_by_key(|p| {
                (p.priority, p.arrival)
            });

            let is_my_turn = match best_plane {
                Some(p) => p.id == plane_id,
                None => false,
            };

            if let Some(idx) = free_index {
                if is_my_turn {
                    // ocupar pista
                    st.runways[idx].is_occupied = true;

                    // salir de la cola
                    st.waiting.retain(|p| p.id != plane_id);

                    return st.runways[idx].id;
                }
            }

            // esperar
            st = self.cond.wait(st).unwrap();
        }
    }

    // -------- release runway --------

    fn release_runway(&self, runway_id: u32) {
        let mut st = self.estado.lock().unwrap();

        if let Some(r) = st.runways.iter_mut().find(|r| r.id == runway_id) {
            r.is_occupied = false;
        }

        drop(st);
        self.cond.notify_all();
    }
}

// ---------------- main ----------------

fn main() {
    let runways: Vec<Runway> = (0..3).map(Runway::new).collect();

    // prioridades arbitrarias (menor = más prioridad)
    let planes: Vec<Plane> = (0..10)
        .map(|i| Plane::new(i, i % 3))
        .collect();

    let airport = Arc::new(Airport::new(runways));

    thread::scope(|s| {
        for plane in planes {
            let airport = Arc::clone(&airport);
            let plane_id = plane.id;
            let priority = plane.priority;

            s.spawn(move || {
                println!(
                    "Plane {} requesting landing (priority {})",
                    plane_id, priority
                );

                let runway_id = airport.request_runway(plane_id, priority);

                println!(
                    "Plane {} landing on runway {}",
                    plane_id, runway_id
                );

                thread::sleep(Duration::from_secs(1));

                println!("Plane {} landed", plane_id);

                airport.release_runway(runway_id);
            });
        }
    });
}
