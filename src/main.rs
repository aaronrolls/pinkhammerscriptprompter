#![allow(non_snake_case)]
use std::fs::File;
use std::io::prelude::*;
use dioxus::prelude::*;

mod the_pink_hammer;

#[derive(Debug,Clone,PartialEq)]
pub struct Line {
    text: String,
    class: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Buffer {
    current_line: usize,
    previous_line: usize,
    lines: Vec<String>,
    display: Vec<Line>,
    current_char: Option<String>,
}

enum Charatures {
    Woody,
    Annabel,
    Siobhan,
    Helen,
    Louise,
}

impl Charatures {
    fn get_text(self) -> String {
        match self {
            Charatures::Woody => String::from("WOODY"),
            Charatures::Helen => String::from("HELEN"),
            Charatures::Louise => String::from("LOUISE"),
            Charatures::Siobhan => String::from("SIOBHAN"),
            Charatures::Annabel => String::from("ANABEL"),
        }
    }
}
//impl Deref for Buffer{
//
//    type Target = Buffer;

 //   fn deref(&self) -> &Self::Target{
 //       &self
 //   }
//}


impl Buffer {
     
    fn new() -> Buffer {
        Buffer {
            current_line: 0,
            previous_line: 0,
            lines: Vec::new(),
            display: Vec::new(),
            current_char: None,
        }
    }


    fn clear(&mut self) {
        self.display = Vec::new();
        self.current_line = 0;
        self.previous_line = 0;
    }

    fn add(&mut self, line: Line) {
        if self.display.len() > 10 {
            self.display.remove(0);
        }
        self.display.push(line);
        
    }

    fn set_current_counter(&mut self, number: usize) {
       self.current_line = number;
    }

    fn set_previous_counter(&mut self, number: usize) {
        self.previous_line = number;
    }
    fn set_counter_from_page(&mut self, lines: &Vec<String>, number: usize) {
        if number < 25 {
            return
        }
        for each in 0..lines.len() {
                    if lines[each].contains((&number.to_string())) {
                        self.current_line = each;
                        return
                    }
        }
    }

    fn next(&mut self) {
        let chars: [&str;5] = ["WOODY:", "HELEN:", "SIOBHAN:", "ANABEL:", "LOUISE:"];
        if self.current_char.is_none() {
            return
        }
        let current_char = match &self.current_char {
                Some(name) => String::from(name),
                None => String::from(""),
        };

        for each in self.current_line..self.lines.len() {
            for c in chars {
               if self.lines[each].contains(c) {
                    if self.lines[each].contains(&current_char) {
                        self.set_current_counter(each);
                        self.add(Line {text: String::from(&self.lines[self.previous_line]), class: String::from("red")});
                        self.add(Line {text: String::from(&self.lines[self.current_line]), class: String::from("red")});
                    }
                    self.set_previous_counter(each);
                }
            }
        }
    }
    
    fn next_out(&mut self, lines: &Vec<String>, current_line: &usize) {

        println!("{:?}", self.display);
        let chars: [&str;5] = ["WOODY:", "HELEN:", "SIOBHAN:", "ANABEL:", "LOUISE:"];
        if self.current_char.is_none() {
            self.current_char = Some("WOODY:".to_owned());
        }
        let current_char = match &self.current_char {
                Some(name) => String::from(name),
                None => String::from(""),
        };

        for each in *current_line..lines.len() {
            for c in chars {
               if lines[each].contains(c) {
                    if lines[each].contains(&current_char) {
                        self.set_current_counter(each);
                        self.add(Line {text: String::from(&lines[self.previous_line]), class: String::from("red")});
                        self.add(Line {text: String::from(&lines[*current_line]), class: String::from("red")});
                        return
                    }
                    self.set_previous_counter(each);
                }
            }
        }
    }



}

 
fn get_current_number(lines: &Vec<String>, char: &str, current_line: &usize) -> usize {
    let mut prev_line: usize = 0;
    let next_line = *current_line + 1;
    
    let chars: [&str;5] = ["WOODY:", "HELEN:", "SIOBHAN:", "ANNABEL:", "LOUISE:"];

    for each in next_line..lines.len() {
        for c in chars {
            if lines[each].contains(c) {
                if lines[each].contains(char) {
                    return each
                }
                prev_line = each;
            }
        }
    }
    0
}


fn get_previous_number(lines: &Vec<String>, char: &str, current_line: &usize) -> Vec<usize> {
    
    let chars: [&str;6] = ["WOODY:", "HELEN:", "SIOBHAN:", "ANNABEL:", "LOUISE:", "sd:"];
    let mut counters: Vec<usize> = Vec::new();
    let mut counter = *current_line;
    counter = counter -1;
    while counter > 0 {
       for c in chars { 
            if lines[counter].contains(c) {
                if lines[counter].contains("sd:") {
                    counters.push(counter);
                } else{
                    counters.push(counter);
                    return counters
                }
            }
       }

            counter -= 1;
    }
    Vec::new()
}


fn load_file() -> Vec<String> {

    let mut s = the_pink_hammer::get_text();

    let lines: Vec<String> = s.split("\n").map(str::to_string).collect();

    lines
}


fn create_line(line: &str, color: &str) -> Line {
    let line = Line { text: String::from(line), class: String::from(color) };
    line
}

struct Char {
    charature: String,
}

impl Char {

