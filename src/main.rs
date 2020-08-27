use druid::{
    widget::{Button, Controller, Flex, Label, TextBox},
    AppLauncher, Data, Env, Event, EventCtx, Lens, PlatformError, Selector, Widget, WidgetExt,
    WindowDesc,
};
use druid_enums::Matcher;

const NEXT: Selector = Selector::new("wizard.next");
const BACK: Selector = Selector::new("wizard.back");

#[derive(Clone, Data, Matcher)]
#[matcher(matcher_name = App)] // defaults to WizardMatcher
enum Wizard {
    Name(NameState),
    Age(AgeState),
}

#[derive(Clone, Data, Lens, Default)]
struct NameState {
    name: String,
}

impl NameState {
    fn done(&self) -> bool {
        self.name.len() > 0
    }
}

#[derive(Clone, Data, Lens)]
struct AgeState {
    name: String,
    age: String,
}

impl AgeState {
    fn done(&self) -> bool {
        self.age.len() > 0
    }
}

fn main() -> Result<(), PlatformError> {
    let window = WindowDesc::new(ui).title("Druid Enums");
    let state = Wizard::Name(NameState::default());
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(state)
}

fn ui() -> impl Widget<Wizard> {
    // Wizard::matcher() or
    App::new()
        .age(age_ui())
        .name(name_ui())
        .controller(WizardController)
}

fn name_ui() -> impl Widget<NameState> {
    fn next(ctx: &mut EventCtx, state: &mut NameState, _: &Env) {
        if state.done() {
            ctx.submit_command(NEXT, None)
        }
    }

    Flex::row()
        .with_child(Label::new("Name"))
        .with_child(TextBox::new().lens(NameState::name))
        .with_spacer(5.0)
        .with_child(Button::new("Next").on_click(next))
        .center()
}

fn age_ui() -> impl Widget<AgeState> {
    fn next(ctx: &mut EventCtx, state: &mut AgeState, _: &Env) {
        if state.done() {
            ctx.submit_command(NEXT, None)
        }
    }
    Flex::row()
        .with_child(Label::new("Age"))
        .with_child(TextBox::new().lens(AgeState::age))
        .with_spacer(5.0)
        .with_child(Button::new("Next").on_click(next))
        .center()
}

struct WizardController;
impl Controller<Wizard, App> for WizardController {
    fn event(
        &mut self,
        child: &mut App,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut Wizard,
        env: &Env,
    ) {
        match event {
            Event::Command(cmd) if cmd.is(NEXT) => data.next(),
            _ => {}
        }
        child.event(ctx, event, data, env)
    }
}

impl Wizard {
    fn next(&mut self) {
        match self {
            Wizard::Name(name_state) => *self = Wizard::Age(AgeState::from(name_state.clone())),
            Wizard::Age(age_state) => println!("done"),
        }
    }
}

impl From<NameState> for AgeState {
    fn from(name_state: NameState) -> Self {
        AgeState {
            name: name_state.name,
            age: "".to_string(),
        }
    }
}


