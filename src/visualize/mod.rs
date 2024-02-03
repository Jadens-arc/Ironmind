use cursive::align::Align;
use cursive::{Cursive, CursiveExt};
use cursive::backends::curses::n::ncurses::ll::instr;
use cursive::view::Nameable;
use crate::Machine;
use cursive::views::{Button, Dialog, LinearLayout, TextView};
use crate::parser::Parser;
pub struct Visualizer {
}

impl Visualizer {
    pub fn visualize (parser: Parser) {
        let mut siv = Cursive::default();
        siv.add_global_callback('q',|s| s.quit());
        siv.set_user_data(parser);
        let buttons = LinearLayout::horizontal()
            .child(Button::new(
                "Quit",
                |cursive| {
                    cursive.quit();
                }
            ))
            .child(Button::new(
                "Step One",
                |cursive| {
                    if let Some(data) = cursive.user_data::<Parser>() {
                        let _ = data.match_current_instruction();
                        data.instruction_index += 1;
                        let index = data.instruction_index.clone();
                        let output = data.get_output().clone();
                        let mut memory_string = "|".to_string();
                        for cell in &data.machine.tape {
                            memory_string.push_str(&format!("{}|", cell));
                        }
                        cursive.call_on_name("instruction_index", |instruction_index: &mut TextView| {
                            instruction_index.set_content(format!("{}", index));
                        });
                        cursive.call_on_name("standard_output", |standard_output: &mut TextView| {
                            standard_output.set_content(output);
                        });
                        cursive.call_on_name("memory", |memory: &mut TextView| {
                            memory.set_content(memory_string);
                        });
                    }
                }
            ))
            .child(Button::new(
                "Step All",
                |cursive| ()
            ));

        let main = LinearLayout::vertical()
            .child(TextView::new("Your code:").align(Align::center()))
            .child(TextView::new(siv.user_data::<Parser>().unwrap().instructions.clone()).with_name("instructions"))
            .child(TextView::new("Memory:").align(Align::center()))
            .child(TextView::new("").align(Align::center()).with_name("memory"))
            .child(TextView::new("Standard Output:").align(Align::center()))
            .child(TextView::new("").align(Align::center()).with_name("standard_output"))
            .child(TextView::new(format!("{}", siv.user_data::<Parser>().unwrap().instruction_index)).with_name("instruction_index"))
            .child(buttons);
        ;
        siv.add_layer(Dialog::around(LinearLayout::horizontal().child(main)));
        siv.run();
    }
}