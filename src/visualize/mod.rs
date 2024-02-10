use cursive::align::Align;
use cursive::{Cursive, CursiveExt};
use cursive::view::{Nameable, Resizable};
use cursive::views::{Button, Dialog, EditView, LinearLayout, ScrollView, TextView};
use crate::parser::Parser;

/// Iterate program to next instruction and update TUI
fn step(cursive: &mut Cursive) {
    if let Some(data) = cursive.user_data::<Parser>() {
        if data.get_instruction_index() + 1 == data.get_instructions().len() {
            return;
        }
        let _ = data.step();
        let index = data.get_instruction_index();
        let output = data.get_output();
        let mut memory_string = "|".to_string();
        let mut instructions_string= data.get_instructions();
        if instructions_string.get(index - 1.. index).unwrap() != "\n" {
            instructions_string.replace_range(index - 1.. index, "▋");
        }
        for cell in &data.get_memory() {
            memory_string.push_str(&format!("{}|", cell));
        }
        cursive.call_on_name("standard_output", |standard_output: &mut TextView| {
            standard_output.set_content(output);
        });
        cursive.call_on_name("memory", |memory: &mut TextView| {
            memory.set_content(memory_string);
        });
        cursive.call_on_name("instructions", |instructions: &mut TextView| {
            instructions.set_content(instructions_string);
        });
    }
}

fn handle_count_submit(cursive: &mut Cursive, count: &str) {
    cursive.pop_layer();
    if let Ok(count) = String::from(count).parse::<i32>() {
        for _ in 0.. count {
            step(cursive);
        }
        return;
    }
    cursive.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(TextView::new("Error that's not a number!"))
                .child(Button::new("Ok", |cursive: &mut Cursive| {
                    cursive.pop_layer();
                }))
        )
    );
}

fn step_x(cursive: &mut Cursive) {
    cursive.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(TextView::new("How many times to step forward"))
                .child(EditView::new().on_submit(handle_count_submit))
        )
    )
}

/// Visualize execution of program
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
        .child(Button::new("Step One", step))
        .child(Button::new("Step X", step_x));

    let main = LinearLayout::vertical()
        .child(TextView::new("Your code:").align(Align::center()))
        .child(
            ScrollView::new(
                TextView::new(siv.user_data::<Parser>().unwrap().get_instructions())
                    .with_name("instructions")
            ).max_height(20)
        )
        .child(TextView::new("Memory:").align(Align::center()))
        .child(TextView::new("").align(Align::center()).with_name("memory"))
        .child(TextView::new("Standard Output:").align(Align::center()))
        .child(TextView::new("").align(Align::center()).with_name("standard_output"))
        .child(buttons);
    siv.add_layer(Dialog::around(LinearLayout::horizontal().child(main)));
    siv.run()
}