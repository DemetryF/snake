use egui::{emath::TSTransform, epaint::RectShape, Color32, Rect, Rounding, Stroke};

#[derive(Clone)]
pub struct Painter {
    pub raw: egui::Painter,
    pub transform: TSTransform,
}

impl Painter {
    pub fn draw(&self, shape: impl Into<egui::Shape>) {
        let mut shape = shape.into();

        shape.transform(self.transform);

        self.raw.add(shape);
    }

    pub fn rect(&self, rect: Rect, color: Color32) {
        let shape = RectShape::new(rect, Rounding::ZERO, color, Stroke::NONE);

        self.draw(shape);
    }
}
