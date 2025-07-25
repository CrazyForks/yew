use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
struct Props {
    a: usize,
}

#[component(Comp)]
fn comp<'a>(props: &'a Props) -> Html {
    html! {
        <p>
            { props.a }
        </p>
    }
}

fn main() {}
