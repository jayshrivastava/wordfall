mod letter_gen;
use rand::{random, Rng};
mod header;
mod trie;
mod words;
mod scoring;
mod seed;

use gloo_storage::Storage;
use leptos::*;
use stylance::import_crate_style;
use leptos::logging::log;
use core::time::Duration;
use std::ops::Deref;
use serde::{Serialize, Deserialize};
use crate::letter_gen::{Generator, LetterGenerator, TestGenerator, MIN_WORD_SIZE};
use crate::scoring::get_score_single;
use crate::trie::TrieNode;
use crate::words::make_words;
use crate::seed::get_seed;

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

const GREEN_MS: u64 = 500;
const YELLOW_MS: u64 = 200;

const KEY_A: &str = "KeyA";
const KEY_S: &str = "KeyS";
const KEY_D: &str = "KeyD";
const KEY_W: &str = "KeyW";
const ARR_L: &str = "ArrowLeft";
const ARR_D: &str = "ArrowDown";
const ARR_R: &str = "ArrowRight";
const ARR_U: &str = "ArrowUp";

#[derive(Clone, Serialize, Deserialize)]
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

fn make_block_vec(set_gen: WriteSignal<LetterGenerator>) -> Vec<BlockState> {
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

#[derive(Clone, Serialize, Deserialize)]
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
fn ModalFooter() -> impl IntoView {
   return view! {
       <p class=styles::modal_content_p> Any issues? Report them <a href="https://github.com/jayshrivastava/wordfall/issues">here</a> </p>
       <p class=styles::modal_content_p> Enjoying the game? <a href="https://www.buymeacoffee.com/jayants"> Buy me a coffee</a> " :)" </p>
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
            <img src="./../img/info5.png"> </img>
            <br/>
            <ModalFooter/>
        </div>
      </div>
   }
}

#[component]
fn EndModal(
    display: ReadSignal<&'static str>,
    set_display: WriteSignal<&'static str>,
    words: ReadSignal<Vec<WordWithKey>>,
    score: ReadSignal<u32>,
) -> impl IntoView {
    return view! {
      <div
        class=styles::modal
        style:display=move || {
            if display.get() == "lost" || display.get() == "won" {
                return "block"
            }
            return "none"
        }
   >
        <div class=styles::modal_content>

            // <p
            //     class=styles::modal_close
            //      on:mouseup=move |_| { set_display.update(|d| {*d = ""}); }
            //      on:touchend=move |ev| { ev.prevent_default(); set_display.update(|d| {*d = ""}); }
            // >
            //     close
            // </p>
            <p class=styles::modal_end_header> {move || {
                if display.get() == "lost" {
                    return "Better luck next time..."
                }
                return "Nice work!"
            }} </p>
            <p class=styles::modal_content_p> Your total score was {move || {score.get() }}</p>
            <br/>
            <For
                    each=words
                    key=|word_with_key| word_with_key.key.clone()
                    let: wwk
            >
                <p class=styles::modal_content_p>{format!("{} - {}", wwk.word, get_score_single(wwk.word))}</p>
            </For>
            <br/>
                <p class=styles::modal_content_p>{"Come back tomorrow for a new challenge!"}</p>
            <br/>
            <ModalFooter/>
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
    let store = gloo_storage::LocalStorage::raw();

    let seed = get_seed();
    let seed_str = seed
        .iter()
        .map(|&byte| format!("{:08b}", byte)) // Format each byte as binary with leading zeros
        .collect::<Vec<_>>()
        .join("");
    let stored_seed = store.get("WORDFALL_SEED").unwrap();

    let words = make_words();
    let (gen, set_gen) = create_signal(LetterGenerator::new(words.clone(), seed));
    let (grid, set_grid) = create_signal(make_block_vec(set_gen));
    let (current, set_current) = create_signal(GRID_WIDTH / 2);
    let (checking, set_checking) = create_signal(false); // unsaved
    let (t, _) = create_signal(make_trie(words.clone())); // unsaved
    let (game_meta_text, set_game_meta_text) = create_signal(vec![]);
    let (num_remaining, set_num_remaining) = create_signal(0);
    let (score, set_score) = create_signal(0);
    let (last_words, set_last_words) = create_signal(vec![]);
    let (words_found, set_words_found) = create_signal(vec![]);
    let store = gloo_storage::LocalStorage::raw();
    let (show_intro_modal, set_show_intro_modal) = create_signal(store.get(WORDFALL_FIRST_TIME).unwrap().is_none()); // unsaved
    let (cycle, set_cycle) = create_signal(false); // unsaved
    let (display_end, set_display_end) = create_signal("");

    let reset = move || {
        set_gen.update(|sg| *sg = LetterGenerator::new(words.clone(), seed));
        set_grid.update(|g| *g = make_block_vec(set_gen));
        set_current.update(|c| {*c = GRID_WIDTH / 2});
        // set_game_meta_text.update(|gm| *gm = vec![]);
        // set_num_remaining.update(|nr| { *nr = 0});
        set_score.update(|s| {*s = 0});
        set_last_words.update(|lw| { *lw = vec![]});
        set_words_found.update(|wf| { *wf = vec![]});
        set_show_intro_modal.update(|i| {*i = true});
        set_display_end.update(|disp| { *disp = ""});
    };

    if stored_seed.is_some() {
        if seed_str != stored_seed.unwrap() {
            reset();
            store.clear().unwrap();
            store.set("WORDFALL_SEED", &seed_str).unwrap();
        }
    } else {
        store.set("WORDFALL_SEED", &seed_str).unwrap();
    }

    let load_state = move || {
        let storage = gloo_storage::LocalStorage::raw();
        // Nothing stored, so use defaults.
        if storage.get("WORDFALL_GEN").unwrap().is_none() {
            return
        }
        set_gen.update(|ge| {
            let j = storage.get("WORDFALL_GEN").unwrap().unwrap();
            *ge = serde_json::from_str(j.as_str()).unwrap();
        });
        set_grid.update(|g| {
            let j = storage.get("WORDFALL_GRID").unwrap().unwrap();
            *g = serde_json::from_str(j.as_str()).unwrap();
        });
        set_current.update(|c| {
            let j = storage.get("WORDFALL_CURRENT").unwrap().unwrap();
            *c = serde_json::from_str(j.as_str()).unwrap();
        });
        set_game_meta_text.update(|meta_text| {
            let j = storage.get("WORDFALL_META_TEXT").unwrap().unwrap();
            *meta_text = serde_json::from_str(j.as_str()).unwrap();
        });
        set_num_remaining.update(|rem| {
            let j = storage.get("WORDFALL_NUM_REMAINING").unwrap().unwrap();
            *rem = serde_json::from_str(j.as_str()).unwrap();
        });
        set_score.update(|s| {
            let j = storage.get("WORDFALL_SCORE").unwrap().unwrap();
            *s = serde_json::from_str(j.as_str()).unwrap();
        });
        set_last_words.update(|lw| {
            let j = storage.get("WORDFALL_LAST_WORDS").unwrap().unwrap();
            let static_s = Box::leak(j.into_boxed_str());
            *lw = serde_json::from_str(static_s).unwrap();
        });
        set_words_found.update(|wf| {
            let j = storage.get("WORDFALL_WORDS_FOUND").unwrap().unwrap();
            let static_s = Box::leak(j.into_boxed_str());
            *wf = serde_json::from_str(static_s).unwrap();
        });
        set_display_end.update(|d| {
            let j = storage.get("WORDFALL_DISPLAY_END").unwrap().unwrap();
            let static_s = Box::leak(j.into_boxed_str());
            *d = serde_json::from_str(static_s).unwrap();
        });
    };

    // Load initial state if it exists.
    load_state();

    let save_state = move || {
        let storage = gloo_storage::LocalStorage::raw();
        gen.with(|ge| {
            let json = serde_json::to_string(&ge).unwrap();
            storage.set("WORDFALL_GEN", json.as_str()).unwrap();
        });
        grid.with(|g| {
            let json = serde_json::to_string(&g).unwrap();
            storage.set("WORDFALL_GRID", json.as_str()).unwrap();
        });
        current.with(|c| {
            let json = serde_json::to_string(&c).unwrap();
            storage.set("WORDFALL_CURRENT", json.as_str()).unwrap();
        });
        game_meta_text.with(|meta_text| {
            let json = serde_json::to_string(&meta_text).unwrap();
            storage.set("WORDFALL_META_TEXT", json.as_str()).unwrap();
        });
        num_remaining.with(|rem| {
            let json = serde_json::to_string(&rem).unwrap();
            storage.set("WORDFALL_NUM_REMAINING", json.as_str()).unwrap();
        });
        score.with(|s| {
            let json = serde_json::to_string(&s).unwrap();
            storage.set("WORDFALL_SCORE", json.as_str()).unwrap();
        });
        last_words.with(|lw| {
            let json = serde_json::to_string(&lw).unwrap();
            storage.set("WORDFALL_LAST_WORDS", json.as_str()).unwrap();
        });
        words_found.with(|wf| {
            let json = serde_json::to_string(&wf).unwrap();
            storage.set("WORDFALL_WORDS_FOUND", json.as_str()).unwrap();
        });
        display_end.with(|d| {
            let json = serde_json::to_string(&d).unwrap();
            storage.set("WORDFALL_DISPLAY_END", json.as_str()).unwrap();
        });
    };

    // Set next letters and num remaining initially.
    create_effect(move |_| {
        set_gen.update(|g| {
            set_game_meta_text.update(|nl| *nl = g.next_n_letters(LOOKAHEAD));
            set_num_remaining.update(|rem| *rem = g.num_letters_left())
        });
    });

     let spawn = move || {
         // Lost condition
        if grid.get()[GRID_WIDTH/2].val.get() != EMPTY {
            // TODO: handle user lost the game condition
            set_display_end.update(|d| {*d = "lost"});
            return
        }
        set_current(GRID_WIDTH / 2);
        let _ = grid.with(|blocks| {
            blocks[GRID_WIDTH / 2].val.update(|val| {
                set_gen.update(|g| {
                    let next = g.next_letter();
                    if next.is_none() {
                        // Win condition
                        set_display_end.update(|d| {*d = "won"});
                        return
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
                // log!("{:?}", words);

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
                    // Allow events to be handled again, spawn a new block, save the state to storage.
                    set_checking(false);
                    spawn();
                    save_state();
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
                        set_words_found.update(|words_found| {words_found.push(make_word_with_key(word))});
                    }

                    // Slide all the blocks down.
                    for i in (0..GRID_SIZE).rev() {
                        up_idx(i);
                    }
                    // Hold the slid down blocks for a small period of time.
                    set_timeout(move || {
                        // Signal to basically call check_for_words() recursively here.
                        set_cycle(true);
                    }, Duration::from_millis(YELLOW_MS));
                }, Duration::from_millis(GREEN_MS));
            });
        });
    };

    create_effect(move |_| {
        if cycle.get() {
            set_cycle.set(false);
            check_for_words();
        }
    });

    let handle_key_press = move |code: &str| {
        if display_end.get() != "" {
            return
        }
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
            <EndModal display=display_end set_display=set_display_end words=words_found score=score/>
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
                <p class=styles::game_meta_text>"Words Found: "{move || words_found.get().len()}</p>
                </div>
                <div class=styles::right_meta>
                    <p class=styles::game_meta_text>"Score: "{move || score.get()}</p>
                </div>
            </div>
            // Previous words
            <div class=styles::meta_container>
                <div class=styles::left_meta>
                    <div>
                        <p class=styles::game_meta_text>{"Last Few Words:"}</p>
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