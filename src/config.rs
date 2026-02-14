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
pub const COMPONENTS: &[Component] = &[
    #[cfg(feature = "ram_used")]
    Component {
        fmt: "[ Used RAM %s GB ]",
        func: crate::components::ram::ram_used,
        arg: None,
        interval_s: 5,
    },
    #[cfg(feature = "ram_total")]
    Component {
        fmt: "[ Total Ram %s GB ]",
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
        fmt: "[Free RAM %s MB]",
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
    #[cfg(feature = "datetime")]
    Component {
        fmt: "[ %s ]",
        func: crate::components::datetime::datetime,
        arg: Some("[month repr:short] [day] [hour]:[minute]:[second]"),
        interval_s: 1,
    },
];
