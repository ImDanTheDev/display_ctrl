use std::{fmt::Display, str::FromStr};

use clap::Parser;
use ddc::FeatureCode;
use ddc_hi::{Ddc, Display as DdcDisplay};
use rdev::{EventType, listen};
use serde::Deserialize;

fn main() {
    let args: DisplayCtrlCli = DisplayCtrlCli::parse();

    make_console_window_on_top();

    apply_actions(&args.on_start);

    if args.auto_exit {
        apply_actions(&args.on_quit);
    } else {
        let listener_quit_actions = args.on_quit.clone();
        if let Err(err) = listen(move |e| match e.event_type {
            EventType::KeyPress(_)
            | EventType::KeyRelease(_)
            | EventType::ButtonPress(_)
            | EventType::ButtonRelease(_) => {
                apply_actions(&listener_quit_actions);
                std::process::exit(0);
            }
            _ => {}
        }) {
            eprintln!(
                "Error while listening for global input. Running quit actions now. Error: {err:?}"
            );
            apply_actions(&args.on_quit);
        }
    }
}

fn apply_actions(actions: &[MonitorAction]) {
    for action in actions {
        let mut monitors = DdcDisplay::enumerate();

        if let Some(ref filter) = action.monitor_filter {
            match filter {
                MonitorFilter::Id(id) => {
                    monitors.retain(|mon| mon.info.id == *id);
                }
                MonitorFilter::ModelId(id) => monitors.retain(|mon| mon.info.model_id == *id),
                MonitorFilter::ModelName(name) => {
                    monitors.retain(|mon| mon.info.model_name == *name)
                }
                MonitorFilter::Serial(serial) => monitors.retain(|mon| mon.info.serial == *serial),
                MonitorFilter::SerialNumber(serial) => {
                    monitors.retain(|mon| mon.info.serial_number == *serial)
                }
            }
        }

        if monitors.is_empty() {
            println!("No displays matched filter: {:?}", action.monitor_filter);
        }

        for mon in &mut monitors {
            if let Err(err) = mon.handle.set_vcp_feature(action.code, action.value) {
                eprintln!("Failed to run action. Display: {}. Error: {err}", mon.info)
            }
        }
    }
}

#[derive(Debug, Parser)]
struct DisplayCtrlCli {
    #[clap(long, short, action)]
    auto_exit: bool,
    #[clap(long, default_values_t = Vec::<MonitorAction>::new())]
    on_start: Vec<MonitorAction>,
    #[clap(long, default_values_t = Vec::<MonitorAction>::new())]
    on_quit: Vec<MonitorAction>,
}

#[derive(Debug, Clone, Deserialize)]
struct MonitorAction {
    monitor_filter: Option<MonitorFilter>,
    code: FeatureCode,
    value: u16,
}

impl FromStr for MonitorAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: MonitorAction =
            serde_json::from_str(s).map_err(|e| format!("error parsing monitor action: {e}"))?;
        Ok(res)
    }
}

impl Display for MonitorAction {
    fn fmt(&self, mut f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(&mut f, "{self:?}")
    }
}

#[derive(Debug, Clone, Deserialize)]
enum MonitorFilter {
    Id(String),
    ModelId(Option<u16>),
    ModelName(Option<String>),
    Serial(Option<u32>),
    SerialNumber(Option<String>),
}

fn make_console_window_on_top() {
    #[cfg(target_os = "windows")]
    use windows::Win32::{
        System::Console::GetConsoleWindow,
        UI::WindowsAndMessaging::{HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE, SetWindowPos},
    };

    #[cfg(target_os = "windows")]
    unsafe {
        // Get the console window handle
        let hwnd = GetConsoleWindow();
        if hwnd.is_invalid() {
            eprintln!("Failed to get console window handle.");
            return;
        } else {
            // Set the window to be always on top
            if let Err(err) = SetWindowPos(
                hwnd,
                Some(HWND_TOPMOST),
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE,
            ) {
                eprintln!("Failed to make console window always on top. Error: {err}");
                return;
            }
        }
    }
}
