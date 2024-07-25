mod styles;
mod buttons;

use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};
use iced::{Element, Renderer, Application, Settings, Theme, Command, window, executor, Size, Length, Padding, Alignment, Color};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, text, text_input, Column, TextInput, Text, Container, Row, Button, Tooltip, tooltip};
use iced::window::close;
use rand::{Rng};
use crate::buttons::{check_button, finish_button, next_button, quit_button, restart_button, start_button};
use crate::styles::{LineFakeButton};

#[derive(Debug, Clone)]
enum Message {
    Start,
    ReadInput(String),
    Check,
    Next,
    Finish,
    Restart,
    Quit
}
impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::Start                  => {write!(f, "Start")},
            Message::ReadInput(s)   => {write!(f, "{}", s)}
            Message::Check                  => {write!(f, "Check")},
            Message::Next                   => {write!(f, "Next")},
            Message::Finish                 => {write!(f, "Finish")},
            Message::Restart                => {write!(f, "Restart")},
            Message::Quit                   => {write!(f, "Quit")},
        }
    }
}
impl Default for Message {
    fn default() -> Self {
        Message::Start
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Mode {
    Start,
    Exercising,
    Result,
    FinalEvaluation
}
impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Start             => {write!(f, "Start")},
            Mode::Exercising        => {write!(f, "Exercising")},
            Mode::Result            => {write!(f, "Result")},
            Mode::FinalEvaluation   => {write!(f, "FinalEvaluation")},
        }
    }
}
impl Default for Mode {
    fn default() -> Self {
        Mode::Start
    }
}

#[derive(Debug, Copy, Clone)]
struct State {
    mode: Mode,
    error_made: bool,
    errors_count: u32,
    exercise_count: u32,
    num_a: u32,
    den_a: u32,
    num_b: u32,
    den_b: u32,
    input_field_number: u32,
    input_number: u32,
    errors: [u32; 12],
}
impl Default for State {
    fn default() -> Self {
        State{
            mode: Mode::default(),
            error_made: false,
            errors_count: 0,
            exercise_count: 0,
            num_a: 0,
            den_a: 0,
            num_b: 0,
            den_b: 0,
            input_field_number: 0,
            input_number: 0,
            errors: [0; 12],
        }
    }
}
impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "state: {}\nerror made: {}\terror count: {}\t exercise count: {}\nnum_a:  {}\tnum_b:  {}\nden_a:  {}\tden_b:  {}\ninput field: {}\tinput number: {}\n1: {}\t  4: {}\t  7: {}\t 10: {}\n2: {}\t  5: {}\t  8: {}\t 11: {}\n3: {}\t  6: {}\t  9: {}\t 12: {}\n",
               self.mode, self.error_made, self.errors_count, self.exercise_count, self.num_a, self.num_b, self.den_a, self.den_b, self.input_field_number, self.input_number, self.errors[0], self.errors[3], self.errors[6], self.errors[9], self.errors[1], self.errors[4], self.errors[7], self.errors[10], self.errors[2], self.errors[5], self.errors[8], self.errors[11])
    }
}

