use crate::core::prelude::*;
use crate::state::prelude::*;
use itertools::Itertools;

use shrewd_orca::language::prelude::Example;
use web_sys::{HtmlSelectElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew_hooks::{use_debounce, use_infinite_scroll};
use yewdux::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {

        <div class="container" style="display: flex; flex-direction: column; overflow-y: none;" >

        <Examples />
        <InputBox />

        <ErrorBox />
        <DisplayBox/>
        // <LoadMoreButton/>
        </div>
    }
}

#[function_component(InputBox)]
pub fn input_box() -> Html {
    let value = use_selector(|state: &FullState| state.text.clone());

    let debounce = use_debounce(
        move || Dispatch::<FullState>::new().reduce_mut(|s| s.update_if_hot()),
        500,
    );

    let oninput = Callback::from(move |e: InputEvent| {
        let input: HtmlTextAreaElement = e.target_unchecked_into();
        Dispatch::<FullState>::new().reduce_mut(|x| x.change_text(input.value()));
        debounce.run();
    });

    html!(
        <div>
            <input type="text" id="textinput" name="input" placeholder="Search" value={value.to_string()} {oninput}/>
        </div>
    )
}

#[function_component(ErrorBox)]
pub fn error_box() -> Html {
    let err = use_selector(|s: &FullState| s.info_text());
    html!(<code> {err} </code>)
}

#[function_component(DisplayBox)]
pub fn display_box() -> Html {
    let node = use_node_ref();

    use_infinite_scroll(node.clone(), || {
        Dispatch::<FullState>::new().reduce_mut(|x| x.load_more());
    });

    let selected = use_selector(|s: &FullState| (s.data.clone(), s.is_complete));

    let rows = selected.0.iter().map(row).collect_vec();

    html!(
        <div style="height: 75vh; overflow-y: scroll; overflow-x: hidden;" ref={node}>
        <div>
        <table >
        <tbody>
            {rows}
        </tbody>
        </table>
        </div>
        {if !selected.as_ref().1{
            html!(<div style="height: 40vh; width: 100%; background: none;"></div>)
        }else{
            html!(<></>)
        }}


        </div>
    )
}

#[function_component(Examples)]
pub fn examples_dropdown() -> Html {
    let onchange = Dispatch::<FullState>::new().reduce_mut_callback_with(|s, e: Event| {
        let input: HtmlSelectElement = e.target_unchecked_into();
        let value = input.value();
        s.change_text(value);
        s.update_if_hot();
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

pub fn row(solution: &QuestionSolution) -> Html {
    match solution {
        QuestionSolution::Expression(expression) => {
            let spans = expression
                .homographs
                .iter()
                .map(|x| homograph_display(x, "right"))
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
                .map(|x| homograph_display(x, "right"))
                .collect_vec();
            let right_spans = anagram
                .right
                .homographs
                .iter()
                .map(|x| homograph_display(x, "left"))
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
                .map(|x| homograph_display(x, "right"))
                .collect_vec();
            let right_spans = spoonerism
                .right
                .homographs
                .iter()
                .map(|x| homograph_display(x, "left"))
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

fn homograph_display(homograph: &Homograph, tooltip_placement: &'static str) -> Html {
    let text = homograph.text.to_string() + " ";
    let definition = homograph.first_definition();

    html!(
        <span style="border-bottom: none;" data-tooltip={definition} data-placement={tooltip_placement}>{text} </span>
    )
}
