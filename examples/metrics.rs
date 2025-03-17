use anyhow::Result;
use concurrency::Metrics;
use std::thread;

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = Metrics::new();

    for idx in 0..N {
        task_worker(idx, metrics.clone())?;
    }

    for _ in 0..M {
        request_worker(metrics.clone())?;
    }

    loop {
        thread::sleep(std::time::Duration::from_secs(5));
        println!("{}", metrics);
    }
}

fn task_worker(idx: usize, metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            //do a long term stuff
            let mut rng = rand::thread_rng();

            thread::sleep(std::time::Duration::from_millis(rand::Rng::gen_range(
                &mut rng,
                100..5000,
            )));
            metrics.inc(format!("call.thread.worker.{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });

    Ok(())
}

fn request_worker(metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            //do a long term stuff
            let mut rng = rand::thread_rng();

            thread::sleep(std::time::Duration::from_millis(rand::Rng::gen_range(
                &mut rng,
                50..800,
            )));

            let page = rand::Rng::gen_range(&mut rng, 1..20);
            metrics.inc(format!("req.page.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });

    Ok(())
}
