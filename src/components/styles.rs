pub enum ButtonStyle {
    Primary,
    Secondary,
    Danger,
    Neutral,
    Nothing,
}

impl ButtonStyle {
    fn class(&self) -> &str {
        use ButtonStyle::*;
        match self {
            Primary => "bg-cyan-500",
            Secondary => "bg-lime-600",
            Danger => "bg-rose-400",
            Neutral => "bg-zinc-400",
            Nothing => "",
        }
    }
}

pub fn button_class(style: ButtonStyle, custom: &str) -> String {
    format!(
        "{} {} text-blue-50 block rounded p-2 disabled:cursor-not-allowed disabled:bg-slate-600",
        style.class(),
        custom,
    )
}

pub fn text_input_class(custom: &str) -> String {
    format!("{} border rounded border-slate-400 bg-slate-600 px-3", custom)
}

pub fn number_input_class(custom: &str) -> String {
    format!("{} border rounded border-slate-400 bg-slate-600 px-2", custom)
}
