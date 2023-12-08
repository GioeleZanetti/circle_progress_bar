use anyhow::anyhow;
use gtk::{cairo, prelude::*};
use relm4::drawing::DrawHandler;
use relm4::{gtk, ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};

pub struct CircleProgressBar {
    handler: DrawHandler,
    start_at: i64,
    value: f64,
    thickness: i64,
    clockwise: bool,
    size: i32,
    percentage_color: Color,
    background_color: Color,
    label: String,
    use_label: bool,
}

#[derive(Debug)]
pub enum CircleProgressBarEvents {
    Draw,
    UpdateValue(f64),
}

pub struct CircleProgressBarInit {
    pub start_at: i64,
    pub value: f64,
    pub thickness: i64,
    pub clockwise: bool,
    pub size: i32,
    pub percentage_color: Color,
    pub background_color: Color,
    pub label: Option<String>,
}

#[relm4::component(pub)]
impl SimpleComponent for CircleProgressBar {
    type Input = CircleProgressBarEvents;
    type Output = ();
    type Init = CircleProgressBarInit;

    view! {
        #[root]
        gtk::Box{
            set_margin_all: 10,

            gtk::Overlay {
                set_width_request: 200,
                set_height_request: 200,
                set_halign: gtk::Align::Center,
                set_valign: gtk::Align::Center,

                #[local_ref]
                area -> gtk::DrawingArea {
                    set_vexpand: false,
                    set_hexpand: false,
                    set_width_request: model.size,
                    set_height_request: model.size,
                    set_halign: gtk::Align::Center,
                    set_valign: gtk::Align::Center,
                    connect_resize[sender] => move |_, _, _| {
                        sender.input(CircleProgressBarEvents::Draw);
                    },

                },

                add_overlay = if model.use_label {
                        &gtk::Label{
                            set_vexpand: false,
                            set_hexpand: false,
                            set_halign: gtk::Align::Center,
                            set_valign: gtk::Align::Center,

                            set_label: &model.label,
                        }
                    } else {
                        &gtk::Label{
                            set_vexpand: false,
                            set_hexpand: false,
                            set_halign: gtk::Align::Center,
                            set_valign: gtk::Align::Center,

                            #[watch]
                            set_label: &format!("{}%", model.value),
                        }
                    }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = CircleProgressBar {
            size: init.size,
            start_at: init.start_at - 25,
            value: init.value,
            thickness: init.thickness,
            clockwise: init.clockwise,
            percentage_color: init.percentage_color,
            background_color: init.background_color,
            label: {
                if init.label.is_some() {
                    init.label.clone().unwrap()
                } else {
                    "".to_string()
                }
            },
            use_label: init.label.is_some(),
            handler: DrawHandler::new(),
        };
        let area = model.handler.drawing_area();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: CircleProgressBarEvents, _sender: ComponentSender<Self>) {
        let cx = self.handler.get_context();
        match msg {
            CircleProgressBarEvents::UpdateValue(new_value) => {
                if self.value <= 100. {
                    self.value = new_value;
                }
            }
            _ => {}
        }
        draw(
            &cx,
            self.clockwise,
            self.size as f64,
            self.thickness as f64,
            self.start_at as f64,
            self.value,
            &self.percentage_color,
            &self.background_color,
        );
    }
}

fn percentage_to_radiant(n: f64) -> f64 {
    (n / 100f64) * 2f64 * std::f64::consts::PI
}

fn draw(
    cr: &cairo::Context,
    clockwise: bool,
    size: f64,
    thickness: f64,
    start_at: f64,
    value: f64,
    percentage_color: &Color,
    background_color: &Color,
) {
    let start: f64;
    let end: f64;
    if clockwise {
        start = 0.0;
        end = percentage_to_radiant(value as f64);
    } else {
        start = percentage_to_radiant(100.0 - value as f64);
        end = 2. * std::f64::consts::PI;
    };
    let center = (size / 2., size / 2.);
    let outer_circle = size / 2.;
    let inner_circle = size / 2. - thickness;
    cr.save().unwrap();
    cr.translate(center.0, center.1);
    cr.rotate(percentage_to_radiant(start_at));
    cr.translate(-center.0, -center.1);
    cr.move_to(center.0, center.1);
    cr.arc(
        center.0,
        center.1,
        outer_circle,
        0.0,
        percentage_to_radiant(100.0),
    );
    cr.set_source_rgb(
        background_color.r as f64,
        background_color.g as f64,
        background_color.b as f64,
    );
    cr.move_to(center.0, center.1);
    cr.arc(
        center.0,
        center.1,
        inner_circle,
        0.0,
        percentage_to_radiant(100.0),
    );
    cr.set_fill_rule(cairo::FillRule::EvenOdd);
    cr.fill().unwrap();
    cr.move_to(center.0, center.1);
    cr.arc(center.0, center.1, outer_circle, start, end);
    cr.set_source_rgb(
        percentage_color.r as f64,
        percentage_color.g as f64,
        percentage_color.b as f64,
    );
    cr.move_to(center.0, center.1);
    cr.arc(center.0, center.1, inner_circle, start, end);
    cr.set_fill_rule(cairo::FillRule::EvenOdd);
    cr.fill().unwrap();
    cr.restore().unwrap();
}

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn from_hex_string(hex: &str) -> anyhow::Result<Color> {
        if !hex.starts_with("#") || hex.len() != 7 {
            return Err(anyhow!("Error in hex format"));
        };
        let r = i64::from_str_radix(&hex[1..3], 16)? as f64 / 255.;
        let g = i64::from_str_radix(&hex[3..5], 16)? as f64 / 255.;
        let b = i64::from_str_radix(&hex[5..7], 16)? as f64 / 255.;
        Ok(Color { r, g, b })
    }
}
