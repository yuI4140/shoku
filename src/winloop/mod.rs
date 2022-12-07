#![allow(unused)]
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
mod structs;
use structs::Win;
pub(crate) fn winloop() 
{       let opengl=OpenGL::V4_2;
    let settings=WindowSettings::new(
        "window".to_string(),
        [300,300]).resizable(true).build();
        let mut window: GlutinWindow = settings.unwrap();
        let mut win=Win::new(GlGraphics::new(opengl),0,9);
    let mut events = Events::new(EventSettings::new()).ups(10);
    while let Some(e) = events.next(&mut window) 
    {
        if let Some(r) = e.render_args()
        {   //this inisialize the window 
            win.render(&r);          
        }
        if let Some(m)=e.update_args()
        {   // this inisilize the function of win update
            win.update();
        }
        if let Some(z)=e.button_args()
        {   // this inisilize the function of win pressed
            win.pressed(&z.button);
        }
        
    }
        
         
}
