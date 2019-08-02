use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct SignalDataIntern {
    ctrlc: AtomicBool,
    ctrlc_armed: AtomicBool,
}
pub type SignalData = Arc<SignalDataIntern>;

impl SignalDataIntern {
    pub fn new() -> Self {
        Self {
            ctrlc: AtomicBool::new(false),
            ctrlc_armed: AtomicBool::new(true),
        }
    }
    pub fn got_ctrlc(&self) -> bool {
        self.ctrlc.load(Ordering::SeqCst)
    }
    pub fn handle_ctrlc(&self) {
        if self.is_ctrlc_armed() {
            std::process::exit(128 + signal_hook::SIGINT);
        } else {
            self.ctrlc.store(true, Ordering::SeqCst);
        }
    }
    pub fn is_ctrlc_armed(&self) -> bool {
        self.ctrlc_armed.load(Ordering::SeqCst)
    }
    pub fn set_ctrlc_armed(&self, val: bool) {
        self.ctrlc_armed.store(val, Ordering::SeqCst);
        self.ctrlc.store(false, Ordering::SeqCst);
    }
}

pub fn register_signal_handlers(dat: SignalData) {
    unsafe {
        signal_hook::register(signal_hook::SIGINT, move || dat.handle_ctrlc())
    }
    .or_else(|e| {
        warn!("Failed to register for SIGINT {:?}", e);
        Err(e)
    })
    .ok();
}
