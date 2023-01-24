use std::rc::{Rc, Weak};

fn main() {
    let mut gfx = Graphics::default();

    println!("First draw:");
    gfx.draw();

    {
        let _sprite_0 = gfx.load_sprite(0);

        println!("Second draw:");
        gfx.draw();

        {
            let _sprite_1 = gfx.load_sprite(1);

            println!("Third draw:");
            gfx.draw();
        }

        println!("Fourth draw:");
        gfx.draw();
    }

    println!("Last draw:");
    gfx.draw();
}

#[derive(Default)]
struct Graphics {
    sprites: Vec<Weak<Sprite>>,
}

impl Graphics {
    pub fn load_sprite(&mut self, id: u64) -> Rc<Sprite> {
        let sprite = Rc::new(Sprite::new(id));
        self.sprites.push(Rc::downgrade(&sprite));
        sprite
    }

    pub fn draw(&mut self) {
        self.sprites.retain(|s| match s.upgrade() {
            Some(s) => {
                s.draw();
                true
            }
            None => false,
        });
    }
}

#[derive(Default)]
struct Sprite {
    id: u64,
}

impl Sprite {
    fn new(id: u64) -> Self {
        Self { id }
    }

    fn draw(&self) {
        println!("\tSprite #{}", self.id)
    }
}
