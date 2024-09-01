use iced::{
    mouse,
    widget::{
        canvas::{Fill, Frame, Geometry, Path, Program, Stroke, Text},
        column, Canvas,
    },
    Alignment, Color, Length, Point, Rectangle, Renderer, Sandbox, Settings, Size, Theme, Vector,
};

fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

struct MyApp;

impl Sandbox for MyApp {
    type Message = ();

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("gdraw")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> iced::Element<Self::Message> {
        column![
            "Look at this graph",
            Canvas::new(RelationGraph {
                toto: "Coucou".into()
            })
            .width(Length::Fill)
            .height(Length::Fill)
        ]
        .align_items(Alignment::Center)
        .into()
    }
}

struct RelationGraph {
    toto: String,
}

impl<Message> Program<Message> for RelationGraph {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        frame.fill(
            &Path::circle(frame.center(), frame.width().min(frame.height()) / 4.0),
            Color::from_rgb(0.6, 0.8, 1.0),
        );

        frame.stroke(
            &Path::line(
                frame.center() + Vector::new(-250.0, 100.0),
                frame.center() + Vector::new(250.0, -100.0),
            ),
            Stroke {
                style: Color::WHITE.into(),
                width: 50.0,
                ..Default::default()
            },
        );

        let path = Path::new(|p| {
            p.move_to(Point { x: 10., y: 10. });
            p.line_to(Point { x: 20., y: 20. });
            p.line_to(Point { x: 40., y: 12. });
        });

        frame.stroke(
            &path,
            Stroke {
                style: Color::BLACK.into(),
                width: 4.,
                ..Default::default()
            },
        );

        let text = Text {
            position: Point { x: 12., y: 42. },
            content: self.toto.clone(),
            color: Color::from_rgb(1., 0., 0.),
            ..Default::default()
        };

        let width = text.size.0 * self.toto.len() as f32;

        frame.fill_text(text);

        frame.fill_rectangle(
            Point { x: 12., y: 42. },
            Size { width, height: 20. },
            Fill {
                style: Color::from_rgb(0., 0.2, 0.2).into(),
                ..Default::default()
            },
        );

        vec![frame.into_geometry()]
    }
}
