fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/pro-clock.ico");
        res.set("ProductName", "Pro Clock");
        res.set("FileDescription", "Pro Clock — Offline Time Tracker");
        res.set("LegalCopyright", "MIT License — Pro Clock Contributors");
        if let Err(e) = res.compile() {
            eprintln!("cargo:warning=Failed to embed Windows resources: {}", e);
        }
    }
}
