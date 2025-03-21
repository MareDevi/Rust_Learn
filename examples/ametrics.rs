use anyhow::Result;
use concurrency::AmapMetrics;
use std::thread;

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = AmapMetrics::new(&[
        "call.thread.worker.0",
        "call.thread.worker.1",
        "req.page.1",
        "req.page.2",
        "req.page.3",
        "req.page.4",
        "req.page.5",
        "req.page.6",
        "req.page.7",
        "req.page.8",
        "req.page.9",
        "req.page.10",
        "req.page.11",
        "req.page.12",
        "req.page.13",
        "req.page.14",
        "req.page.15",
        "req.page.16",
        "req.page.17",
        "req.page.18",
        "req.page.19",
    ]);

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

fn task_worker(idx: usize, metrics: AmapMetrics) -> Result<()> {
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

fn request_worker(metrics: AmapMetrics) -> Result<()> {
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
