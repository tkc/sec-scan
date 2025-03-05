use indicatif::{ProgressBar as IndicatifProgressBar, ProgressStyle};

pub struct ProgressBar {
    pb: IndicatifProgressBar,
}

impl ProgressBar {
    pub fn new(total_files: u64) -> Self {
        let pb = IndicatifProgressBar::new(total_files);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:40.cyan/blue}] {msg}")
                .expect("Failed to set progress bar template")
                .progress_chars("█▓▒░"),
        );
        Self { pb }
    }

    pub fn update(&self) {
        self.pb.inc(1);
    }

    pub fn finish(&self) {
        self.pb.finish_with_message("done");
    }
}
