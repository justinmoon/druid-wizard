use druid::{
    widget::{Button, Controller, Flex, Label, TextBox, ViewSwitcher},
    Data, Env, Event, EventCtx, Lens, Selector, Widget, WidgetExt,
};
use druid_enums::Matcher;

use crate::Person;

pub const NEXT: Selector = Selector::new("wizard.next");
pub const BACK: Selector = Selector::new("wizard.back");
pub const FINISH: Selector<Person> = Selector::new("wizard.done");
pub const QUIT: Selector = Selector::new("wizard.quit");

macro_rules! wizard_next {
    ( $struct:ty ) => {
        |ctx: &mut EventCtx, state: &mut $struct, _: &Env| {
            if state.done() {
                ctx.submit_command(NEXT, None)
            }
        }
    };
}

macro_rules! wizard_quit {
    ( $struct:ty ) => {
        |ctx: &mut EventCtx, _state: &mut $struct, _: &Env| ctx.submit_command(QUIT, None)
    };
}

macro_rules! wizard_back {
    ( $struct:ty ) => {
        |ctx: &mut EventCtx, _state: &mut $struct, _: &Env| ctx.submit_command(BACK, None)
    };
}

#[derive(Clone, Data, Lens, Default)]
pub struct NameState {
    name: String,
}

impl NameState {
    fn done(&self) -> bool {
        self.name.len() > 0
    }
}

fn name_ui() -> impl Widget<NameState> {
    // Proof-of-concept that we can disable buttons based on step completion ...
    // Not going to implement for each step because it's jank
    let next_view_switcher = ViewSwitcher::new(
        |data: &NameState, _env| data.done(),
        |done, _data, _env| match done {
            true => Box::new(Button::new("Next").on_click(wizard_next!(NameState))),
            false => Box::new(Label::new("Next")),
        },
    );

    Flex::row()
        .with_child(Label::new("Name"))
        .with_child(TextBox::new().lens(NameState::name))
        .with_spacer(5.0)
        //.with_child(Button::new("Next").on_click(next))
        .with_child(next_view_switcher)
        .with_child(Button::new("Quit").on_click(wizard_quit!(NameState)))
        .center()
}

impl From<AgeState> for NameState {
    fn from(age_state: AgeState) -> Self {
        Self {
            name: age_state.name,
        }
    }
}

impl From<NameState> for AgeState {
    fn from(name_state: NameState) -> Self {
        Self {
            name: name_state.name,
            age: "".to_string(),
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct AgeState {
    name: String,
    age: String,
}

impl AgeState {
    fn done(&self) -> bool {
        self.age.len() > 0
    }
}

fn age_ui() -> impl Widget<AgeState> {
    Flex::row()
        .with_child(Label::new("Age"))
        .with_child(TextBox::new().lens(AgeState::age))
        .with_spacer(5.0)
        .with_child(Button::new("Back").on_click(wizard_back!(AgeState)))
        .with_child(Button::new("Next").on_click(wizard_next!(AgeState)))
        .with_child(Button::new("Quit").on_click(wizard_quit!(AgeState)))
        .center()
}

impl From<AgeState> for HeightState {
    fn from(age_state: AgeState) -> Self {
        Self {
            name: age_state.name,
            age: age_state.age,
            height: "".to_string(),
        }
    }
}

impl From<HeightState> for AgeState {
    fn from(height_state: HeightState) -> Self {
        Self {
            name: height_state.name,
            age: height_state.age,
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct HeightState {
    pub name: String,
    pub age: String,
    pub height: String,
}

impl HeightState {
    fn done(&self) -> bool {
        self.height.len() > 0
    }
}

impl From<HeightState> for Person {
    fn from(height_state: HeightState) -> Self {
        Self {
            name: height_state.name,
            age: height_state.age,
            height: height_state.height,
        }
    }
}

fn height_ui() -> impl Widget<HeightState> {
    Flex::row()
        .with_child(Label::new("Height"))
        .with_child(TextBox::new().lens(HeightState::height))
        .with_spacer(5.0)
        .with_child(Button::new("Back").on_click(wizard_back!(HeightState)))
        .with_child(Button::new("Done").on_click(wizard_next!(HeightState)))
        .with_child(Button::new("Quit").on_click(wizard_quit!(HeightState)))
        .center()
}

#[derive(Clone, Data, Matcher)]
#[matcher(matcher_name = WizardMatcher)]
pub enum Wizard {
    Name(NameState),
    Age(AgeState),
    Height(HeightState),
}

impl Wizard {
    pub fn new() -> Self {
        Self::Name(NameState {
            name: "".to_string(),
        })
    }
    fn next(&mut self, ctx: &mut EventCtx) {
        match self {
            Wizard::Name(name_state) => *self = Wizard::Age(AgeState::from(name_state.clone())),
            Wizard::Age(age_state) => *self = Wizard::Height(HeightState::from(age_state.clone())),
            Wizard::Height(height_state) => {
                ctx.submit_command(FINISH.with(Person::from(height_state.clone())), None)
            }
        }
    }
    fn back(&mut self) {
        match self {
            Wizard::Name(_) => {}
            Wizard::Age(age_state) => *self = Wizard::Name(NameState::from(age_state.clone())),
            Wizard::Height(height_state) => {
                *self = Wizard::Age(AgeState::from(height_state.clone()))
            }
        }
    }
}

struct WizardController;

impl Controller<Wizard, WizardMatcher> for WizardController {
    fn event(
        &mut self,
        child: &mut WizardMatcher,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut Wizard,
        env: &Env,
    ) {
        match event {
            Event::Command(cmd) if cmd.is(NEXT) => data.next(ctx),
            Event::Command(cmd) if cmd.is(BACK) => data.back(),
            _ => {}
        }
        child.event(ctx, event, data, env)
    }
}

pub fn wizard() -> impl Widget<Wizard> {
    // Wizard::matcher() or
    WizardMatcher::new()
        .age(age_ui())
        .name(name_ui())
        .height(height_ui())
        .controller(WizardController)
}
