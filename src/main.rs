use druid::{
    widget::{Flex, Label},
    AppDelegate, AppLauncher, Command, Data, DelegateCtx, Env, Lens, PlatformError, Target, Widget,
    WidgetExt, WindowDesc,
};
use druid_enums::Matcher;

mod wizard;

#[derive(Clone, Data, Lens)]
pub struct Person {
    name: String,
    age: String,
    height: String,
}

#[derive(Clone, Data, Lens)]
struct MainState {
    person: Person,
}

fn main_ui() -> impl Widget<MainState> {
    Flex::row()
        .with_child(Label::dynamic(|data: &MainState, _env| {
            format!("Name: {}", data.person.name)
        }))
        .with_child(Label::dynamic(|data: &MainState, _env| {
            format!("Age: {}", data.person.age)
        }))
        .with_child(Label::dynamic(|data: &MainState, _env| {
            format!("Height: {}", data.person.height)
        }))
        .center()
}

#[derive(Clone, Data, Matcher)]
#[matcher(matcher_name = AppMatcher)]
enum App {
    Wizard(wizard::Wizard),
    Main(MainState),
}

fn ui() -> impl Widget<App> {
    AppMatcher::new().wizard(wizard::wizard()).main(main_ui())
}

fn main() -> Result<(), PlatformError> {
    let window = WindowDesc::new(ui).title("Druid Enums");
    let state = App::Wizard(wizard::Wizard::new());
    AppLauncher::with_window(window)
        .delegate(Delegate)
        .use_simple_logger()
        .launch(state)
}

struct Delegate;

impl AppDelegate<App> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut App,
        _env: &Env,
    ) -> bool {
        println!("delegate: {:?}", cmd);
        if let Some(person) = cmd.get(wizard::DONE) {
            *data = App::Main(MainState {
                person: person.clone(),
            });
        }
        true
    }
}
