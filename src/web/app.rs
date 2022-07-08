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

        <div class="paper container margin-bottom-large" style="display: flex; flex-direction: column;">


            <DisplayBox/>
            <ErrorBox />
            <details>
            <summary>{"Code"}</summary>
            <NameBox />
            <InputBox />
            </details>
            <details>
            <summary>{"Variables"}</summary>
            <SlidersControl/>
            </details>
            <details>
            <summary>{"Settings"}</summary>
            <SettingsControl/>
            </details>


        </div>
    }
}

#[function_component(ExamplesSelect)]
pub fn examples_select() -> Html {
    let oninput = Dispatch::<InputState>::new().reduce_mut_callback_with(|s, e: InputEvent| {
        let input: HtmlSelectElement = e.target_unchecked_into();
        let value = input.value();
        s.use_creation(value);
    });

    let creations = use_store_value::<SavedCreationsState>();
    let chosen = use_selector(|state: &InputState| state.name.clone())
        .as_ref()
        .clone();

    let options = creations.creations.values().map(|e| {
        let selected = e.name == chosen;
        html!(<option {selected} value={e.name.clone()}>{e.name.clone()} </option>)
    });

    html!(
    <select {oninput}>
    {for options}
    </select>

        )
}

#[function_component(SettingsControl)]
pub fn settings_control() -> Html {
    let settings = *use_selector(|state: &InputState| state.settings).as_ref();

    let on_max_nodes_input =
        Dispatch::<InputState>::new().reduce_mut_callback_with(move |s, e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let new_value = input.value();
            let new_u_value: usize = new_value.parse().unwrap();
            let new_settings = ExpandSettings {
                max_nodes: new_u_value,
                ..settings
            };
            s.update_settings(new_settings);
        });

    let on_max_depth_input =
        Dispatch::<InputState>::new().reduce_mut_callback_with(move |s, e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let new_value = input.value();
            let new_u_value: usize = new_value.parse().unwrap();
            let new_settings = ExpandSettings {
                max_depth: new_u_value,
                ..settings
            };
            s.update_settings(new_settings);
        });

    html!(
        <>
        <div class="slider">
                    <code style="width:80px" >{"Max Nodes"}</code>
                    <input style="width:80px" oninput={on_max_nodes_input} type="number"  value={format!("{}",settings.max_nodes )} min={100} max={10000}  step={100} />
                </div>
                <div class="slider">
                    <code style="width:80px" >{"Max Depth"}</code>
                    <input style="width:80px" oninput={on_max_depth_input} type="number"  value={format!("{}",settings.max_depth )} min={4} max={40}  step={1} />
                </div>
                </>


    )
}

#[function_component(SlidersControl)]
pub fn sliders_control() -> Html {
    let properties = use_selector(|state: &InputState| state.grammar.get_variables());

    let result = properties.iter().map(|p| {
        let prop_key = p.0.clone();
        let p_type = p.1;

        html!(<InputSlider  {p_type} {prop_key} />)
    });

    let onclick = Dispatch::<InputState>::new().reduce_mut_callback(|s| s.reroll_seed());

    html!(
        <>
        <button {onclick} >{"⚄"}</button>

        {for result}
    </>
    )
}

#[derive(Properties, PartialEq)]
pub struct InputSliderProperties {
    pub prop_key: String,
    #[prop_or(None)]
    pub p_type: Option<PropertyType>,
}

#[function_component(InputSlider)]
pub fn input_slider(properties: &InputSliderProperties) -> Html {
    let key = properties.prop_key.clone();

    let value = *use_selector_with_deps(
        |state: &InputState, k| state.get_variable_value(k),
        key.clone(),
    )
    .as_ref();

    if let Some(p_type) = properties.p_type {
        let (min, max, step) = p_type.deconstruct();

        let key2 = key.clone();
        let key3 = key.clone();

        let on_slider_input =
            Dispatch::<InputState>::new().reduce_mut_callback_with(move |s, e: InputEvent| {
                let input: HtmlInputElement = e.target_unchecked_into();
                let new_value = input.value();
                let new_f_value: f32 = new_value.parse().unwrap();
                s.set_variable_value(key2.clone(), new_f_value);
            });

        let on_box_input =
            Dispatch::<InputState>::new().reduce_mut_callback_with(move |s, e: InputEvent| {
                let input: HtmlInputElement = e.target_unchecked_into();
                let new_value = input.value();
                let new_f_value: f32 = new_value.parse().unwrap();
                s.set_variable_value(key3.clone(), new_f_value);
            });

        html!(
                <div class="slider">

            <code style="width:80px" >{format!("{}", key)}</code>
          <input oninput={on_slider_input} type="range"  value={format!("{}",value )} min={format!("{}",min )} max={format!("{}",max )}  step={format!("{}",step )} />
          <input style="width:80px" oninput={on_box_input} type="number"  value={format!("{}",value )} min={format!("{}",min )} max={format!("{}",max )}  step={format!("{}",step )} />


        </div>
            )
    } else {
        html!(
                <div class="slider">
                <code style="width:80px" >{format!("{}", key)}</code>
                <input  type="range"  value={format!("{}",value )} disabled=true />
                <input style="width:80px" type="number"  value={format!("{}",value )} disabled=true  />
        </div>
            )
    }
}

#[function_component(NameBox)]
pub fn name_box() -> Html {
    let name = use_selector(|state: &InputState| state.name.clone())
        .as_ref()
        .clone();
    let oninput = Dispatch::<InputState>::new().reduce_mut_callback_with(|s, e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let value = input.value();
        s.name = value;
    });

    let onclick = Dispatch::<InputState>::new().reduce_mut_callback(|s| s.save());

    html! {
        <div style="display: flex;">
        <input {oninput}   value={name}   style="width: 100px;"         />
        <button {onclick}> {"Save"} </button>
        <ExamplesSelect/>
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
        s.update_text(value);
    });

    html!(
            <div>
    <p>

    </p>
    //https://css-tricks.com/creating-an-editable-textarea-that-supports-syntax-highlighted-code/
            <textarea id="input-textarea" name="input-textarea" class="input-textarea" rows="10" {oninput}
            value={text}
            spellcheck="false"
            >
            </textarea>
            </div>
        )
}

#[function_component(ErrorBox)]
pub fn erorr_box() -> Html {
    let err = use_selector(|s: &InputState| s.error.clone())
        .as_ref()
        .clone()
        .unwrap_or_else(|| "‎".to_string());
    html!(<code> {err} </code>)
}

#[function_component(DisplayBox)]
pub fn diplay_box() -> Html {
    let svg = use_selector(|s: &ImageState| s.svg.clone())
        .as_ref()
        .clone();

    html!(
        <iframe class="display-iframe" srcdoc={svg} scrolling="no"></iframe>
    )
}
