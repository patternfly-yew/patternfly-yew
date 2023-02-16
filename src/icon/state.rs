pub enum State {
    None,
    Danger,
    Default,
    Info,
    Success,
    Warning,
    Disabled,
}

impl State {
    fn var(name: &str, weight: usize) -> String {
        format!("--pf-global--{}-color--{}", name, weight)
    }

    pub fn as_var(&self, weight: usize) -> Option<String> {
        match self {
            Self::None => None,
            Self::Danger => Some(Self::var("danger", weight)),
            Self::Default => Some(Self::var("default", weight)),
            Self::Info => Some(Self::var("info", weight)),
            Self::Success => Some(Self::var("success", weight)),
            Self::Warning => Some(Self::var("warning", weight)),
            Self::Disabled => Some(Self::var("disabled", weight)),
        }
    }
}
