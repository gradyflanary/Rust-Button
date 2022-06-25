use yew::prelude::*;

struct Model {
    value: i64,
    fizz_state: String
        
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model {
        value: 0,
        fizz_state: String::from("0")
    });

    let onclick = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Model {
                value: state.value + 1,
                fizz_state: match (state.value % 3, state.value % 5, state.value){
                    (_,_,0) => state.value.to_string(),
                    (0,0,_) => String::from("FizzBuzz"),
                    (0,_,_) => String::from("Fizz"),
                    (_,0,_) => String::from("Buzz"),
                    _ => state.value.to_string()

                }
            }
            )
        }
        )
    };

    html! {
        <div>
            <button {onclick}>{" +1 "}</button>
            <p>{ &*state.fizz_state }</p>
        </div>
    }
    }

fn main() {
    yew::start_app::<App>();
}
