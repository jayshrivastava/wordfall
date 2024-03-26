mod letter_gen;
use rand::{random, Rng};
mod header;
use crate::letter_gen::LETTERS;
use leptos::*;
use stylance::import_crate_style;
use leptos::logging::log;
use core::time::Duration;
import_crate_style!(styles, "./src/styles.module.scss");

// NB: The width variable exists separately in scss as well.
const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 10;

#[derive(Clone)]
struct BlockState {
   val: &'static str,
    id: u64,
}

impl BlockState {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    fn render(&self) -> impl IntoView {
        let div_class = match self.val {
            " " =>  styles::block,
            _ => styles::block_green,
        };
       view! {
           <div class=div_class>
                <p class=styles::block_text> {self.val} </p>
           </div>
       }
    }
}


// impl IntoView for BlockState {
//     fn into_view(self) -> View {
//         View::Text(leptos_dom::Text::new(Oco::from(self.val)))
//     }
// }
const EMPTY:  &'static str = "_";
fn new_block_state(val :&'static str) -> BlockState {
    return BlockState{
        val,
        id: random(),
    }
}
//
#[component]
fn App() -> impl IntoView {
    let mut set_block: Vec<Vec<WriteSignal<BlockState>>>  = Vec::with_capacity(GRID_HEIGHT);
    let mut get_block: Vec<Vec<ReadSignal<BlockState>>> = Vec::with_capacity(GRID_HEIGHT) ;
    for i in 0..GRID_HEIGHT {
        set_block.push(Vec::with_capacity(GRID_WIDTH));
        get_block.push(Vec::with_capacity(GRID_WIDTH));
        for j in 0..GRID_WIDTH {
            let (get, set) = create_signal(new_block_state(EMPTY));
            set_block[i].push(set);
            get_block[i].push(get);
        }
    }

    let (current_block, set_current_block) = create_signal(vec![-1,-1]);


    // let handle = window_event_listener(ev::keypress,move |ev| {
    //     if current_block()[0] != -1 {
    //         let code = ev.code();
    //         log!("code = {code:?}");
    //     }
    //
    // });
    // on_cleanup(move || handle.remove());
    //
    create_effect(move |_| {
        set_interval(move || {
            // if current_block()[0] == -1 {
            //     log!("A");
            //     set_current_block(vec![0, 5]);
            //     set_block[0][5](new_block_state(LETTERS[0]))
            // }
            log!("B")
        }, Duration::from_secs(3));
    });
    view! {
    <div class=styles::grid_container>
        {get_block.into_iter()
            .map(|row| view! {
            <div class=styles::grid_row>
                {row.into_iter()
                .map(|block| view! { <div>{block().render()}</div>})
                .collect_view()}
            </div>
        })
            .collect_view()}

    </div>
    }

}


fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| view! {
        <header::Header/>
        <App />
    })
}