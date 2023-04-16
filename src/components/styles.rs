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
            Primary => "bg-cyan-500 text-blue-50",
            Secondary => "bg-lime-600 text-blue-50",
            Danger => "bg-rose-400 text-blue-50",
            Neutral => "bg-zinc-400 text-stone-200",
            Nothing => "",
        }
    }
}

pub fn button_class(style: ButtonStyle, custom: &str) -> String {
    format!(
        "{} {} rounded p-2 disabled:cursor-not-allowed disabled:bg-slate-600",
        style.class(),
        custom,
    )
}

pub fn text_input_class(custom: &str) -> String {
    format!(
        "{} border rounded border-slate-400 bg-slate-600 px-3",
        custom
    )
}

pub fn number_input_class(custom: &str) -> String {
    format!(
        "{} border rounded border-slate-400 bg-slate-600 px-2",
        custom
    )
}

pub fn judge_class() -> &'static str {
    "font-bold text-amber-600"
}

pub fn counter_class() -> &'static str {
    //"text-cyan-700"
    "text-lime-100"
}
