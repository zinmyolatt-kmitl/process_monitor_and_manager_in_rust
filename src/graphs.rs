// This file creates small line charts for graphs
use std::collections::VecDeque;
use iced::{Color, Element, Length, Rectangle, Theme};
use iced::widget::{column, container, text};
use iced_widget::canvas::{self, Frame, Stroke};
use crate::models::{GraphSeries, Message};

// creates a label graph widget
pub fn sparkline<'a>(label: &str, series: &'a GraphSeries, color: Color) -> Element<'a, Message> {
    struct Plot<'a>(&'a VecDeque<f32>, Color);

    // this implements the canvas drawing
    impl<'a> canvas::Program<Message> for Plot<'a> {
        type State = ();

        fn draw(
            &self,
            _state: &(),
            renderer: &iced::Renderer,
            _theme: &Theme,
            bounds: Rectangle,
            _cursor: iced::mouse::Cursor,
        ) -> Vec<canvas::Geometry> {
            // this creates a drawing frame with w and h
            let mut frame = Frame::new(renderer, bounds.size());
            let w = bounds.width;
            let h = bounds.height;
            let data = self.0;

            if data.len() >= 2 {
                // finds max
                let max = data.iter().cloned().fold(1.0, f32::max);
                let step = w / (data.len().saturating_sub(1) as f32);
                let mut builder = iced_widget::canvas::path::Builder::new();

                for (i, v) in data.iter().enumerate() {
                    let x = i as f32 * step;
                    let margin = 2.0;
                    let y = (h - margin) - (v / max) * (h - 2.0 - margin);
                    if i == 0 {
                        builder.move_to([x, y].into());
                    } else {
                        builder.line_to([x, y].into());
                    }
                }

                let path = builder.build();
                
                let stroke = Stroke::default()
                    .with_width(2.0)
                    .with_color(self.1);
                
                frame.stroke(&path, stroke);
            }
            vec![frame.into_geometry()]
        }
    }

    let canvas = iced_widget::canvas(Plot(&series.points, color))
        .width(Length::Fill)
        .height(80.0);

    column![text(label).size(14), canvas]
        .spacing(4)
        .width(Length::FillPortion(1))
        .into()
}

pub fn graph_card<'a>(label: &str, series: &'a GraphSeries, color: Color) -> Element<'a, Message> {
    let sparkline_widget = sparkline(label, series, color);
    
    container(sparkline_widget)
        .padding(12)
        .width(Length::FillPortion(1)) // multiple cards share space equally
        .style(|_theme: &Theme| {
            container::Appearance {
                background: Some(iced::Background::Color(Color::from_rgb(0.25, 0.25, 0.25))),
                border: iced::Border {
                    radius: 8.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .into()
}