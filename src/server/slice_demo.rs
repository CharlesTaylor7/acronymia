use ::leptos::*;
use ::macros::slice::*;

#[derive(Sliceable)]
pub struct Person {
    pub age: usize,
    pub name: String,
}

pub fn demo_struct(demo: RwSignal<Person>, cx: Scope) {
    let sliced: Person_Sliced = demo.slice(cx);
    let _age: usize = sliced.age.with(|a| a.clone());
    // set name
    sliced.name.set("foobar".to_owned());
}
