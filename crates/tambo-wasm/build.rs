use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let font_path = Path::new(&out_dir).join("Inter-Regular.ttf");

    if !font_path.exists() {
        let url = "https://fonts.gstatic.com/s/inter/v20/UcCO3FwrK3iLTeHuS_nVMrMxCp50SjIw2boKoduKmMEVuLyfMZg.ttf";
        let status = Command::new("curl")
            .args(["-sL", "-o", &font_path.to_string_lossy(), url])
            .status()
            .expect("curl failed");
        assert!(status.success(), "Failed to download Inter font");
    }

    println!("cargo::rerun-if-changed=build.rs");
}
