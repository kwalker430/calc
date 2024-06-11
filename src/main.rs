use std::cell::RefCell;
use std::rc::Rc;

slint::slint!{
    import { Button, VerticalBox } from "std-widgets.slint";

    export global CalcLogic{
        callback button-pressed(string);
    }

    component Button {
        in property <string>text;
        min-height: 30px;
        min-width: 30px;
        Rectangle {
            background: ta.pressed ? white : ta.has-hover ? black: grey;
            animate background {duration: 100ms;}
            border-radius: 4px;
            border-width: 2px;
            border-color: self.background.darker(20%);
            ta := TouchArea {
                clicked => {CalcLogic.button-pressed(root.text);}
            }
        }
            Text {text: root.text;}
    }

    export component App inherits Window {
        in property <int> value: 0;
        title:  "rCalc";
        GridLayout {
            padding: 20px;
            spacing: 10px;
            Text { text: value; colspan: 3; }
            Row {
                Button { text: "1";}
                Button { text: "2";}
                Button { text: "3";}
                Button { text: "+";}
            }
            Row {
                Button { text: "4";}
                Button { text: "5";}
                Button { text: "6";}
                Button { text: "-";}
            } 
            Row {
                Button { text: "7";}
                Button { text: "8";}
                Button { text: "9";}
                Button { text: "*";}
            }  
            Row {
                Button { text: "0";}
                Button { text: "/";}
                Button { text: "=";}
                Button { text: "c";}
            } 
        }
    }
}

#[derive(Default)]
struct CalcState {
    prev_value: i32,
    curr_value: i32,
    operator: slint::SharedString,
}

fn main() {
    let app = App::new().unwrap();
    let weak = app.as_weak();
    let state: Rc<RefCell<CalcState>> = Rc::new(RefCell::new(CalcState::default()));

    app.global::<CalcLogic>().on_button_pressed(move | value| {
        let app = weak.unwrap();
        let mut state = state.borrow_mut();
        if let Ok(val) = value.parse::<i32>(){
            state.curr_value *= 10;
            state.curr_value += val;
            app.set_value(state.curr_value);
            return;
        }
        if value.as_str() == "=" {
            let result: i32 = match state.operator.as_str() {
                "+" => state.prev_value.checked_add(state.curr_value).unwrap_or(0),
                "-" => state.prev_value.checked_sub(state.curr_value).unwrap_or(0),
                "*" => state.prev_value.checked_mul(state.curr_value).unwrap_or(0),
                "/" => state.prev_value.checked_div(state.curr_value).unwrap_or(0),
                  _ => {return;},
            };
            app.set_value(result);
            state.curr_value = result;
            state.prev_value = 0;
            state.operator = Default::default();
        } else if value.as_str() == "c"{
            state.curr_value = 0;
            state.prev_value = 0;
            state.operator = Default::default();
            app.set_value(0);
        } else {
            state.operator = value.clone();
            state.prev_value = state.curr_value;
            state.curr_value = 0;
        }
    });

    app.run().unwrap();
}
