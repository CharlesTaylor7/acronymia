pub fn button_class(custom: &str) -> String {
    format!(
        "{} border rounded p-2 border-slate-200 disabled:cursor-not-allowed disabled:bg-slate-200",
        custom
    )
}

pub fn text_input_class() -> String{
    "border rounded border-slate-400 px-3".to_owned()
}
