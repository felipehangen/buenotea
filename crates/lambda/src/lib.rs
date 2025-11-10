// Lambda utilities and shared code
// Individual Lambda handlers are in src/bin/

pub mod utils {
    use tracing::info;

    pub fn init_tracing() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .with_target(false)
            .without_time()
            .init();
        
        info!("Tracing initialized");
    }
}

