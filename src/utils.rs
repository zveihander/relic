pub struct Component {
    pub fmt: &'static str,
    pub func: fn(&'static str) -> String,
    pub arg: Option<&'static str>,
    pub interval_s: u64,
}
