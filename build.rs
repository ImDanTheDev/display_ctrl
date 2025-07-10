extern crate winres;

fn main() {
    // Add an icon to the Windows executable.
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon.ico");
        res.compile().unwrap();
    }
}
