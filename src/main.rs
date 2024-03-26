use leptos::*;
use stylance::import_crate_style;

import_crate_style!(styles, "./src/styles.module.scss");

// NB: The width variable exists separately in scss as well.
const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 20;
#[derive(Clone)]
struct BlockState {
   val: &'static str
}

impl BlockState {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    fn render(&self) -> impl IntoView {
       view! {
           <div class=styles::block>
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

fn new_block_state() -> BlockState {
    return BlockState{
        val: "F",
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
            let (get, set) = create_signal(new_block_state());
            set_block[i].push(set);
            get_block[i].push(get);
        }
    }

    view! {
    <div class=styles::grid_container>
        {get_block.into_iter()
            .map(|row| view! {
            <div class=styles::grid_row>
                {row.into_iter()
                .map(|get| view! { <div>{get().render()}</div>})
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
        <App />
    })
}