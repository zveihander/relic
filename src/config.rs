use crate::utils::Component;

// COMPONENTS
// NOTE: To use a component, you must first enable it in the "defaults" feature in Cargo.toml
// component            description                argument (example)
//
// datetime             date and time              time format description ([day] [hour]:[minute])
pub const COMPONENTS: &[Component] = &[
    #[cfg(feature = "datetime")]
    Component {
        fmt: "[ %s ]",
        func: crate::components::datetime::datetime,
        arg: Some("[month repr:short] [day] [hour]:[minute]:[second]"),
        interval_s: 1,
    },
];
