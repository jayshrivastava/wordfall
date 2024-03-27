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
   val: RwSignal<&'static str>,
   key: u64,
}

// impl IntoView for BlockState {
//     fn into_view(self) -> View {
//         View::Text(leptos_dom::Text::new(Oco::from(self.val)))
//     }
// }
const EMPTY:  &'static str = "_";
fn new_block_state(val :&'static str) -> BlockState {
    return BlockState{
        val: create_rw_signal(val),
        key: random(),
    }
}

fn makeBlockVec() -> Vec<BlockState> {
   let mut ret: Vec<BlockState> = vec![];
   for _  in 0..GRID_HEIGHT * GRID_HEIGHT {
       ret.push(new_block_state(EMPTY))
   }
    ret
}
#[component]
fn App() -> impl IntoView {
    let (block, setblock) = create_signal(makeBlockVec());
    let (current_block, set_current_block) = create_signal(0);


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
                log!("A");
                // set_current_block(vec![0]);
                let _ = block.with(|blocks| {
                    blocks[0].val.update(|val| *val = "A");
                log!("{}{}", blocks[0].key, blocks[0].val.get());
                    log!("{}{}", blocks[1].key, blocks[1].val.get());
                });
        }, Duration::from_secs(3));
    });
    view! {
        <For
            each=block
            key=|block_state| block_state.key.clone()
            let: bs
        >
        <div>
           {bs.val}
        </div>
        </For>
    }

}


fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| view! {
        <header::Header/>
        <App />
    })
}