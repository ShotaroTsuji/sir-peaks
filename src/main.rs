use eom::traits::*;
use ndarray::arr1;
use structopt::StructOpt;

#[derive(Debug,Clone,StructOpt)]
enum Opt {
    /// Produce a numerical solution of the SIR model.
    Simulate {
        /// The value of the parameter λ
        #[structopt(long,short)]
        lambda: f64,
        /// The value of the parameter γ
        #[structopt(long,short,default_value="1.0")]
        gamma: f64,
        /// The initial value of the infected population
        #[structopt(long,short,default_value="1e-7")]
        infected: f64,
        /// The total population
        #[structopt(short="n",default_value="1.0")]
        population: f64,
    },
    /// Compute the peak values of the infected population
    Peaks {
        /// The start value of the parameter λ
        #[structopt(long,short)]
        start: f64,
        /// The end value of the parameter λ
        #[structopt(long,short)]
        end: f64,
        /// The step size of incrementing the parameter λ
        #[structopt(short="d")]
        step: f64,
        /// The initial value of the infected population
        #[structopt(long,short)]
        infected: f64,
    }
}

fn simulate(l: f64, g: f64, i: f64, n: f64) {
    let e = n.log10().floor();
    let s = n / 10.0_f64.powf(e);
    let s = s.floor();
    let dt = 0.01 / (s * 10.0_f64.powf(e));
    let print_interval = (s * 10.0_f64.powf(e+1.0)) as usize;
    eprintln!("dt = {}", dt);

    let eom = sir_peaks::SirModel { lambda: l, gamma: g };
    let mut teo = eom::explicit::RK4::new(eom, dt);
    let ts = eom::adaptor::time_series(arr1(&[n-i, i, 0.0]), &mut teo);
    let mut counter = 0;
    for v in ts.take_while(|v| v[1] > i/10.0) {
        if counter == 0 {
            println!("{} {} {}",v[0], v[1], v[2]);
        }

        if counter == print_interval-1 {
            counter = 0;
        } else {
            counter = counter + 1;
        }
    }
}

fn peaks(i: f64, start: f64, end: f64, step: f64) {
    let dt = 0.01;
    let initial_condition = arr1(&[1.0-i, i, 0.0]);

    let mut l = start;
    while l < end+step {
        let eom = sir_peaks::SirModel { lambda: l, gamma: 1.0 };
        let mut teo = eom::explicit::RK4::new(eom, dt);
        let ts = eom::adaptor::time_series(initial_condition.clone(), &mut teo);
        let peak = ts.take_while(|v| v[1] > i/10.0)
            .fold(0.0/0.0, |m, v| v[1].max(m));
        println!("{} {}", l, peak);
        l += step;
    }
}

fn main() {
    let opt = Opt::from_args();

    match opt {
        Opt::Simulate { lambda: l, gamma: g, infected: i, population: n } => {
            eprintln!("# simulation");
            eprintln!("## parameters");
            eprintln!("- l : {}", l);
            eprintln!("- g : {}", g);
            eprintln!("## initial values");
            eprintln!("- S : {}", n-i);
            eprintln!("- I : {}", i);
            eprintln!("- R : {}", 0.0);
            eprintln!("## reproduction number");
            eprintln!("{}", l*n/g);
            simulate(l, g, i, n);
        },
        Opt::Peaks { start, end, step, infected } => {
            eprintln!("# peaks");
            eprintln!("## parameters");
            eprintln!("- l : {}:{}", start, end);
            eprintln!("- g : {}", 1.0);
            eprintln!("## initial values");
            eprintln!("- S : {}", 1.0-infected);
            eprintln!("- I : {}", infected);
            eprintln!("- R : {}", 0.0);
            peaks(infected, start, end, step);
        }
    }
}
