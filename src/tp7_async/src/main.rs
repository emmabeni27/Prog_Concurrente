use clap::Parser;
use std::time::Instant;
use tokio::task;

// ── CLI ──────────────────────────────────────────────────────────────────────

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Modelo: "threads" o "async"
    #[arg(long)]
    model: String,

    /// Tipo de tarea: "io" o "cpu"
    #[arg(long)]
    tipo: String,

    /// Número de tareas concurrentes
    #[arg(long)]
    tasks: usize,

    /// Cantidad total de términos para Leibniz (solo cpu)
    #[arg(long, default_value_t = 0)]
    terms: usize,

    /// Número de run (para el CSV)
    #[arg(long, default_value_t = 1)]
    run: usize,

    /// Duración del sleep simulado en ms (solo io)
    #[arg(long, default_value_t = 100)]
    sleep_ms: u64,
}

// ── Leibniz ───────────────────────────────────────────────────────────────────

fn leibniz_pi_partial(start: usize, count: usize) -> f64 {
    (start..start + count)
        .map(|k| {
            let sign = if k % 2 == 0 { 1.0 } else { -1.0 };
            sign / (2.0 * k as f64 + 1.0)
        })
        .sum::<f64>()
}

// ── THREADS: I/O ──────────────────────────────────────────────────────────────

fn run_threads_io(tasks: usize, sleep_ms: u64) -> (u128, &'static str) {
    let start = Instant::now();
    let mut handles = Vec::with_capacity(tasks);

    for _ in 0..tasks {
        let h = std::thread::Builder::new()
            .stack_size(64 * 1024) // stack mínimo para no agotar memoria
            .spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(sleep_ms));
            });

        match h {
            Ok(handle) => handles.push(handle),
            Err(_) => return (start.elapsed().as_millis(), "error_spawn"),
        }
    }

    for h in handles {
        if h.join().is_err() {
            return (start.elapsed().as_millis(), "error_join");
        }
    }

    (start.elapsed().as_millis(), "ok")
}

// ── THREADS: CPU ──────────────────────────────────────────────────────────────

fn run_threads_cpu(tasks: usize, terms: usize) -> (u128, &'static str, f64) {
    let start = Instant::now();
    let terms_per_task = terms / tasks;
    let mut handles = Vec::with_capacity(tasks);

    for i in 0..tasks {
        let start_term = i * terms_per_task;
        let count = if i == tasks - 1 {
            terms - start_term // última tarea toma el resto
        } else {
            terms_per_task
        };

        let h = std::thread::Builder::new()
            .stack_size(64 * 1024)
            .spawn(move || leibniz_pi_partial(start_term, count));

        match h {
            Ok(handle) => handles.push(handle),
            Err(_) => return (start.elapsed().as_millis(), "error_spawn", 0.0),
        }
    }

    let mut partial_results = Vec::with_capacity(tasks);
    for h in handles {
        match h.join() {
            Ok(v) => partial_results.push(v),
            Err(_) => return (start.elapsed().as_millis(), "error_join", 0.0),
        }
    }

    let pi: f64 = partial_results.iter().sum::<f64>() * 4.0;
    (start.elapsed().as_millis(), "ok", pi)
}

// ── ASYNC: I/O ────────────────────────────────────────────────────────────────

async fn run_async_io(tasks: usize, sleep_ms: u64) -> (u128, &'static str) {
    let start = Instant::now();
    let mut handles = Vec::with_capacity(tasks);

    for _ in 0..tasks {
        let h = task::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(sleep_ms)).await;
        });
        handles.push(h);
    }

    for h in handles {
        if h.await.is_err() {
            return (start.elapsed().as_millis(), "error_join");
        }
    }

    (start.elapsed().as_millis(), "ok")
}

// ── ASYNC: CPU ────────────────────────────────────────────────────────────────

async fn run_async_cpu(tasks: usize, terms: usize) -> (u128, &'static str, f64) {
    let start = Instant::now();
    let terms_per_task = terms / tasks;
    let mut handles = Vec::with_capacity(tasks);

    for i in 0..tasks {
        let start_term = i * terms_per_task;
        let count = if i == tasks - 1 {
            terms - start_term
        } else {
            terms_per_task
        };

        // spawn_blocking para no bloquear el runtime de Tokio con trabajo CPU-bound
        let h = task::spawn_blocking(move || leibniz_pi_partial(start_term, count));
        handles.push(h);
    }

    let mut partial_results = Vec::with_capacity(tasks);
    for h in handles {
        match h.await {
            Ok(v) => partial_results.push(v),
            Err(_) => return (start.elapsed().as_millis(), "error_join", 0.0),
        }
    }

    let pi: f64 = partial_results.iter().sum::<f64>() * 4.0;
    (start.elapsed().as_millis(), "ok", pi)
}

// ── MAIN ──────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let (time_ms, status, extra) = match (args.model.as_str(), args.tipo.as_str()) {
        ("threads", "io") => {
            let (t, s) = run_threads_io(args.tasks, args.sleep_ms);
            (t, s.to_string(), String::new())
        }
        ("threads", "cpu") => {
            let (t, s, pi) = run_threads_cpu(args.tasks, args.terms);
            (t, s.to_string(), format!(" [pi≈{:.8}]", pi))
        }
        ("async", "io") => {
            let (t, s) = run_async_io(args.tasks, args.sleep_ms).await;
            (t, s.to_string(), String::new())
        }
        ("async", "cpu") => {
            let (t, s, pi) = run_async_cpu(args.tasks, args.terms).await;
            (t, s.to_string(), format!(" [pi≈{:.8}]", pi))
        }
        _ => {
            eprintln!("Combinación inválida. Usá --model {{threads|async}} --tipo {{io|cpu}}");
            std::process::exit(1);
        }
    };

    // Línea CSV lista para redirigir
    let terms_field = if args.tipo == "cpu" {
        args.terms.to_string()
    } else {
        String::new()
    };

    println!(
        "{},{},{},{},{},{},{}{}",
        args.model, args.tipo, args.tasks, terms_field, args.run, time_ms, status, extra
    );
}