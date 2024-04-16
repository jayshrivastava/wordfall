mod letter_gen;
use rand::{random, Rng};
mod header;
mod trie;
mod words;
mod scoring;

use gloo_storage::Storage;
use leptos::*;
use stylance::import_crate_style;
use leptos::logging::log;
use core::time::Duration;
use std::ops::Deref;
use crate::letter_gen::{Generator, LetterGenerator, TestGenerator, MIN_WORD_SIZE};
use crate::scoring::get_score_single;
use crate::trie::TrieNode;
use crate::words::make_words;


import_crate_style!(styles, "./src/styles.module.scss");

// NB: The width variable exists separately in scss as well.
const GRID_WIDTH: usize = 9;
const GRID_HEIGHT: usize = 9;
const GRID_SIZE: usize =GRID_WIDTH * GRID_HEIGHT;

const LOOKAHEAD: usize = 3;

const LAST_WORDS_WINDOW: usize= 3;

const EMPTY:  char = ' ';

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
    val: RwSignal<char>,
    selected: RwSignal<bool>,
    key: u64,
}

fn new_block_state(val: char) -> BlockState {
    return BlockState{
        val: create_rw_signal(val),
        selected: create_rw_signal(false),
        key: random(),
    }
}

fn make_block_vec(set_gen: WriteSignal<TestGenerator>) -> Vec<BlockState> {
   let mut ret: Vec<BlockState> = vec![];
   for _  in 0..GRID_WIDTH * GRID_HEIGHT {
       ret.push(new_block_state(EMPTY))
   }
   // Spawn the first block.
   ret[GRID_WIDTH / 2].val.update(|val|{
       set_gen.update(|g| {
           // Guaranteed to be words here because we just initialized the generator.
           *val = g.next_letter().unwrap()
       })
   });
   ret
}

fn make_trie(words: Vec<&'static str>) -> TrieNode {
    let mut t = trie::TrieNode::new();
    for word in words {
        t.add_word(word)
    }
    t
}

#[derive(Clone)]
struct WordWithKey {
    word: &'static str,
    key: u64,
}

fn make_word_with_key(word: &'static str) -> WordWithKey {
    return WordWithKey{
        word,
        key: random(),
    }
}

#[component]
fn InfoModal(display: ReadSignal<bool>, set_display: WriteSignal<bool>) -> impl IntoView {
   return view! {
      <div
        class=styles::modal
        style:display=move || {
            if display.get() {
                return "block"
            }
            return "none"
        }
   >
        <div class=styles::modal_content>

            <p
                class=styles::modal_close
                 on:mouseup=move |_| { set_wordfall_first_time(); set_display.update(|d| {*d = false}); }
                 on:touchend=move |ev| { ev.prevent_default(); set_wordfall_first_time(); set_display.update(|d| {*d = false}); }
            >
                close
            </p>
            <img src="./../img/info2.png"> </img>
            <br/>
            <p> Any issues? Report them <a href="https://github.com/jayshrivastava/wordfall/issues">here</a> </p>
            <p> Enjoying the game? <a href="https://www.buymeacoffee.com/jayants"> Buy me a coffee</a> " :)" </p>
        </div>
      </div>
   }
}

const WORDFALL_FIRST_TIME: &str = "WORDFALL_FIRST_TIME";

fn set_wordfall_first_time() {
   let s = gloo_storage::LocalStorage::raw();
   s.set("WORDFALL_FIRST_TIME", "true").unwrap();
}

