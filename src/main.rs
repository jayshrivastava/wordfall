use leptos::*;
use stylance::import_crate_style;
use leptos_dom::View::Text;


import_crate_style!(my_style, "./src/styles.module.scss");

const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 20;
#[derive(Clone)]
struct BlockState {
   val: &'static str
}

impl IntoView for BlockState {
    fn into_view(self) -> View {
        View::Text(leptos_dom::Text::new(Oco::from(self.val)))
    }
}

fn newBlockState() -> BlockState {
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
            let (get, set) = create_signal(newBlockState());
            set_block[i].push(set);
            get_block[i].push(get);
        }
    }

    let values = vec![0, 1, 2];
    view! {
    // this will just render "012"
    <p>{values.clone()}</p>
    // or we can wrap them in <li>
    <ul>
        {get_block.into_iter()
            .map(|row| view! {
            <ul>
                {row.into_iter()
                .map(|get| view! { <li>{get()}</li>})
                .collect_view()}
            </ul>
        })
            .collect_view()}
    </ul>
}

    // let (get_count, set_count) = create_signal(0);
    //
    // view! {
    //     <button
    //         class=my_style::test
    //         on:click=move |_| {
    //             // on stable, this is set_count.set(3);
    //             set_count(3);
    //         }
    //     >
    //         "Click me: "
    //         // on stable, this is move || count.get();
    //         {move || get_count()}
    //     </button>
    // }
}


fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| view! {
        <App />
    })
}