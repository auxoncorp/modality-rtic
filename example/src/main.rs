#![no_main]
#![no_std]

use heapless::{
    pool,
    pool::singleton::{Box, Pool},
};
use panic_semihosting as _;

pub struct ProbeReport {
    size: usize,
    buffer: [u8; 128],
}

impl ProbeReport {
    fn new() -> Self {
        ProbeReport {
            size: 0,
            buffer: [0; 128],
        }
    }
}

pool!(P: ProbeReport);

#[rtic::app(device = lm3s6965, dispatchers = [SSI0])]
mod app {
    use crate::{Box, Pool, ProbeReport, P};
    use cortex_m_semihosting::{debug, hprintln};
    use rtic::time::duration::*;
    use systick_monotonic::Systick;

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = Systick<100>; // 100 Hz / 10 ms granularity

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init(local = [memory: [u8; 4096] = [0; 4096]])]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        hprintln!("Initializing").ok();

        let systick = ctx.core.SYST;

        let mono = Systick::new(systick, 12_000_000);

        // Setup a memory pool for allocating probe reports
        P::grow(ctx.local.memory);

        foo::spawn_after(1.seconds()).unwrap();

        (Shared {}, Local {}, init::Monotonics(mono))
    }

    #[task(local = [cnt: u32 = 0])]
    #[modality_probe(name = FOO, size = 1024, local_name = probe)]
    fn foo(ctx: foo::Context) {
        hprintln!("foo").ok();

        *ctx.local.cnt += 1;

        record!(
            ctx.local.probe,
            EVENT_FOO,
            "Event FOO happened",
            tags!("foo", "RTIC")
        );

        // Send a probe report every 4th run of the task
        if *ctx.local.cnt % 4 == 0 {
            let mut r = P::alloc().unwrap().init(ProbeReport::new());
            if let Some(size) = ctx.local.probe.report(&mut r.buffer).unwrap() {
                r.size = size.get();
                comms_task::spawn(r).ok();
            }
        }

        if *ctx.local.cnt == 20 {
            hprintln!("All done").ok();
            debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
        }

        // Periodic ever 100 ms
        foo::spawn_after(100.milliseconds()).unwrap();
    }

    #[task(capacity = 4)]
    fn comms_task(_ctx: comms_task::Context, r: Box<P>) {
        hprintln!(
            "Comms task should send a ModalityProbe report size={}",
            r.size
        )
        .ok();
    }
}
