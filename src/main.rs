use leptos::*;
use stylance::import_crate_style;

import_crate_style!(my_style, "./src/styles.module.scss");

const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 20;

struct BlockState {
   val: char
}

fn newBlockState() -> BlockState {
    return BlockState{
        val: 'F',
    }
}
//
#[component]
fn App() -> impl IntoView {
    // let mut set_block: Vec<Vec<WriteSignal<BlockState>>> = vec![vec![(); GRID_WIDTH]; GRID_HEIGHT];
    // let mut get_block: Vec<Vec<ReadSignal<BlockState>>> = vec![vec![(); GRID_WIDTH]; GRID_HEIGHT];
    // for i in 0..GRID_HEIGHT {
    //     for j in 0..GRID_WIDTH {
    //         let (get, set) = create_signal(newBlockState());
    //         set_block[i][j] = set;
    //         get_block[i][j] = get;
    //     }
    // }
    //


    let (get_count, set_count) = create_signal(0);

    view! {
        <button
            class=my_style::test
            on:click=move |_| {
                // on stable, this is set_count.set(3);
                set_count(3);
            }
        >
            "Click me: "
            // on stable, this is move || count.get();
            {move || get_count()}
        </button>
    }
}


fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| view! {
        <App />
    })
}