    fn new() -> Char {
        Char { charature: String::new() }
    }

    fn set(&mut self, name: &str) {
        
        if name.contains(":") {
            self.charature = String::from(name);
        } else {
            let mut thing = String::from(name);
            thing.push(':');
            self.charature = thing;
        }
    }
}


fn App(cx: Scope) -> Element {
    let lines = load_file();
    let lines2 = lines.clone();
    let lines3 = lines.clone();
    let mut buffer = use_ref(cx, Buffer::new);
    let mut char = use_ref(cx, Char::new);

    move || {
        char.with_mut(|c| c.set("WOODY:")); 
    };
    cx.render(rsx! {
        p{ "Currently - {char.read().charature.clone()}"}
        div {
            overflow:"auto",
            buffer.read().display.iter().map(|lines| {
                rsx!{ p{ 
                        color: "{lines.class}",
                        "{lines.text}"
                }
                }
            })
            }
        div {
            position: "fixed",
            bottom: "0",
            left: "0",
            width: "100vh",
        button {
            padding: "10px",
            margin: "5px",
            onclick: move |_| {
                let count = buffer.read().current_line;
                buffer.with_mut(|buff| buff.add(create_line(&lines2[count], "green")));
            },
            "line"
        }
        button {
            padding: "10px",
            margin: "5px",
            onclick: move |_| {
                let count = get_current_number(&lines, &char.read().charature, &buffer.read().current_line);
                let pcount = get_previous_number(&lines, &char.read().charature, &count);
                //let count = buffer.read().current_line;
                let pcount: Vec<usize> = pcount.into_iter().rev().collect();
                for each in &pcount {
                    if lines[*each].contains("sd:") {
                        buffer.with_mut(|buff| buff.add(create_line(&lines[*each], "#e3aa19")));
                    } else {    
                        buffer.with_mut(|buff| buff.add(create_line(&lines[*each], "red")));
                    }
                }
                let it = match pcount.last() {
                    Some(number) => number.to_owned(),
                    None => 0,
                };
                buffer.with_mut(|buff| buff.set_current_counter(count));
                buffer.with_mut(|buff| buff.set_previous_counter(it));
            },
            "next"
        }
        p { "Goto Page Number" }
        input {
            //type: "number",
            oninput: move |evt| buffer.with_mut(|buff| buff.set_counter_from_page(&lines3, evt.value.clone().parse::<usize>().unwrap())),
        }

        div {
        button {
            padding: "10px",
            margin: "5px",
            onclick: move |_| {
                char.with_mut(|s| s.set("WOODY:"));
            },
            "WOODY"
        }
        button {
            padding: "10px",
            margin: "5px",
            onclick: move |_| {
                char.with_mut(|s| s.set("HELEN:"));
            },
            "HELEN"
        }
        button {
            padding: "10px",
            margin: "5px",
            onclick: move |_| {
                char.with_mut(|s| s.set("SIOBHAN:"));
            },
            "SIOBHAN"
        }
        button {
            padding: "10px",
            margin: "5px",
            onclick: move |_| {
                char.with_mut(|s| s.set("ANNABEL:"));
            },
            "ANNABEL"
        }
        button {
            padding: "10px",
            margin: "5px",
            onclick: move |_| {
                char.with_mut(|s| s.set("LOUISE:"));
            },
            "LOUSIE"
        }
        }

        
        }

    })
}


fn main() {
    dioxus_web::launch(App);
}
