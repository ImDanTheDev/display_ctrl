# Display CTRL

<img src="./icon.ico" height="100" align="left">

Display CTRL is a CLI utility that allows you to control your displays using the [DDC/CI](https://en.wikipedia.org/wiki/Display_Data_Channel#DDC/CI) protocol that's available on many(but not all) computer displays.
<br/>
<br/>

> [!NOTE]
> Use a utility like [ControlMyMonitor](https://www.nirsoft.net/utils/control_my_monitor.html) to see which DDC/CI codes your display supports, their default values, and to experiment with values.

# Usage
You can download the executable from the releases page. Then, run `display_ctrl --help` to see what it can do.

```
$ display_ctrl --help
A lightweight CLI utility for controlling displays using the DDC/CI protocol.
Multiple on-start and on-quit actions can be provided by specifying the flag multiple times.

Usage: display_ctrl [OPTIONS]

Options:
  -a, --auto-exit            Waits for a global key or button press before running `on_quit` actions and exiting
  -s, --on-start <ON_START>  JSON-formatted DDC/CI action to run when the program starts
  -q, --on-quit <ON_QUIT>    JSON-formatted DDC/CI action to run before the program exits
  -h, --help                 Print help

Example:
  Dims the backlight of a specific monitor on start, then brightens it when program exits.
  $ display_ctr --on-start '{"monitor_filter": {"ModelName": "100140682"}, "code": 16, "value": 10}' --on-quit '{"monitor_filter":{"ModelName": "100140682"}, "code": 16, "value": 100}'
```