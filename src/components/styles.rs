pub fn button_class(custom: &str) -> String {
    format!(
        "{} border rounded p-2 border-slate-200 disabled:cursor-not-allowed disabled:bg-slate-200",
        custom
    )
}
