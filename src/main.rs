use ddc_hi::{Ddc, Display};
use rdev::{EventType, listen};
use windows::Win32::{
    System::Console::GetConsoleWindow,
    UI::WindowsAndMessaging::{HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE, SetWindowPos},
};
fn main() {
    unsafe {
        // Get the console window handle
        let hwnd = GetConsoleWindow();
        if !hwnd.is_invalid() {
            // Set the window to be always on top
            SetWindowPos(
                hwnd,
                Some(HWND_TOPMOST),
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE,
            )
            .unwrap();
        } else {
            eprintln!("Failed to get console window handle.");
        }
    }

    let mut dell = None;
    let mut onn = None;
    for mut display in Display::enumerate() {
        display.update_capabilities().unwrap_or_default();

        // Turn off
        if display
            .info
            .model_name
            .clone()
            .is_some_and(|x| x.contains("S2415H"))
        {
            dell = Some(display);
        } else {
            onn = Some(display);
        }
    }

    // Turn Off
    if let Some(ref mut dell) = dell {
        dell.handle.set_vcp_feature(0x10, 0).unwrap();
        dell.handle.set_vcp_feature(0x14, 12).unwrap();
    }

    if let Some(ref mut onn) = onn {
        onn.handle.set_vcp_feature(0xD6, 5).unwrap();
    }

    if let Err(error) = listen(move |e| match e.event_type {
        EventType::KeyPress(_)
        | EventType::KeyRelease(_)
        | EventType::ButtonPress(_)
        | EventType::ButtonRelease(_) => {
            let dell = dell.take();
            if let Some(mut dell) = dell {
                dell.handle.set_vcp_feature(0x10, 100).unwrap();
                dell.handle.set_vcp_feature(0x14, 5).unwrap();
            }

            let onn = onn.take();
            if let Some(mut onn) = onn {
                onn.handle.set_vcp_feature(0xD6, 1).unwrap();
            }

            std::process::exit(0);
        }
        _ => {}
    }) {
        println!("Error: {:?}", error)
    }
}
