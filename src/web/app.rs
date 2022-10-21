use crate::core::prelude::*;
use crate::state::prelude::*;
use itertools::Itertools;
use shrewd_orca::language::prelude::Example;
use web_sys::{HtmlSelectElement, HtmlTextAreaElement};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {

        <div class="container" style="display: flex; flex-direction: column;">

        <Examples />
        <InputBox />

        <ErrorBox />
        <DisplayBox/>
        <LoadMoreButton/>
        </div>
    }
}

#[function_component(InputBox)]
pub fn input_box() -> Html {
    let text = use_selector(|state: &FullState| state.text.clone())
        .as_ref()
        .clone();
    let oninput = Dispatch::<FullState>::new().reduce_mut_callback_with(|s, e: InputEvent| {
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
    let err = use_selector(|s: &FullState| s.warning.clone())
        .as_ref()
        .clone()
        .unwrap_or_else(|| "‎".to_string());
    html!(<code> {err} </code>)
}

#[function_component(DisplayBox)]
pub fn diplay_box() -> Html {
    let terms = use_selector(|s: &FullState| s.data.clone())
        .as_ref()
        .clone();

    let rows = terms.iter().map(row).collect_vec();

    html!(
        <table>
        <tbody>
            {rows}
        </tbody>
        </table>
    )
}

#[function_component(Examples)]
pub fn examples_dropdown() -> Html {
    let onchange = Dispatch::<FullState>::new().reduce_mut_callback_with(|s, e: Event| {
        let input: HtmlSelectElement = e.target_unchecked_into();
        let value = input.value();
        s.change(value);
    });

    let options = Example::list()
        .into_iter()
        .map(|example| {
            html!(  <option value={example.text}>{example.description}</option>
            )
        })
        .collect_vec();

    html!(
        <select {onchange}>
        <option value="" disabled={true} selected={true}>{"Examples"}</option>

            {options}
        </select>
    )
}

#[function_component(LoadMoreButton)]
pub fn load_more_button() -> Html {
    let onclick =
        Dispatch::<FullState>::new().reduce_mut_callback_with(|s, _: MouseEvent| s.load_more());

    let total_results = use_selector(|s: &FullState| s.data.len());
    let max_results = use_selector(|s: &FullState| s.max_solutions);
    let disabled = total_results < max_results;

    html!(<button {onclick} {disabled}>{"Load More"}</button>)
}

pub fn row(solution: &QuestionSolution) -> Html {
    match solution {
        QuestionSolution::Expression(expression) => {
            let spans = expression
                .homographs
                .iter()
                .map(homograph_display)
                .collect_vec();

            html!(
                <tr>
                    <td>{spans}</td>
                </tr>
            )
        }
        QuestionSolution::Anagram(anagram) => {
            let left_spans = anagram
                .left
                .homographs
                .iter()
                .map(homograph_display)
                .collect_vec();
            let right_spans = anagram
                .right
                .homographs
                .iter()
                .map(homograph_display)
                .collect_vec();

            html!(
                <tr>
                    <td>{left_spans}</td>
                    <td>{right_spans}</td>
                </tr>
            )
        }
        QuestionSolution::Spoonerism(spoonerism) => {
            let left_spans = spoonerism
                .left
                .homographs
                .iter()
                .map(homograph_display)
                .collect_vec();
            let right_spans = spoonerism
                .right
                .homographs
                .iter()
                .map(homograph_display)
                .collect_vec();

            html!(
                <tr>
                    <td>{left_spans}</td>
                    <td>{right_spans}</td>
                </tr>
            )
        }
    }
}

fn homograph_display(homograph: &Homograph) -> Html {
    let text = homograph.text.to_owned() + " ";

    if let Some(definition) = homograph
        .meanings
        .iter()
        .filter_map(|x| x.definition)
        .next()
    {
        html!(
            <span style="border-bottom: none;" data-tooltip={definition}>{text} </span>
        )
    } else {
        html!(
            <span style="border-bottom: none;" >{text} </span>
        )
    }
}
