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
        <LoadMoreButton/>
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

    let rows = terms.iter().map(row).collect_vec();

    html!(
        <table>
        <tbody>
            {rows}
        </tbody>
        </table>
    )
}

#[function_component(LoadMoreButton)]
pub fn load_more_button()->Html{

    let onclick = Dispatch::<InputState>::new().reduce_mut_callback_with(|s, e: MouseEvent| {        
        s.load_more()
    });

    let total_results = use_selector(|s : &ResultsState| s.data.len());
    let max_results = use_selector(|s: &InputState| s.max_solutions);
    let disabled = total_results < max_results;

 html!(<button {onclick} {disabled}>{"Load More"}</button>)
}

pub fn row(solution: &QuestionSolution) -> Html {
    
    match solution{
        QuestionSolution::Expression(expression) =>{
            let spans = expression.homographs .iter().map(homograph_display).collect_vec();

    html!(
        <tr>
            <td>{spans}</td>
        </tr>
    )
        },
        QuestionSolution::Anagram(anagram) =>{
            let left_spans = anagram.left.homographs .iter().map(homograph_display).collect_vec();
            let right_spans = anagram.right.homographs .iter().map(homograph_display).collect_vec();

    html!(
        <tr>
            <td>{left_spans}</td>
            <td>{right_spans}</td>
        </tr>
    )
        },
    }

    
}

fn homograph_display(homograph: &Homograph) -> Html {
    let text = homograph.text.to_owned() + " ";

    if let Some(definition) = homograph.meanings.first().map(|x|x.definition.clone()){
        html!(
            <span style="border-bottom: none;" data-tooltip={definition}>{text} </span>
        )
    }
    else{
        html!(
            <span style="border-bottom: none;" >{text} </span>
        )
    }    
}
