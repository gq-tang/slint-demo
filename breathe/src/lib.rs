use std::rc::Rc;
use std::sync::atomic;
use std::time::{self, Duration};

use slint::TimerMode;

slint::include_modules!();

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(all(debug_assertions, target_arch = "wasm32"))]
fn enable_panic_hook() {
    console_error_panic_hook::set_once();
}

#[cfg(not(all(debug_assertions, target_arch = "wasm32")))]
fn enable_panic_hook() {}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    enable_panic_hook();

    let app = MainWindow::new().expect("MainWindow initial failed");

    let pre_timer = Rc::new(slint::Timer::default());

    let app_weak = app.as_weak();
    app.on_prepare(move || {
        let app = match app_weak.upgrade() {
            Some(a) => a,
            None => return,
        };

        if app.get_breathe_state() != "ready" {
            return;
        }
        app.set_breathe_state("busy".into());

        // 倒计时
        let countdown = atomic::AtomicI8::new(3);
        let pre_timer_stop = pre_timer.clone();
        let app_weak2 = app.as_weak();
        app.set_breathe_tip(format!("{}", countdown.load(atomic::Ordering::SeqCst)).into());
        pre_timer.start(
            slint::TimerMode::Repeated,
            Duration::from_secs(1),
            move || {
                let count = countdown.load(atomic::Ordering::SeqCst);
                if let Some(app) = app_weak2.upgrade() {
                    if count == 0 {
                        app.set_breathe_tip("Go".into());
                        app.set_breathe_state("prepare".into());
                        app.invoke_start();
                        pre_timer_stop.stop();
                        return;
                    } else {
                        app.set_breathe_tip(format!("{}", count).into());
                    }

                    countdown.store(count - 1, atomic::Ordering::SeqCst);
                }
            },
        );
    });

    let app_weak = app.as_weak();
    let start_timer = Rc::new(slint::Timer::default());
    let start_timer_stop = start_timer.clone();

    app.on_start(move || {
        let app = match app_weak.upgrade() {
            Some(a) => a,
            None => return,
        };

        if app.get_breathe_state() != "prepare" {
            return;
        }
        app.set_breathe_state("start".into());
        let circle_count = atomic::AtomicI32::new(0);
        let mut start_time = time::Instant::now();
        let mut step = 0;
        start_timer.start(TimerMode::Repeated, Duration::from_millis(100), move || {
            let mut left_progress: f32 = 0.0;
            let mut left_time: i32 = 0;
            let mut up_progress: f32 = 0.0;
            let mut up_time: i32 = 0;
            let mut right_progress: f32 = 0.0;
            let mut right_time: i32 = 0;
            let mut down_progress: f32 = 0.0;
            let mut down_time: i32 = 0;

            let now = time::Instant::now();
            let duration = now.duration_since(start_time);
            if step == 0 {
                left_progress = duration.as_secs_f32() / LEFT_SECS.as_secs_f32();
                left_time = duration.as_secs() as i32;
                if left_progress >= 1.0 {
                    step = 1;
                    start_time = time::Instant::now();
                    left_progress = 1.0;
                    left_time = LEFT_SECS.as_secs() as i32;
                }
            } else if step == 1 {
                left_progress = 1.0;
                left_time = LEFT_SECS.as_secs() as i32;
                up_progress = duration.as_secs_f32() / UP_SECS.as_secs_f32();
                up_time = duration.as_secs() as i32;
                if up_progress >= 1.0 {
                    step = 2;
                    start_time = time::Instant::now();
                    up_progress = 1.0;
                    up_time = UP_SECS.as_secs() as i32;
                }
            } else if step == 2 {
                left_progress = 1.0;
                left_time = LEFT_SECS.as_secs() as i32;
                up_progress = 1.0;
                up_time = UP_SECS.as_secs() as i32;
                right_progress = duration.as_secs_f32() / RIIGHT_SECS.as_secs_f32();
                right_time = duration.as_secs() as i32;
                if right_progress >= 1.0 {
                    step = 3;
                    start_time = time::Instant::now();
                    right_progress = 1.0;
                    right_time = RIIGHT_SECS.as_secs() as i32;
                }
            } else if step == 3 {
                left_progress = 1.0;
                left_time = LEFT_SECS.as_secs() as i32;
                up_progress = 1.0;
                up_time = UP_SECS.as_secs() as i32;
                right_progress = 1.0;
                right_time = RIIGHT_SECS.as_secs() as i32;
                down_progress = duration.as_secs_f32() / DOWN_SECS.as_secs_f32();
                down_time = duration.as_secs() as i32;
                if down_progress >= 1.0 {
                    step = 0;
                    start_time = time::Instant::now();
                    down_progress = 1.0;
                    down_time = DOWN_SECS.as_secs() as i32;
                    circle_count.fetch_add(1, atomic::Ordering::SeqCst);
                }
            }

            let data = BreatheData {
                left_progress,
                left_time,
                up_progress,
                up_time,
                right_progress,
                right_time,
                down_progress,
                down_time,
            };
            app.set_breathe_data(data);
            let circle = circle_count.load(atomic::Ordering::SeqCst);
            if circle > 0 {
                app.set_breathe_tip(format!("Circle {}", circle).into());
            }
        });
    });

    let app_weak = app.as_weak();
    app.on_pause(move || {
        let app = match app_weak.upgrade() {
            Some(a) => a,
            None => return,
        };

        if app.get_breathe_state() != "start" {
            return;
        }
        app.set_breathe_state("pause".into());
        app.set_breathe_tip("Paused".into());
        start_timer_stop.stop();
    });

    let app_weak = app.as_weak();
    app.on_stop(move || {
        let app = match app_weak.upgrade() {
            Some(a) => a,
            None => return,
        };

        if app.get_breathe_state() != "pause" {
            return;
        }
        app.set_breathe_state("ready".into());
        app.set_breathe_tip("Ready".into());
        app.set_breathe_data(BreatheData::default());
    });

    app.run().expect("MainWindow run failed");
}

const LEFT_SECS: Duration = Duration::from_secs(4);
const UP_SECS: Duration = Duration::from_secs(2);
const RIIGHT_SECS: Duration = Duration::from_secs(6);
const DOWN_SECS: Duration = Duration::from_secs(2);
