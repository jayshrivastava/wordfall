mod letter_gen;
use rand::{random, Rng};
mod header;
use leptos::*;
use stylance::import_crate_style;
use leptos::logging::log;
use core::time::Duration;
use crate::letter_gen::LetterGenerator;
import_crate_style!(styles, "./src/styles.module.scss");

// NB: The width variable exists separately in scss as well.
const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 10;

const GRID_SIZE: usize =GRID_WIDTH * GRID_HEIGHT;

#[derive(Clone)]
struct BlockState {
   val: RwSignal<&'static str>,
   key: u64,
}

const EMPTY:  &'static str = " ";

const TICK: u64 = 1;

const STARTING: usize = 4;
fn new_block_state(val :&'static str) -> BlockState {
    return BlockState{
        val: create_rw_signal(val),
        key: random(),
    }
}

fn make_block_vec() -> Vec<BlockState> {
   let mut ret: Vec<BlockState> = vec![];
   for _  in 0..GRID_HEIGHT * GRID_HEIGHT {
       ret.push(new_block_state(EMPTY))
   }
   ret
}

#[component]
fn App() -> impl IntoView {
    let gen = LetterGenerator{};
    let (grid, set_grid) = create_signal(make_block_vec());
    let (current, set_current) = create_signal(0);

    let spawn = move || {
        set_current(4);
        let _ = grid.with(|blocks| {
            blocks[4].val.update(|val| *val = gen.next_letter());
        });
    };

    let translate = move |next_idx: usize| {
        let _ = grid.with(|blocks| {
            if next_idx >= 0 && next_idx < GRID_SIZE && blocks[next_idx].val.get() == EMPTY {
                let prev = blocks[current.get()].val.get();
                blocks[current.get()].val.update(|val| *val = EMPTY);
                set_current(next_idx);
                blocks[next_idx].val.update(|val| *val = prev);
            }
        });
    };

    // let handle = window_event_listener(ev::keypress,move |ev| {
    //     if current_block()[0] != -1 {
    //         let code = ev.code();
    //         log!("code = {code:?}");
    //     }
    //
    // });
    // on_cleanup(move || handle.remove());
    //
    spawn();
    create_effect(move |_| {
        set_interval(move || {
            translate(current.get()+10);

                // let _ = grid.with(|blocks| {
                //     let c = current.get();
                //     let next = c + GRID_WIDTH;
                //     if next >= GRID_SIZE {
                //
                //     } else {
                //         blocks[c].val.update(|val| *val = "A");
                //         set_current(next);
                //     }
                //
                // });
        }, Duration::from_secs(TICK));
    });
    view! {
        <div class=styles::grid_container_container>
            <div class=styles::grid_container>
                <For
                    each=grid
                    key=|block_state| block_state.key.clone()
                    let: bs
                >
                <div
                    class=styles::grid_item
                    style:background-color=move || {
                        match bs.val.get() {
                            EMPTY => "",
                            _ =>  "rgb(255, 216, 158)",
                        }
                    }
                >
                   {bs.val}
                </div>
                </For>
            </div>
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