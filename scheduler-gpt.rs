use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Clone)]
struct Process {
    name: String,
    arrival: usize,
    burst: usize,
    remaining: usize,
    completion: Option<usize>,
    first_start: Option<usize>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    Fcfs,
    Sjf,
    Rr,
}

struct Config {
    processcount: usize,
    runfor: usize,
    algorithm: Algorithm,
    quantum: Option<usize>,
    processes: Vec<Process>,
}

fn parse_usize(s: &str) -> Option<usize> {
    s.parse::<usize>().ok()
}

fn parse_input(contents: &str) -> Result<Config, String> {
    let mut processcount: Option<usize> = None;
    let mut runfor: Option<usize> = None;
    let mut algorithm: Option<Algorithm> = None;
    let mut quantum: Option<usize> = None;
    let mut processes: Vec<Process> = Vec::new();

    for raw_line in contents.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "processcount" => {
                if parts.len() >= 2 {
                    processcount = parse_usize(parts[1]);
                }
            }
            "runfor" => {
                if parts.len() >= 2 {
                    runfor = parse_usize(parts[1]);
                }
            }
            "use" => {
                if parts.len() >= 2 {
                    algorithm = match parts[1] {
                        "fcfs" => Some(Algorithm::Fcfs),
                        "sjf" => Some(Algorithm::Sjf),
                        "rr" => Some(Algorithm::Rr),
                        _ => None,
                    };
                }
            }
            "quantum" => {
                if parts.len() >= 2 {
                    quantum = parse_usize(parts[1]);
                }
            }
            "process" => {
                if parts.len() >= 7
                    && parts[1] == "name"
                    && parts[3] == "arrival"
                    && parts[5] == "burst"
                {
                    let name = parts[2].to_string();
                    if let (Some(arrival), Some(burst)) = (parse_usize(parts[4]), parse_usize(parts[6])) {
                        processes.push(Process {
                            name,
                            arrival,
                            burst,
                            remaining: burst,
                            completion: None,
                            first_start: None,
                        });
                    }
                }
            }
            "end" => break,
            _ => {}
        }
    }

    if processcount.is_none() {
        return Err("Error: Missing parameter processcount.".to_string());
    }
    if runfor.is_none() {
        return Err("Error: Missing parameter runfor.".to_string());
    }
    if algorithm.is_none() {
        return Err("Error: Missing parameter use.".to_string());
    }

    let algorithm = algorithm.unwrap();
    if algorithm == Algorithm::Rr && quantum.is_none() {
        return Err("Error: Missing quantum parameter when use is 'rr'".to_string());
    }

    Ok(Config {
        processcount: processcount.unwrap(),
        runfor: runfor.unwrap(),
        algorithm,
        quantum,
        processes,
    })
}

fn choose_sjf(processes: &[Process], time: usize) -> Option<usize> {
    let mut best: Option<usize> = None;
    for i in 0..processes.len() {
        let p = &processes[i];
        if p.arrival <= time && p.remaining > 0 {
            if let Some(bi) = best {
                let b = &processes[bi];
                let better = p.remaining < b.remaining
                    || (p.remaining == b.remaining && p.arrival < b.arrival)
                    || (p.remaining == b.remaining && p.arrival == b.arrival && p.name < b.name);
                if better {
                    best = Some(i);
                }
            } else {
                best = Some(i);
            }
        }
    }
    best
}

