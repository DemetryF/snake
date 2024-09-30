use crate::{Snake, SnakeID};

#[derive(Default)]
pub struct Events {
    on_hit: Option<Box<dyn FnMut(SnakeID, Snake) + Send + Sync>>,
}

impl Events {
    pub fn on_hit(&mut self, handler: impl FnMut(SnakeID, Snake) + Send + Sync + 'static) {
        self.on_hit = Some(Box::new(handler));
    }

    pub fn emit_hit(&mut self, id: SnakeID, snake: Snake) {
        if let Some(handler) = &mut self.on_hit {
            handler(id, snake);
        }
    }
}
