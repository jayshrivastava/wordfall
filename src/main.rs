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
const GRID_WIDTH: usize = 9;
const GRID_HEIGHT: usize = 11;
const GRID_SIZE: usize =GRID_WIDTH * GRID_HEIGHT;

const EMPTY:  &'static str = " ";

const TICK: u64 = 1;

const STARTING: usize = 4;

const KEY_A: &str = "KeyA";
const KEY_S: &str = "KeyS";
const KEY_D: &str = "KeyD";
const KEY_W: &str = "KeyW";
const ARR_L: &str = "ArrowLeft";
const ARR_D: &str = "ArrowDown";
const ARR_R: &str = "ArrowRight";
const ARR_U: &str = "ArrowUp";

#[derive(Clone)]
struct BlockState {
    val: RwSignal<&'static str>,
    key: u64,
}

fn new_block_state(val :&'static str) -> BlockState {
    return BlockState{
        val: create_rw_signal(val),
        key: random(),
    }
}

fn make_block_vec(gen: LetterGenerator) -> Vec<BlockState> {
   let mut ret: Vec<BlockState> = vec![];
   for _  in 0..GRID_WIDTH * GRID_HEIGHT {
       ret.push(new_block_state(EMPTY))
   }
   // Spawn the first block.
   ret[GRID_WIDTH / 2].val.update(|val| *val = gen.next_letter());
   ret
}

#[component]
fn App() -> impl IntoView {
    let gen = LetterGenerator{};
    let (grid, set_grid) = create_signal(make_block_vec(gen));
    let (current, set_current) = create_signal(GRID_WIDTH / 2);

    let spawn = move || {
        set_current(GRID_WIDTH / 2);
        if grid.get()[GRID_WIDTH/2].val.get() != EMPTY {
            // TODO: handle user lost the game condition
            panic!("TODO: lost")
        }
        let _ = grid.with(|blocks| {
            blocks[GRID_WIDTH / 2].val.update(|val| *val = gen.next_letter());
        });
    };

    let translate = move |next_idx: usize| -> bool {
        return grid.with(|blocks| {
            if blocks[next_idx].val.get() != EMPTY {
                return false
            }
            let prev = blocks[current.get()].val.get();
            blocks[current.get()].val.update(|val| *val = EMPTY);
            set_current(next_idx);
            blocks[next_idx].val.update(|val| *val = prev);
            return true;
        });
    };
    let down = move || -> bool {
        if current.get() + GRID_WIDTH >= GRID_SIZE {
            return false
        }
        return translate(current.get() + GRID_WIDTH);
    };
    let right = move || -> bool {
        if (current.get() + 1) % GRID_WIDTH < current.get() % GRID_WIDTH  {
            return false
        }
        return translate(current.get() + 1);
    };
    let left = move || -> bool {
        if (current.get() - 1) % GRID_WIDTH > current.get() % GRID_WIDTH  {
            return false
        }
        return translate(current.get() - 1);
    };
    let up = move || -> bool {
        let mut i = current.get();
        grid.with(move |blocks| {
            while i+GRID_WIDTH < GRID_SIZE && blocks[i+GRID_WIDTH].val.get() == EMPTY {
                i = i + GRID_WIDTH;
            }
            translate(i);
        });
        return false;
    };

    let handle_key_press = move |code: &str| {
        let moved = match code {
            KEY_A | ARR_L => left(),
            KEY_S | ARR_D => down(),
            KEY_D | ARR_R => right(),
            KEY_W | ARR_U => up(),
            _ => false,
        };
        // Letter is locked in.
        if !moved && (code == KEY_S || code == KEY_W ||
            code == ARR_D || code == ARR_U) {
            spawn();
        }
    };

    let handle2 = window_event_listener(ev::keydown, move |ev| {
       handle_key_press(ev.code().as_str());
    });
    on_cleanup(move || handle2.remove());
    // create_effect(move |_| {
    //     set_interval(move || {
    //         // down();
    //     }, Duration::from_secs(TICK));
    // });
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
            <div class=styles::arrow_container>
            <button class=styles::arrow_button on:click=move |_| { handle_key_press(KEY_A); }> "<"</button>
            <button class=styles::arrow_button on:click=move |_| { handle_key_press(KEY_S); }> "."</button>
            <button class=styles::arrow_button on:click=move |_| { handle_key_press(KEY_W); }> "^"</button>
            <button class=styles::arrow_button on:click=move |_| { handle_key_press(KEY_D); }> ">"</button>

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