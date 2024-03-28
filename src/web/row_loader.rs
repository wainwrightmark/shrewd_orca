use crate::state::prelude::*;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{window,};
use yew::prelude::*;
use yewdux::prelude::*;

pub struct RowLoader {
    callback: Closure<dyn FnMut()>,
}

impl Component for RowLoader {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let comp_ctx = ctx.link().clone();
        let callback =
            Closure::wrap(Box::new(move || comp_ctx.send_message(())) as Box<dyn FnMut()>);
        ctx.link().send_message(());

        Self { callback }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!(<></>)
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        Dispatch::<FullState>::new().reduce_mut(|x| x.load_more(1));

        let window = window().unwrap();
        let _ = window.request_animation_frame(self.callback.as_ref().unchecked_ref());
        false
    }
}