fn simulate(mut cfg: Config) -> String {
    let mut out = String::new();

    fn line_arrived(out: &mut String, t: usize, name: &str) {
        out.push_str(&format!("Time {:>3} : {} arrived\n", t, name));
    }

    fn line_selected(out: &mut String, t: usize, name: &str, burst: usize) {
        out.push_str(&format!(
            "Time {:>3} : {} selected (burst {:>3})\n",
            t, name, burst
        ));
    }

    fn line_finished(out: &mut String, t: usize, name: &str) {
        out.push_str(&format!("Time {:>3} : {} finished\n", t, name));
    }

    fn line_idle(out: &mut String, t: usize) {
        out.push_str(&format!("Time {:>3} : Idle\n", t));
    }

    out.push_str(&format!("{:>3} processes\n", cfg.processcount));
    match cfg.algorithm {
        Algorithm::Fcfs => out.push_str("Using First-Come First-Served\n"),
        Algorithm::Sjf => out.push_str("Using preemptive Shortest Job First\n"),
        Algorithm::Rr => out.push_str("Using Round-Robin\n"),
    }
    if cfg.algorithm == Algorithm::Rr {
        out.push_str(&format!("Quantum {:>3}\n", cfg.quantum.unwrap()));
        out.push('\n');
    }

    let mut running: Option<usize> = None;
    let mut rr_queue: VecDeque<usize> = VecDeque::new();
    let mut fcfs_queue: VecDeque<usize> = VecDeque::new();
    let mut rr_slice_left: usize = 0;

    for t in 0..cfg.runfor {
        for i in 0..cfg.processes.len() {
            if cfg.processes[i].arrival == t {
                line_arrived(&mut out, t, &cfg.processes[i].name);
                match cfg.algorithm {
                    Algorithm::Fcfs => fcfs_queue.push_back(i),
                    Algorithm::Rr => rr_queue.push_back(i),
                    Algorithm::Sjf => {}
                }
            }
        }

        if let Some(idx) = running {
            if cfg.processes[idx].remaining == 0 {
                cfg.processes[idx].completion = Some(t);
                line_finished(&mut out, t, &cfg.processes[idx].name);
                running = None;
                rr_slice_left = 0;
            } else if cfg.algorithm == Algorithm::Rr && rr_slice_left == 0 {
                rr_queue.push_back(idx);
                running = None;
            }
        }

        match cfg.algorithm {
            Algorithm::Fcfs => {
                if running.is_none() {
                    if let Some(idx) = fcfs_queue.pop_front() {
                        running = Some(idx);
                        if cfg.processes[idx].first_start.is_none() {
                            cfg.processes[idx].first_start = Some(t);
                        }
                        line_selected(&mut out, t, &cfg.processes[idx].name, cfg.processes[idx].remaining);
                    }
                }
            }
            Algorithm::Sjf => {
                let next = choose_sjf(&cfg.processes, t);
                if next != running {
                    if let Some(idx) = next {
                        if cfg.processes[idx].first_start.is_none() {
                            cfg.processes[idx].first_start = Some(t);
                        }
                        line_selected(&mut out, t, &cfg.processes[idx].name, cfg.processes[idx].remaining);
                    }
                }
                running = next;
            }
            Algorithm::Rr => {
                if running.is_none() {
                    if let Some(idx) = rr_queue.pop_front() {
                        running = Some(idx);
                        rr_slice_left = cfg.quantum.unwrap();
                        if cfg.processes[idx].first_start.is_none() {
                            cfg.processes[idx].first_start = Some(t);
                        }
                        line_selected(&mut out, t, &cfg.processes[idx].name, cfg.processes[idx].remaining);
                    }
                }
            }
        }

        if running.is_none() {
            line_idle(&mut out, t);
        } else {
            let idx = running.unwrap();
            cfg.processes[idx].remaining -= 1;
            if cfg.algorithm == Algorithm::Rr && rr_slice_left > 0 {
                rr_slice_left -= 1;
            }
        }
    }

    if let Some(idx) = running {
        if cfg.processes[idx].remaining == 0 {
            cfg.processes[idx].completion = Some(cfg.runfor);
            line_finished(&mut out, cfg.runfor, &cfg.processes[idx].name);
        }
    }

    out.push_str(&format!("Finished at time {:>3}\n", cfg.runfor));
    out.push('\n');

    let mut order: Vec<usize> = (0..cfg.processes.len()).collect();
    order.sort_by(|&a, &b| cfg.processes[a].name.cmp(&cfg.processes[b].name));

    for idx in order {
        let p = &cfg.processes[idx];
        if let Some(done) = p.completion {
            let turnaround = done as isize - p.arrival as isize;
            let wait = turnaround - p.burst as isize;
            let response = p.first_start.unwrap_or(p.arrival) as isize - p.arrival as isize;
            out.push_str(&format!(
                "{} wait {:>3} turnaround {:>3} response {:>3}\n",
                p.name, wait, turnaround, response
            ));
        } else {
            out.push_str(&format!("{} did not finish\n", p.name));
        }
    }

    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: scheduler-gpt <input file>");
        return;
    }

    let input_path = &args[1];
    let contents = match fs::read_to_string(input_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            return;
        }
    };

    let cfg = match parse_input(&contents) {
        Ok(c) => c,
        Err(msg) => {
            println!("{}", msg);
            return;
        }
    };

    let output = simulate(cfg);
    let out_path = Path::new(input_path).with_extension("out");
    if let Err(e) = fs::write(&out_path, output) {
        eprintln!("Error writing output file: {}", e);
    }
}