impl State {
    fn get_new_numbers (&mut self) {
        let mul = rand::thread_rng().gen_range(2..11);
        let num = rand::thread_rng().gen_range(1..13);
        let den = rand::thread_rng().gen_range(1..13);

        let a_or_b = rand::thread_rng().gen_bool(0.5);

        self.num_a = if a_or_b {num} else {num*mul};
        self.den_a = if a_or_b {den} else {den*mul};
        self.num_b = if a_or_b {num*mul} else {num};
        self.den_b = if a_or_b {den*mul} else {den};
        self.input_field_number = rand::thread_rng().gen_range(0..4);
    }
    fn start (&mut self) {
        self.get_new_numbers();
        self.input_number = 0;
        self.mode = Mode::Exercising;
    }
    fn read_input(&mut self, input_string: String) {
        self.input_number = match input_string.parse() {
            Ok(number) => {if number>0 && number<1000 {number} else {0}}
            Err(_) => {0}
        };
    }
    fn evaluate (&mut self) {
        self.exercise_count += 1;
        match self.input_field_number {
            0 => {
                if self.input_number != self.num_a {
                    self.errors_count += 1;
                    self.error_made = true;
                    let mul = if self.num_a > self.num_b {self.num_a/self.num_b} else {self.num_b/self.num_a};
                    self.errors[mul as usize-1] += 1;
                    let num = if self.num_a > self.num_b {self.num_b} else {self.num_a};
                    self.errors[num as usize-1] += 1;
                }
            }
            1 => {
                if self.input_number != self.den_a {
                    self.errors_count += 1;
                    self.error_made = true;
                    let mul = if self.den_a > self.den_b {self.den_a/self.den_b} else {self.den_b/self.den_a};
                    self.errors[mul as usize-1] += 1;
                    let num = if self.den_a > self.den_b {self.den_b} else {self.den_a};
                    self.errors[num as usize-1] += 1;
                }
            }
            2 => {
                if self.input_number != self.num_b {
                    self.errors_count += 1;
                    self.error_made = true;
                    let mul = if self.num_a > self.num_b {self.num_a/self.num_b} else {self.num_b/self.num_a};
                    self.errors[mul as usize-1] += 1;
                    let num = if self.num_a > self.num_b {self.num_b} else {self.num_a};
                    self.errors[num as usize-1] += 1;
                }
            }
            3 => {
                if self.input_number != self.den_b {
                    self.errors_count += 1;
                    self.error_made = true;
                    let mul = if self.den_a > self.den_b {self.den_a/self.den_b} else {self.den_b/self.den_a};
                    self.errors[mul as usize-1] += 1;
                    let num = if self.den_a > self.den_b {self.den_b} else {self.den_a};
                    self.errors[num as usize-1] += 1;
                }
            }
            _ => {
                panic!("Error: input field different from expected [0, 1, 2, 3]")
            }
        }
        self.mode = Mode::Result;
    }
    fn next (&mut self) {
        self.get_new_numbers();
        self.input_number = 0;
        self.error_made = false;
        self.mode = Mode::Exercising;
    }
    fn finish (&mut self) {
        self.mode = Mode::FinalEvaluation;
    }
    fn restart (&mut self) {
        self.mode = Mode::Exercising;
        self.error_made = false;
        self.errors_count= 0;
        self.exercise_count= 0;
        self.num_a= 0;
        self.den_a= 0;
        self.num_b= 0;
        self.den_b= 0;
        self.input_field_number= 0;
        self.input_number= 0;
        self.errors = [0; 12];
        self.get_new_numbers();
    }
}

