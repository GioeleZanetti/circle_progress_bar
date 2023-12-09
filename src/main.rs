use circle_progress_bar::{
    CircleProgressBar, CircleProgressBarEvents, CircleProgressBarInit, Color,
};
use gtk::prelude::*;
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};
use relm4::{Component, ComponentController, Controller, RelmApp};

mod circle_progress_bar;

pub struct App {
    cpb1: Controller<CircleProgressBar>,
    cpb2: Controller<CircleProgressBar>,
    cpb3: Controller<CircleProgressBar>,
    cpb4: Controller<CircleProgressBar>,
    cpb5: Controller<CircleProgressBar>,
}

#[derive(Debug)]
pub enum AppEvents {
    DoNothing,
    UpdateValues,
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Input = AppEvents;
    type Output = ();
    type Init = ();

    view! {
        #[root]
        gtk::Window{
            set_default_size: (100,600),
            set_resizable: false,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_hexpand: true,
                set_vexpand: true,

                gtk::Button{
                    set_label: "Update",
                    connect_clicked => AppEvents::UpdateValues,
                },

                gtk::Box{

                    model.cpb1.widget(),
                    model.cpb2.widget(),
                    model.cpb3.widget(),
                    model.cpb4.widget(),
                    model.cpb5.widget(),
                }
            }
        }
    }

    fn init(
        _: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let cpb1 = CircleProgressBar::builder()
            .launch(CircleProgressBarInit {
                size: 200,
                thickness: 5,
                start_at: 0,
                clockwise: true,
                value: 20.,
                label: None,
                background_color: Color::from_hex_string("#555657").unwrap(),
                percentage_color: Color::from_hex_string("#ffffff").unwrap(),
            })
            .forward(sender.input_sender(), |_| AppEvents::DoNothing);
        let cpb2 = CircleProgressBar::builder()
            .launch(CircleProgressBarInit {
                size: 200,
                thickness: 100,
                start_at: 0,
                clockwise: true,
                value: 100.,
                label: Some("100% full".to_string()),
                background_color: Color::from_hex_string("#555657").unwrap(),
                percentage_color: Color::from_hex_string("#ff0000").unwrap(),
            })
            .forward(sender.input_sender(), |_| AppEvents::DoNothing);
        let cpb3 = CircleProgressBar::builder()
            .launch(CircleProgressBarInit {
                size: 200,
                thickness: 5,
                start_at: 25,
                clockwise: true,
                value: 50.,
                label: Some("50% starting from right".to_string()),
                background_color: Color::from_hex_string("#555657").unwrap(),
                percentage_color: Color::from_hex_string("#e834bc").unwrap(),
            })
            .forward(sender.input_sender(), |_| AppEvents::DoNothing);
        let cpb4 = CircleProgressBar::builder()
            .launch(CircleProgressBarInit {
                size: 200,
                thickness: 5,
                start_at: 0,
                clockwise: false,
                value: 46.,
                label: Some("46% counter clockwise".to_string()),
                background_color: Color::from_hex_string("#555657").unwrap(),
                percentage_color: Color::from_hex_string("#008423").unwrap(),
            })
            .forward(sender.input_sender(), |_| AppEvents::DoNothing);
        let cpb5 = CircleProgressBar::builder()
            .launch(CircleProgressBarInit {
                size: 200,
                thickness: 5,
                start_at: 0,
                clockwise: false,
                value: 0.,
                label: None,
                background_color: Color::from_hex_string("#555657").unwrap(),
                percentage_color: Color::from_hex_string("#2300c1").unwrap(),
            })
            .forward(sender.input_sender(), |_| AppEvents::DoNothing);

        let model = App {
            cpb1,
            cpb2,
            cpb3,
            cpb4,
            cpb5,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppEvents::DoNothing => {}
            AppEvents::UpdateValues => {
                self.cpb1.emit(CircleProgressBarEvents::UpdateValue(64.));
                self.cpb5.emit(CircleProgressBarEvents::UpdateValue(25.));
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("org.relm4.song_widget");
    app.run::<App>(());
}
