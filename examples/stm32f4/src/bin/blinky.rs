#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use panic_probe as _;
use rtos_trace;
use systemview_target::SystemView;

static LOGGER: systemview_target::SystemView = systemview_target::SystemView::new();
rtos_trace::global_trace! {SystemView}

struct TraceInfo();

impl rtos_trace::RtosTraceApplicationCallbacks for TraceInfo {
    fn system_description() {}
    fn sysclock() -> u32 {
        250000000
    }
}
rtos_trace::global_application_callbacks! {TraceInfo}

#[embassy_executor::task]
async fn run(mut led: Output<'static>) {
    loop {
        led.set_high();
        Timer::after_millis(300).await;
        led.set_low();
        Timer::after_millis(300).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    LOGGER.init();
    let led = Output::new(p.PB7, Level::High, Speed::Low);
    spawner.spawn(run(led)).unwrap();
}

//use embassy_executor::Spawner;
//use embassy_stm32::gpio::{Level, Output, Speed};
//use embassy_time::{Instant, Timer};
//use systemview_target::SystemView;
//use log::*;
//use panic_probe as _;
//use rtos_trace;
//
//static LOGGER: systemview_target::SystemView = systemview_target::SystemView::new();
//struct TraceInfo();
//rtos_trace::global_application_callbacks! {TraceInfo}
//
//impl rtos_trace::RtosTraceApplicationCallbacks for TraceInfo {
//    fn system_description() {}
//    fn sysclock() -> u32 {
//        64000000
//    }
//}
//
//#[embassy_executor::task]
//async fn run() {
//    loop {
//        log::info!("tick");
//        Timer::after_millis(300).await;
//    }
//}
//
//#[embassy_executor::main]
//async fn main(spawner: Spawner) {
//    let p = embassy_stm32::init(Default::default());
//
//    spawner.spawn(run()).unwrap();
//}
//
//#[unsafe(no_mangle)]
//unsafe extern "Rust" fn _embassy_trace_task_new(executor: u32, task: u32) {
//    log::info!("new task: {} {} {}", executor, task, Instant::now().as_ticks());
//}
//
//#[unsafe(no_mangle)]
//unsafe extern "Rust" fn _embassy_trace_task_exec_begin(executor: u32, task: u32) {}
//
//#[unsafe(no_mangle)]
//unsafe extern "Rust" fn _embassy_trace_task_exec_end(executor: u32, task: u32) {}
//
//#[unsafe(no_mangle)]
//unsafe extern "C" fn _embassy_trace_executor_idle(executor: u32) {}
//
//#[unsafe(no_mangle)]
//unsafe extern "C" fn _embassy_trace_task_ready_begin(executor: u32, task: u32) {}
