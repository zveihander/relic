use crate::utils::Component;

// COMPONENTS
// NOTE: To use a component, you must first enable it in the "defaults" feature in Cargo.toml
// component          description                argument (example)
//
// datetime           date and time              time format description ([day] [hour]:[minute]). Yes, I know it's weird.
// battery_perc       battery percentage         battery device name (usually BAT0)
// battery_state      battery status             battery device name (usually BAT0)
// battery_custom     custom battery with icons  battery device name (usually BAT0)
// cpu_perc           CPU usage percentage       none
// ram_free           Free memory in MB          none
// ram_perc           Used memory as percentage  none
// ram_total          Total memory in GB         none
// ram_used           Used memory in GB          none
// swap_free          Free swap in MB            none
// swap_perc          Used swap as percentage    none
// swap_total         Total swap in GB           none
// swap_used          Used swap in GB            none
// disk_free          Free space in GB           path (default is '/')
// disk_perc          Used space as percentage   path (default is '/')
// disk_total         Total space in GB          path (default is '/')
// disk_free          Free space in GB           path (default is '/')
// hostname           System hostname            none
// username           current user's name        none
// userathost         username@hostname          none

pub const COMPONENTS: &[Component] = &[
    #[cfg(feature = "updates")]
    Component {
        fmt: "[   %s ]",
        func: crate::components::system::updates,
        arg: Some("xbps"),
        interval_s: 1800, // NOTE: A high interval is recommended as package queries can be quite expensive.
    },
    #[cfg(feature = "userathost")]
    Component {
        fmt: "[ %s ]",
        func: crate::components::system::userathost,
        arg: None,
        interval_s: 3600,
    },
    #[cfg(feature = "username")]
    Component {
        fmt: "[ %s ]",
        func: crate::components::system::username,
        arg: None,
        interval_s: 3600,
    },
    #[cfg(feature = "hostname")]
    Component {
        fmt: "[ %s ]",
        func: crate::components::system::hostname,
        arg: None,
        interval_s: 3600,
    },
    #[cfg(feature = "disk_used")]
    Component {
        fmt: "[   %s GB Used ]",
        func: crate::components::disk::disk_used,
        arg: Some("/"),
        interval_s: 60,
    },
    #[cfg(feature = "disk_total")]
    Component {
        fmt: "[   %s GB Total ]",
        func: crate::components::disk::disk_total,
        arg: Some("/"),
        interval_s: 60,
    },
    #[cfg(feature = "disk_perc")]
    Component {
        fmt: "[   %s% Used ]",
        func: crate::components::disk::disk_perc,
        arg: Some("/"),
        interval_s: 60,
    },
    #[cfg(feature = "disk_free")]
    Component {
        fmt: "[   %s GB Free ]",
        func: crate::components::disk::disk_free,
        arg: Some("/"),
        interval_s: 60,
    },
    #[cfg(feature = "swap_used")]
    Component {
        fmt: "[ Swap %s GB Used ]",
        func: crate::components::ram::swap_used,
        arg: None,
        interval_s: 5,
    },
    #[cfg(feature = "swap_total")]
    Component {
        fmt: "[ Swap %s GB Total ]",
        func: crate::components::ram::swap_total,
        arg: None,
        interval_s: 5,
    },
    #[cfg(feature = "swap_perc")]
    Component {
        fmt: "[ Swap %s% ]",
        func: crate::components::ram::swap_perc,
        arg: None,
        interval_s: 5,
    },
    #[cfg(feature = "swap_free")]
    Component {
        fmt: "[ Swap %s MB Free ]",
        func: crate::components::ram::swap_free,
        arg: None,
        interval_s: 5,
    },
    #[cfg(feature = "ram_used")]
    Component {
        fmt: "[ RAM %s GB Used ]",
        func: crate::components::ram::ram_used,
        arg: None,
        interval_s: 5,
    },
    #[cfg(feature = "ram_total")]
    Component {
        fmt: "[ Ram %s GB Total ]",
        func: crate::components::ram::ram_total,
        arg: None,
        interval_s: 5,
    },
    #[cfg(feature = "ram_perc")]
    Component {
        fmt: "[ RAM %s% ]",
        func: crate::components::ram::ram_perc,
        arg: None,
        interval_s: 5,
    },
    #[cfg(feature = "ram_free")]
    Component {
        fmt: "[ RAM %s MB Free ]",
        func: crate::components::ram::ram_free,
        arg: None,
        interval_s: 5,
    },
    #[cfg(feature = "cpu_perc")]
    Component {
        fmt: "[ CPU %s% ]",
        func: crate::components::cpu::cpu_perc,
        arg: None,
        interval_s: 2, // NOTE: For this component, a low interval is required due to the way CPU usage is calculated.
    },
    #[cfg(feature = "battery_custom")]
    Component {
        fmt: "[ %s ]",
        func: crate::components::battery::battery_custom,
        arg: Some("macsmc-battery"),
        interval_s: 15,
    },
    #[cfg(feature = "battery_state")]
    Component {
        fmt: "[ %s",
        func: crate::components::battery::battery_state,
        arg: Some("BAT0"),
        interval_s: 15,
    },
    #[cfg(feature = "battery_perc")]
    Component {
        fmt: "%s% ]",
        func: crate::components::battery::battery_perc,
        arg: Some("BAT0"),
        interval_s: 30,
    },
    #[cfg(feature = "wifi_custom")]
    Component {
        fmt: "[ %s ]",
        func: crate::components::wifi::wifi_custom,
        arg: Some("wlp1s0f0"),
        interval_s: 5,
    },
    #[cfg(feature = "wifi_perc")]
    Component {
        fmt: "[ %s ]",
        func: crate::components::wifi::wifi_perc,
        arg: Some("wlp1s0f0"),
        interval_s: 5,
    },
    #[cfg(feature = "wifi_essid")]
    Component {
        fmt: "[ %s ]",
        func: crate::components::wifi::wifi_essid,
        arg: Some("wlp1s0f0"),
        interval_s: 5,
    },
    #[cfg(feature = "datetime")]
    Component {
        fmt: "[ %s ]",
        func: crate::components::datetime::datetime,
        arg: Some("[month repr:short] [day] [hour]:[minute]:[second]"),
        interval_s: 1,
    },
];
