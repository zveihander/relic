#[cfg(feature = "datetime")]
pub mod datetime;

#[cfg(any(feature = "wifi_essid", feature = "wifi_perc", feature = "wifi_custom"))]
pub mod wifi;

#[cfg(any(
    feature = "battery_perc",
    feature = "battery_state",
    feature = "battery_custom"
))]
pub mod battery;

#[cfg(feature = "cpu_perc")]
pub mod cpu;

#[cfg(any(
    feature = "ram_free",
    feature = "ram_perc",
    feature = "ram_total",
    feature = "ram_used",
    feature = "swap_free",
    feature = "swap_perc",
    feature = "swap_total",
    feature = "swap_used"
))]
pub mod ram;

#[cfg(any(
    feature = "disk_free",
    feature = "disk_perc",
    feature = "disk_total",
    feature = "disk_used"
))]
pub mod disk;

#[cfg(any(
    feature = "hostname",
    feature = "username",
    feature = "userathost",
    feature = "kernel",
    feature = "updates"
))]
pub mod system;

#[cfg(any(feature = "temperature_c", feature = "temperature_f"))]
pub mod temperature;

#[cfg(any(feature = "pipewire", feature = "pipewire_icon",))]
pub mod audio;