impl Application for State{
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }
    fn title(&self) -> String {
        String::from("PropInva")
    }
    fn update(&mut self, message: Self::Message) -> Command<Message>{
        match message {
            Message::Start                  => {
                self.start();
                //println!("Start");
                Command::none()
            }
            Message::ReadInput(input_string)    => {
                self.read_input(input_string);
                //println!("{}",self.input_number);
                Command::none()
            }
            Message::Check                  => {
                self.evaluate();
                //println!("Check");
                Command::none()
            }
            Message::Next                   => {
                self.next();
                //println!("Next");
                Command::none()
            }
            Message::Finish                 => {
                self.finish();
                //println!("Finish");
                Command::none()
            }
            Message::Restart                =>  {
                self.restart();
                //println!("Restart");
                Command::none()
            }
            Message::Quit                   => {
                //println!("Quit");
                close(window::Id::MAIN)
            }
        }
    }
    fn view(&self) -> Element<'_, Self::Message> {

        let start_button = start_button();

        let check_button = check_button();
        let check_button = if self.input_number != 0 {check_button.on_press(Message::Check)} else {check_button};

        let next_button = next_button();

        let finish_button = finish_button();

        let restart_button = restart_button();

        let quit_button = quit_button();

        let start_title: Text<'_, Theme, Renderer> = text("Exercises on Invariant Property").size(35);
        let result_labels: Text<'_, Theme, Renderer> = text("Correct:\nErrors:\nAccuracy:").size(41);
        let result_numbers: Text<'_, Theme, Renderer> = text(format!("{}\n{}\n{}%",
                                                                    self.exercise_count-self.errors_count,
                                                                    self.errors_count,
                                                                    (self.exercise_count as f32 - self.errors_count as f32) / self.exercise_count as f32 * 100.0)).size(41);

        let max_err = self.errors.iter().max().unwrap();
        let repeated_err = *max_err >= 2;

        let focus_text: Text<'_, Theme, Renderer> = text(format!("You should focus on the {} times table", max_err)).size(21);

        let equal_sign = text("=").size(50);
        let inserted = text(format!("Inserted: {}", self.input_number.to_string())).size(30).style(Color::from_rgb(0.6, 0.8, 1.0));

        let numerator_a =
            if self.mode == Mode::Result && self.input_field_number == 0 {
                if self.error_made {
                    text(self.num_a.to_string()).size(46).style(Color::from_rgb(0.9, 0.35, 0.26))
                } else {
                    text(self.num_a.to_string()).size(46).style(Color::from_rgb(0.4, 0.8, 0.0))
                }
            }
            else {
                text(self.num_a.to_string()).size(46)
            };

        let denominator_a =
            if self.mode == Mode::Result && self.input_field_number == 1 {
                if self.error_made {
                    text(self.den_a.to_string()).size(46).style(Color::from_rgb(0.9, 0.35, 0.26))
                } else {
                    text(self.den_a.to_string()).size(46).style(Color::from_rgb(0.4, 0.8, 0.0))
                }
            }
            else {
                text(self.den_a.to_string()).size(46)
            };

        let numerator_b =
            if self.mode == Mode::Result && self.input_field_number == 2 {
                if self.error_made {
                    text(self.num_b.to_string()).size(46).style(Color::from_rgb(0.9, 0.35, 0.26))
                }
                else {
                    text(self.num_b.to_string()).size(46).style(Color::from_rgb(0.4, 0.8, 0.0))
                }
            }
            else {
                text(self.num_b.to_string()).size(46)
            };

        let denominator_b =
            if self.mode == Mode::Result && self.input_field_number == 3 {
                if self.error_made {
                    text(self.den_b.to_string()).size(46).style(Color::from_rgb(0.9, 0.35, 0.26))
                } else {
                    text(self.den_b.to_string()).size(46).style(Color::from_rgb(0.4, 0.8, 0.0))
                }
            }
            else {
                text(self.den_b.to_string()).size(46)
            };

        let line_a: Button<'_, Message, Theme, Renderer> = button("")
            .style(LineFakeButton::new())
            .width(Length::Fixed(120.0))
            .height(Length::Fixed(2.0));
        let line_b: Button<'_, Message, Theme, Renderer> = button("")
            .style(LineFakeButton::new())
            .width(Length::Fixed(100.0))
            .height(Length::Fixed(2.0));

        let displayed_value = if self.input_number==0 {""} else {&self.input_number.to_string()};
        let input_field : TextInput<'_, Message, Theme, Renderer> = text_input("", displayed_value)
            .size(38.4)
            .width(Length::Fixed(100.0))
            .on_input(Message::ReadInput)
            .on_paste(Message::ReadInput)
            .on_submit(Message::Check);

        match self.mode{
            Mode::Start => {
                Column::new()
                    .width(Length::Fill)
                    .align_items(Alignment::Center)
                    .push(Container::new(start_title).padding(Padding{top: 60.0, right: 0.0, bottom: 0.0, left: 0.0}))
                    .push(Container::new(start_button).padding(Padding{top: 50.0, right: 0.0, bottom: 0.0, left: 0.0}))
                    .push(Container::new(quit_button).padding(Padding{top: 50.0, right: 0.0, bottom: 0.0, left: 0.0}))
                    .into()
            }
            Mode::Exercising => {
                match self.input_field_number {
                    0 => {
                        Column::new()
                        .push(Container::new(Row::new()
                            .push(Container::new(Column::new()
                                    .push(input_field)
                                    .push(Container::new(line_a).padding(Padding{top: 20.0, right: 0.0, bottom: 20.0, left: 0.0}))
                                    .push(denominator_a)
                                    .align_items(Alignment::Center)
                                ).height(Length::Fill)
                                .align_y(Vertical::Center)
                                .align_x(Horizontal::Right)
                            )
                            .push(Container::new(equal_sign)
                                .padding(Padding{top: 0.0, right: 50.0, bottom: 00.0, left: 50.0})
                                .height(Length::Fill)
                                .center_x()
                                .center_y()
                            )
                            .push(Container::new(Column::new()
                                    .push(numerator_b)
                                    .push(Container::new(line_b).padding(Padding{top: 20.0, right: 0.0, bottom: 20.0, left: 0.0}))
                                    .push(denominator_b)
                                    .align_items(Alignment::Center)
                                ).height(Length::Fill)
                                .align_y(Vertical::Center)
                                .align_x(Horizontal::Left)
                            ))
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .align_x(Horizontal::Center)
                            .align_y(Vertical::Center)
                        )
                        .push(Container::new(check_button)
                            .width(Length::Fill)
                            .padding(Padding{top: 0.0, right: 0.0, bottom: 30.0, left: 450.0})
                        )
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .align_items(Alignment::Center)
                        .into()
                    }
                    1 => {
                        Column::new()
                        .push(Container::new(Row::new()
                            .push(Container::new(Column::new()
                                    .push(numerator_a)
                                    .push(Container::new(line_a).padding(Padding{top: 20.0, right: 0.0, bottom: 20.0, left: 0.0}))
                                    .push(input_field)
                                    .align_items(Alignment::Center)
                                ).height(Length::Fill)
                                .align_y(Vertical::Center)
                                .align_x(Horizontal::Right)
                            )
                            .push(Container::new(equal_sign)
                                .padding(Padding{top: 0.0, right: 50.0, bottom: 00.0, left: 50.0})
                                .height(Length::Fill)
                                .center_x()
                                .center_y()
                            )
                            .push(Container::new(Column::new()
                                    .push(numerator_b)
                                    .push(Container::new(line_b).padding(Padding{top: 20.0, right: 0.0, bottom: 20.0, left: 0.0}))
                                    .push(denominator_b)
                                    .align_items(Alignment::Center)
                                ).height(Length::Fill)
                                .align_y(Vertical::Center)
                                .align_x(Horizontal::Left)
                            ))
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .align_x(Horizontal::Center)
                            .align_y(Vertical::Center)
                        )
                        .push(Container::new(check_button)
                            .width(Length::Fill)
                            .padding(Padding{top: 0.0, right: 0.0, bottom: 30.0, left: 450.0})
                        )
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .align_items(Alignment::Center)
                        .into()
                    }
                    2 => {
                        Column::new()
                        .push(Container::new(Row::new()
                            .push(Container::new(Column::new()
                                    .push(numerator_a)
                                    .push(Container::new(line_a).padding(Padding{top: 20.0, right: 0.0, bottom: 20.0, left: 0.0}))
                                    .push(denominator_a)
                                    .align_items(Alignment::Center)
                                ).height(Length::Fill)
                                .align_y(Vertical::Center)
                                .align_x(Horizontal::Right)
                            )
                            .push(Container::new(equal_sign)
                                .padding(Padding{top: 0.0, right: 50.0, bottom: 00.0, left: 50.0})
                                .height(Length::Fill)
                                .center_x()
                                .center_y()
                            )
                            .push(Container::new(Column::new()
                                    .push(input_field)
                                    .push(Container::new(line_b).padding(Padding{top: 20.0, right: 0.0, bottom: 20.0, left: 0.0}))
                                    .push(denominator_b)
                                    .align_items(Alignment::Center)
                                ).height(Length::Fill)
                                .align_y(Vertical::Center)
                                .align_x(Horizontal::Left)
                            ))
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .align_x(Horizontal::Center)
                            .align_y(Vertical::Center)
                        )
                        .push(Container::new(check_button)
                            .width(Length::Fill)
                            .padding(Padding{top: 0.0, right: 0.0, bottom: 30.0, left: 450.0})
                        )
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .align_items(Alignment::Center)
                        .into()
                    }
                    3 => {
                        Column::new()
                        .push(Container::new(Row::new()
                            .push(Container::new(Column::new()
                                    .push(numerator_a)
                                    .push(Container::new(line_a).padding(Padding{top: 20.0, right: 0.0, bottom: 20.0, left: 0.0}))
                                    .push(denominator_a)
                                    .align_items(Alignment::Center)
                                ).height(Length::Fill)
                                .align_y(Vertical::Center)
                                .align_x(Horizontal::Right)
                            )
                            .push(Container::new(equal_sign)
                                .padding(Padding{top: 0.0, right: 50.0, bottom: 00.0, left: 50.0})
                                .height(Length::Fill)
                                .center_x()
                                .center_y()
                            )
                            .push(Container::new(Column::new()
                                    .push(numerator_b)
                                    .push(Container::new(line_b).padding(Padding{top: 20.0, right: 0.0, bottom: 20.0, left: 0.0}))
                                    .push(input_field)
                                    .align_items(Alignment::Center)
                                ).height(Length::Fill)
                                .align_y(Vertical::Center)
                                .align_x(Horizontal::Left)
                            ))
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .align_x(Horizontal::Center)
                            .align_y(Vertical::Center)
                        )
                        .push(Container::new(check_button)
                            .width(Length::Fill)
                            .padding(Padding{top: 0.0, right: 0.0, bottom: 30.0, left: 450.0})
                        )
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .align_items(Alignment::Center)
                        .into()
                    }
                    _ => {
                        panic!("Error: input field different from expected [0, 1, 2, 3]");
                        // Column::new().push(text("Error: input field different from expected [0, 1, 2, 3]").size(35)).into()
                    }
                }
            }
            Mode::Result => {
                match self.input_field_number {
                    0 => {
                        Column::new()
                            .push(Container::new(Row::new()
                                .push(Container::new(Column::new()
                                        .push(Tooltip::new(numerator_a, inserted, tooltip::Position::Top))
                                        .push(Container::new(line_a).padding(Padding{top: 20.0, right: 0.0, bottom: 20.0, left: 0.0}))
                                        .push(denominator_a)
                                        .align_items(Alignment::Center)
                                    ).height(Length::Fill)
                                    .align_y(Vertical::Center)
                                    .align_x(Horizontal::Right)
                                )
                                .push(Container::new(equal_sign)
                                    .padding(Padding{top: 0.0, right: 50.0, bottom: 00.0, left: 50.0})
                                    .height(Length::Fill)
                                    .center_x()
                                    .center_y()
                                )
                                .push(Container::new(Column::new()
                                        .push(numerator_b)
                                        .push(Container::new(line_b).padding(Padding{top: 20.0, right: 0.0, bottom: 20.0, left: 0.0}))
                                        .push(denominator_b)
                                        .align_items(Alignment::Center)
                                    ).height(Length::Fill)
                                    .align_y(Vertical::Center)
                                    .align_x(Horizontal::Left)
                                ))
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .align_x(Horizontal::Center)
                                .align_y(Vertical::Center)
                            )
                            .push(Container::new(if self.exercise_count <20 {next_button} else {finish_button})
                                .width(Length::Fill)
                                .padding(Padding{top: 0.0, right: 0.0, bottom: 30.0, left: 450.0})
                            )
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .align_items(Alignment::Center)
                            .into()
                    }
                    1 => {
                        Column::new()
                            .push(Container::new(Row::new()
                                .push(Container::new(Column::new()
                                        .push(numerator_a)
                                        .push(Container::new(line_a).padding(Padding{top: 20.0, right: 0.0, bottom: 20.0, left: 0.0}))
                                        .push(Tooltip::new(denominator_a, inserted, tooltip::Position::Bottom))
                                        .align_items(Alignment::Center)
                                    ).height(Length::Fill)
                                    .align_y(Vertical::Center)
                                    .align_x(Horizontal::Right)
                                )
                                .push(Container::new(equal_sign)
                                    .padding(Padding{top: 0.0, right: 50.0, bottom: 00.0, left: 50.0})
                                    .height(Length::Fill)
                                    .center_x()
                                    .center_y()
                                )
                                .push(Container::new(Column::new()
                                        .push(numerator_b)
                                        .push(Container::new(line_b).padding(Padding{top: 20.0, right: 0.0, bottom: 20.0, left: 0.0}))
                                        .push(denominator_b)
                                        .align_items(Alignment::Center)
                                    ).height(Length::Fill)
                                    .align_y(Vertical::Center)
                                    .align_x(Horizontal::Left)
                                ))
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .align_x(Horizontal::Center)
                                .align_y(Vertical::Center)
                            )
                            .push(Container::new(if self.exercise_count <20 {next_button} else {finish_button})
                                .width(Length::Fill)
                                .padding(Padding{top: 0.0, right: 0.0, bottom: 30.0, left: 450.0})
                            )
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .align_items(Alignment::Center)
                            .into()
                    }
                    2 => {
                        Column::new()
                            .push(Container::new(Row::new()
                                .push(Container::new(Column::new()
                                        .push(numerator_a)
                                        .push(Container::new(line_a).padding(Padding{top: 20.0, right: 0.0, bottom: 20.0, left: 0.0}))
                                        .push(denominator_a)
                                        .align_items(Alignment::Center)
                                    ).height(Length::Fill)
                                    .align_y(Vertical::Center)
                                    .align_x(Horizontal::Right)
                                )
                                .push(Container::new(equal_sign)
                                    .padding(Padding{top: 0.0, right: 50.0, bottom: 00.0, left: 50.0})
                                    .height(Length::Fill)
                                    .center_x()
                                    .center_y()
                                )
                                .push(Container::new(Column::new()
                                        .push(Tooltip::new(numerator_b, inserted, tooltip::Position::Top))
                                        .push(Container::new(line_b).padding(Padding{top: 20.0, right: 0.0, bottom: 20.0, left: 0.0}))
                                        .push(denominator_b)
                                        .align_items(Alignment::Center)
                                    ).height(Length::Fill)
                                    .align_y(Vertical::Center)
                                    .align_x(Horizontal::Left)
                                ))
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .align_x(Horizontal::Center)
                                .align_y(Vertical::Center)
                            )
                            .push(Container::new(if self.exercise_count <20 {next_button} else {finish_button})
                                .width(Length::Fill)
                                .padding(Padding{top: 0.0, right: 0.0, bottom: 30.0, left: 450.0})
                            )
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .align_items(Alignment::Center)
                            .into()
                    }
                    3 => {
                        Column::new()
                            .push(Container::new(Row::new()
                                .push(Container::new(Column::new()
                                        .push(numerator_a)
                                        .push(Container::new(line_a).padding(Padding{top: 20.0, right: 0.0, bottom: 20.0, left: 0.0}))
                                        .push(denominator_a)
                                        .align_items(Alignment::Center)
                                    ).height(Length::Fill)
                                    .align_y(Vertical::Center)
                                    .align_x(Horizontal::Right)
                                )
                                .push(Container::new(equal_sign)
                                    .padding(Padding{top: 0.0, right: 50.0, bottom: 00.0, left: 50.0})
                                    .height(Length::Fill)
                                    .center_x()
                                    .center_y()
                                )
                                .push(Container::new(Column::new()
                                        .push(numerator_b)
                                        .push(Container::new(line_b).padding(Padding{top: 20.0, right: 0.0, bottom: 20.0, left: 0.0}))
                                        .push(Tooltip::new(denominator_b, inserted, tooltip::Position::Bottom))
                                        .align_items(Alignment::Center)
                                    ).height(Length::Fill)
                                    .align_y(Vertical::Center)
                                    .align_x(Horizontal::Left)
                                ))
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .align_x(Horizontal::Center)
                                .align_y(Vertical::Center)
                            )
                            .push(Container::new(if self.exercise_count <20 {next_button} else {finish_button})
                                .width(Length::Fill)
                                .padding(Padding{top: 0.0, right: 0.0, bottom: 30.0, left: 450.0})
                            )
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .align_items(Alignment::Center)
                            .into()
                    }
                    _ => {
                        panic!("Error: input field different from expected [0, 1, 2, 3]");
                    }
                }
            }
            Mode::FinalEvaluation => {

                if repeated_err {
                    Column::new()
                        .push(
                            Container::new(
                                Column::new()
                                    .push(Container::new(
                                        Row::new()
                                            .push(Container::new(result_labels).align_x(Horizontal::Left).padding(Padding{top: 0.0, right: 50.0, bottom: 0.0, left: 0.0}))
                                            .push(Container::new(result_numbers).align_x(Horizontal::Right))
                                    ).padding(Padding{top: 0.0, right: 0.0, bottom: 30.0, left: 0.0}))
                                    .push(Container::new(focus_text).padding(Padding{top: 0.0, right: 0.0, bottom: 0.0, left: 0.0}))
                            )
                            .align_y(Vertical::Center)
                            .align_x(Horizontal::Center)
                            .width(Length::Fill)
                            .height(Length::Fill)
                        )
                        .push(
                            Container::new(Row::new()
                                .push(Container::new(quit_button)
                                    .padding(Padding{top: 0.0, right: 0.0, bottom: 30.0, left: 50.0})
                                )
                                .push(Container::new(restart_button)
                                    .padding(Padding{top: 0.0, right: 0.0, bottom: 30.0, left: 280.0})
                                )
                            ).align_x(Horizontal::Left)
                            .width(Length::Fill)
                        )
                        .height(Length::Fill)
                        .width(Length::Fill)
                        .into()
                }
                else {
                    Column::new()
                        .push(
                            Container::new(
                                Column::new()
                                    .push(Container::new(
                                        Row::new()
                                            .push(Container::new(result_labels).align_x(Horizontal::Left).padding(Padding{top: 0.0, right: 50.0, bottom: 0.0, left: 0.0}))
                                            .push(Container::new(result_numbers).align_x(Horizontal::Right))
                                    ).padding(Padding{top: 0.0, right: 0.0, bottom: 30.0, left: 0.0}))
                            )
                            .align_y(Vertical::Center)
                            .align_x(Horizontal::Center)
                            .width(Length::Fill)
                            .height(Length::Fill)
                        )
                        .push(
                            Container::new(Row::new()
                                .push(Container::new(quit_button)
                                    .padding(Padding{top: 0.0, right: 0.0, bottom: 30.0, left: 50.0})
                                )
                                .push(Container::new(restart_button)
                                    .padding(Padding{top: 0.0, right: 0.0, bottom: 30.0, left: 280.0})
                                )
                            ).align_x(Horizontal::Left)
                            .width(Length::Fill)
                        )
                        .height(Length::Fill)
                        .width(Length::Fill)
                        .into()
                }
            }
        }

    }
    fn theme(&self) -> Self::Theme {
        Theme::Light
    }
}

fn main() -> iced::Result {
    let settings = Settings{
        id: None,
        window: window::settings::Settings{
            size: Size { width: 600.0, height: 400.0 },
            position: window::Position::Centered,
            resizable: false,
            ..Default::default()
        },
        antialiasing: true,
        ..Default::default()
    };

    State::run(settings)
}
