use cursive::align::Align;
use cursive::{Cursive, CursiveExt};
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
                        data.match_current_instruction();
                        data.instruction_index += 1;
                        let index = data.instruction_index.clone();
                        cursive.call_on_name("instruction_index", |instruction_index: &mut TextView| {
                            instruction_index.set_content(format!("{}", index));
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
            .child(TextView::new(siv.user_data::<Parser>().unwrap().instructions.clone()))
            .child(TextView::new("Memory:").align(Align::center()))
            .child(TextView::new("Standard Output:").align(Align::center()))
            .child(TextView::new(format!("{}", siv.user_data::<Parser>().unwrap().instruction_index)).with_name("instruction_index"))
            .child(buttons);
        ;
        siv.add_layer(Dialog::around(LinearLayout::horizontal().child(main)));
        siv.run();
    }
}