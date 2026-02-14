#[cfg(feature = "datetime")]
pub mod datetime;

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
    feature = "ram_used"
))]
pub mod ram;
