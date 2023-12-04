use std::sync::{Arc, Mutex};

use crate::main::order::providers::order_exporter::OrderExporter;

pub struct ExportOrderAfterCheckoutRule {
    pub order_exporter: Arc<Mutex<dyn OrderExporter>>,
}