#[component]
fn App() -> impl IntoView {


    let storage = gloo_storage::LocalStorage::raw();
    let first_time = storage.get(WORDFALL_FIRST_TIME).unwrap().is_none();

    let words = make_words();

    let (gen, set_gen) = create_signal(TestGenerator::new(words.clone()));
    let (grid, set_grid) = create_signal(make_block_vec(set_gen));
    let (current, set_current) = create_signal(GRID_WIDTH / 2);
    let (checking, set_checking) = create_signal(false);
    let (t, _) = create_signal(make_trie(words.clone()));
    let (game_meta_text, set_game_meta_text) = create_signal(vec![]);
    let (num_remaining, set_num_remaining) = create_signal(0);
    let (score, set_score) = create_signal(0);
    let (last_words, set_last_words) = create_signal(vec![]);
    let (words_found, set_words_found) = create_signal(0);
    let (show_intro_modal, set_show_intro_modal) = create_signal(first_time);

    // Set next letters and num remaining initially.
    create_effect(move |_| {
        set_gen.update(|g| {
            set_game_meta_text.update(|nl| *nl = g.next_n_letters(LOOKAHEAD));
            set_num_remaining.update(|rem| *rem = g.num_letters_left())
        });
    });

     let spawn = move || {
        if grid.get()[GRID_WIDTH/2].val.get() != EMPTY {
            // TODO: handle user lost the game condition
            panic!("TODO: lost")
        }
        set_current(GRID_WIDTH / 2);
        let _ = grid.with(|blocks| {
            blocks[GRID_WIDTH / 2].val.update(|val| {
                set_gen.update(|g| {
                    let next = g.next_letter();
                    if next.is_none() {
                        panic!("TODO: you won")
                    }
                    *val = next.unwrap();
                    set_game_meta_text.update(|nl| *nl = g.next_n_letters(LOOKAHEAD));
                    set_num_remaining.update(|rem| *rem = g.num_letters_left())
                })
            });
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
        // current.get() - 1 underflows below >:( golang would never
        if current.get() == 0 {
            return false
        }
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

    let translate_idx = move |prev_idx: usize, next_idx: usize| -> bool {
        return grid.with(|blocks| {
            if blocks[next_idx].val.get() != EMPTY {
                return false
            }
            let prev = blocks[prev_idx].val.get();
            blocks[prev_idx].val.update(|val| *val = EMPTY);
            blocks[next_idx].val.update(|val| *val = prev);
            return true;
        });
    };

    let up_idx = move |block_num: usize| -> bool {
        let mut i = block_num;
        grid.with(move |blocks| {
            while i+GRID_WIDTH < GRID_SIZE && blocks[i+GRID_WIDTH].val.get() == EMPTY {
                i = i + GRID_WIDTH;
            }
            translate_idx(block_num, i);
        });
        return false;
    };

    let check_for_words = move || {
        set_checking(true);

        grid.with(move |blocks| {
            t.with(move |tri| {
                let mut indexes_final: Vec<Vec<usize>> = vec![];
                let mut words = vec![];
                for i in 0..blocks.len() {
                    for dir in 0..2 {
                        let mut trav = tri;
                        let mut j = i;
                        let mut indexes = vec![];
                        while blocks[j].val.get() != EMPTY && trav.has_next(blocks[j].val.get()) {
                            trav = trav.next(blocks[j].val.get());
                            indexes.push(j);
                            // Check for words of at least length 3.
                            if trav.terminal() && indexes.len() >= MIN_WORD_SIZE {
                                let word = trav.get_word();
                                indexes_final.push(indexes.clone());
                                words.push(word);
                            }
                            match dir {
                                0 => {
                                    j += GRID_WIDTH;
                                    if j >= GRID_SIZE {
                                        break
                                    }
                                },
                                1 => {
                                    if (j+1) % GRID_WIDTH < j % GRID_WIDTH {
                                        break
                                    }
                                    j += 1
                                },
                                // Left and up directions.
                                // 2 => {
                                //     if j-GRID_WIDTH < 0 {
                                //         break
                                //     }
                                //     j -= GRID_WIDTH;
                                // },
                                // 3 => {
                                //     if (j-1) % GRID_WIDTH > j % GRID_WIDTH  {
                                //         break
                                //     }
                                //     j -= 1
                                // },
                                _ => {},
                            }
                        }
                    }
                }
                log!("{:?}", words);

                let found_words = words.len() > 0;
                let idx_final_iter = &indexes_final;
                for word_idx_arr in idx_final_iter {
                    for idx in word_idx_arr {
                        grid.with(move |blocks: &Vec<BlockState>| {
                            blocks[*idx].selected.update(|s| {
                                *s = true
                            })
                        });
                    }
                }
                let idx_final_clone = indexes_final.clone();
                let words_clone = words.clone();
                if !found_words {
                    set_checking(false);
                    spawn();
                    return
                }
                set_timeout(move || {
                    // Update grid. TODO: slide letters down.
                    for word_idx_arr in idx_final_clone {
                        for idx in word_idx_arr {
                            grid.with(|blocks: &Vec<BlockState>| {
                                blocks[idx].val.update(|val| *val = EMPTY);
                                blocks[idx].selected.update(|s| *s = false)
                            });
                        }
                    }
                    // Set last words.
                    for word in &words_clone {
                        set_last_words.update(|last_words| {
                            last_words.insert(0, make_word_with_key(word));
                            if last_words.len() > LAST_WORDS_WINDOW {
                                last_words.pop();
                            }
                        });
                        set_score.update(|score| {
                            *score = *score + get_score_single(word)
                        });
                        set_words_found.update(|words_found| {*words_found = *words_found + words_clone.len()});
                    }

                    // Slide all the blocks down.
                    for i in (0..GRID_SIZE).rev() {
                        up_idx(i);
                    }

                    set_checking(false);
                    spawn();
                }, Duration::from_millis(500));
            });
        });


    };

    let handle_key_press = move |code: &str| {
        // Ignore keypresses while checking / popping words.
        if checking.get() {
            return
        }
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
            check_for_words();
        }
    };



    let handle2 = window_event_listener(ev::keydown, move |ev| {
       handle_key_press(ev.code().as_str());
    });
    on_cleanup(move || handle2.remove());

    view! {
        <div class=styles::grid_container_container>
            <InfoModal display=show_intro_modal set_display=set_show_intro_modal/>
            <div class=styles::grid_container>
                <For
                    each=grid
                    key=|block_state| block_state.key.clone()
                    let: bs
                >
                <div
                    class=styles::grid_item
                    style:background-color=move || {
                        if bs.val.get() == EMPTY {
                            return ""
                        }
                        if bs.selected.get() {
                            return "rgb(144, 201, 113)"
                        }
                        return "rgb(255, 216, 158)"
                    }
                >
                   {bs.val}
                </div>
                </For>
            </div>
            <div class=styles::arrow_container>
                <button class=styles::arrow_button
                    on:mouseup=move |_| { handle_key_press(KEY_A); }
                    on:touchend=move |ev| {ev.prevent_default(); handle_key_press(KEY_A); }
                > "⬅️"</button>
                <button class=styles::arrow_button
                    on:mouseup=move |_| { handle_key_press(KEY_S); }
                    on:touchend=move |ev| {ev.prevent_default(); handle_key_press(KEY_S); }
                > "⬇️"</button>
                <button class=styles::arrow_button
                    on:mouseup=move |_| { handle_key_press(KEY_W); }
                    on:touchend=move |ev| {ev.prevent_default(); handle_key_press(KEY_W); }
                > "⏬"</button>
                <button class=styles::arrow_button
                    on:mouseup=move |_| { handle_key_press(KEY_D); }
                    on:touchend=move |ev| {ev.prevent_default(); handle_key_press(KEY_D); }
                > "➡️"</button>
            </div>
            // RULES
            <div class=styles::rules>
                <p
                    class=styles::rules_text
                    on:mouseup=move |_| { set_show_intro_modal.update(|show| {*show=true});}
                    on:touchend=move |ev| {ev.prevent_default(); set_show_intro_modal.update(|show| {*show=true});}
                >"⇱ Rules"</p>
            </div>
            // Next chars and remaining count.
            <div class=styles::meta_container>
                <div class=styles::left_meta>
                    <p class=styles::game_meta_text>"Up Next: "{move || game_meta_text.get().iter().map(|&c| c.to_string())
                        .collect::<Vec<_>>().join(", ")}</p>
                </div>
                <div class=styles::right_meta>
                    <p class=styles::game_meta_text>"Remaining: "{move || num_remaining.get()}</p>
                </div>
            </div>

            // Words found + score
            <div class=styles::meta_container>
                <div class=styles::left_meta>
                <p class=styles::game_meta_text>"Words Found: "{move || words_found.get()}</p>
                </div>
                <div class=styles::right_meta>
                    <p class=styles::game_meta_text>"Score: "{move || score.get()}</p>
                </div>
            </div>
            // Previous words
            <div class=styles::meta_container>
                <div class=styles::left_meta>
                    <div>
                        <p class=styles::game_meta_text>{"Previous Words:"}</p>
                        <div class=styles::last_words_indent>
                            <For
                                each=last_words
                                key=|word_with_key| word_with_key.key.clone()
                                let: wwk
                            >
                            <p class=styles::game_meta_text>{format!("{} - {}", wwk.word, get_score_single(wwk.word))}</p>
                            </For>
                        </div>
                    </div>

                </div>

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