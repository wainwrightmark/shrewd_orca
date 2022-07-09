use std::rc::Rc;

use crate::core::prelude::*;
use crate::state::{self, prelude::*};
use crate::web::prelude::*;
use itertools::Itertools;
use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {

        <div class="container" style="display: flex; flex-direction: column;">


        <InputBox />
        <ErrorBox />
        <DisplayBox/>

        </div>
    }
}

#[function_component(InputBox)]
pub fn input_box() -> Html {
    let text = use_selector(|state: &InputState| state.text.clone())
        .as_ref()
        .clone();
    let oninput = Dispatch::<InputState>::new().reduce_mut_callback_with(|s, e: InputEvent| {
        let input: HtmlTextAreaElement = e.target_unchecked_into();
        let value = input.value();
        s.change(value);
    });

    html!(


        <div>
            <input type="text" id="textinput" name="input" placeholder="Search" value={text} {oninput}/>
        </div>
    )
}

#[function_component(ErrorBox)]
pub fn error_box() -> Html {
    let err = use_selector(|s: &ResultsState| s.warning.clone())
        .as_ref()
        .clone()
        .unwrap_or_else(|| "‎".to_string());
    html!(<code> {err} </code>)
}

#[function_component(DisplayBox)]
pub fn diplay_box() -> Html {
    let terms = use_selector(|s: &ResultsState| s.data.clone())
        .as_ref()
        .clone();

    log::debug!("Update Display box solved with {} solutions", terms.len());

    let rows = terms.iter().map(|t| row(t)).collect_vec();

    html!(
        <table>
        <tbody>
            {rows}
        </tbody>
        </table>
    )
}

pub fn row(terms: &Vec<Term>) -> Html {
    let spans = terms.iter().map(|t| term_display(t)).collect_vec();

    html!(
        <tr>
            <td>{spans}</td>
        </tr>
    )
}

pub fn term_display(term: &Term) -> Html {
    let text = term.text.to_owned() + " ";
    let definition = term.definition.to_owned();
    html!(
        <span style="border-bottom: none;" data-tooltip={definition}>{text} </span>
    )
}
