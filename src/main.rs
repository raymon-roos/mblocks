use signal_hook::{
    consts::{SIGHUP, SIGINT, SIGTERM},
    iterator::Signals,
};
use std::{
    ffi::CString,
    ptr,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
    time::Duration,
};
use x11::xlib;

#[macro_use(concat_string)]
extern crate concat_string;

mod block;
mod blocks;
mod config;

const SIGRTMIN: i32 = 34;

fn await_signals(tx: Sender<i32>, signums: &[i32]) {
    let mut signals = Signals::new(signums).unwrap();
    for signal in signals.forever() {
        tx.send(signal).unwrap();
        if signal == SIGTERM || signal == SIGINT || signal == SIGHUP {
            break;
        }
    }
}

fn main() {
    let (tx, rx): (
        Sender<(usize, Option<String>)>,
        Receiver<(usize, Option<String>)>,
    ) = channel();
    let mut handles = vec![];
    let mut outputs = vec![String::from(""); config::BLOCKS.len()];
    let display = unsafe { xlib::XOpenDisplay(ptr::null()) };
    let window = unsafe { xlib::XDefaultRootWindow(display) };

    // fire threads
    for (i, b) in config::BLOCKS.iter().enumerate() {
        let tx_clone = tx.clone();
        match b.kind {
            block::BlockType::Once => {
                let handle = thread::spawn(move || {
                    let msg = b.execute();
                    tx_clone.send((i, msg)).unwrap();
                });
                handles.push(handle);
            }
            block::BlockType::Periodic(t) => {
                let (tx_signals, rx_signals): (Sender<i32>, Receiver<i32>) = channel();
                handles.push(thread::spawn(move || {
                    await_signals(tx_signals, &[SIGTERM, SIGINT, SIGHUP]);
                }));
                let handle = thread::spawn(move || loop {
                    let msg = b.execute();
                    tx_clone.send((i, msg)).unwrap();
                    if let Ok(signal) = rx_signals.recv_timeout(Duration::from_secs(t)) {
                        if signal == SIGTERM || signal == SIGINT || signal == SIGHUP {
                            tx_clone.send((usize::MAX, None)).unwrap();
                            break;
                        }
                    }
                });
                handles.push(handle);
            }
            block::BlockType::Signal(s) => {
                if s < 1 || s > 15 {
                    tx_clone.send((i, None)).unwrap();
                    continue;
                }
                let msg = b.execute();
                tx_clone.send((i, msg)).unwrap();
                let (tx_signals, rx_signals): (Sender<i32>, Receiver<i32>) = channel();
                let _signum = SIGRTMIN + s;
                handles.push(thread::spawn(move || {
                    await_signals(tx_signals, &[SIGTERM, SIGINT, SIGHUP, _signum]);
                }));
                let handle = thread::spawn(move || {
                    while let Ok(signal) = rx_signals.recv() {
                        match signal {
                            SIGTERM | SIGINT | SIGHUP => {
                                tx_clone.send((usize::MAX, None)).unwrap();
                                break;
                            }
                            _signum => {
                                let msg = b.execute();
                                tx_clone.send((i, msg)).unwrap();
                            }
                        }
                    }
                });
                handles.push(handle);
            }
            block::BlockType::PeriodicOrSignal(t, s) => {
                if s < 1 || s > 15 {
                    tx_clone.send((i, None)).unwrap();
                    continue;
                }
                let msg = b.execute();
                tx_clone.send((i, msg)).unwrap();
                let _signum = SIGRTMIN + s;
                let tx_clone_signal = tx_clone.clone();
                let (tx_signals_signal, rx_signals_signal): (Sender<i32>, Receiver<i32>) = channel();
                handles.push(thread::spawn(move || {
                    await_signals(tx_signals_signal, &[SIGTERM, SIGINT, SIGHUP, _signum]);
                }));
                let handle = thread::spawn(move || {
                    while let Ok(signal) = rx_signals_signal.recv() {
                        match signal {
                            SIGTERM | SIGINT | SIGHUP => {
                                tx_clone_signal.send((usize::MAX, None)).unwrap();
                                break;
                            }
                            _signum => {
                                let msg = b.execute();
                                tx_clone_signal.send((i, msg)).unwrap();
                            }
                        }
                    }
                });
                handles.push(handle);
                let tx_clone_periodic = tx_clone.clone();
                let (tx_signals_periodic, rx_signals_periodic): (Sender<i32>, Receiver<i32>) = channel();
                handles.push(thread::spawn(move || {
                    await_signals(tx_signals_periodic, &[SIGTERM, SIGINT, SIGHUP]);
                }));
                let handle = thread::spawn(move || loop {
                    let msg = b.execute();
                    tx_clone_periodic.send((i, msg)).unwrap();
                    if let Ok(signal) = rx_signals_periodic.recv_timeout(Duration::from_secs(t)) {
                        if signal == SIGTERM || signal == SIGINT || signal == SIGHUP {
                            tx_clone_periodic.send((usize::MAX, None)).unwrap();
                            break;
                        }
                    }
                });
                handles.push(handle);
            }
        }
    }

    // update status if a block change occurs
    drop(tx);
    while let Ok((i, msg)) = rx.recv() {
        if i == usize::MAX {
            break;
        }
        let msg = msg.unwrap_or("failed".to_string());
        if outputs[i] == msg {
            continue;
        }
        outputs[i] = msg;
        let status = block::infer_status(&outputs);
        let c_str = CString::new(status.as_str()).expect("panic caused by cstring");
        let str_ptr = c_str.as_ptr() as *const i8;
        unsafe {
            xlib::XStoreName(display, window, str_ptr);
            xlib::XSync(display, 0);
        }
    }

    // graceful termination of threads
    for handle in handles {
        let _ = handle.join();
    }

    // cleanup of the status and close the open display
    let c_str = CString::new("").unwrap();
    let str_ptr = c_str.as_ptr() as *const i8;
    unsafe {
        xlib::XStoreName(display, window, str_ptr);
        xlib::XSync(display, 0);
        xlib::XCloseDisplay(display);
    }
}
