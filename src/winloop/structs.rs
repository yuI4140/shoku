#![allow(unused)]
use crate::prelude::*;
use graphics::{self, types::Color};
use opengl_graphics::GlGraphics;
use piston::{input::{RenderArgs, Button, Key}};
#[derive(Debug)]
struct Floor {
    f_x: i32,
    f_y: i32, 
}
impl Floor {
    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) 
    {
        // creation of square
        let mut _square =
            graphics::rectangle::square((self.f_x * 30) as f64, (self.f_y * 20) as f64, 20_f64);
        gl.draw(args.viewport(), |c, gl| {
            // do a transform
            let transform = c.transform;
            //do a rectangle
            let _white =shexcolor("#ffffff",1.0);
            graphics::rectangle(_white, _square, transform, gl);
            //NOTE:you need to call this method
            //after after declaring the background
            //in Win.
        });    
    }
    pub fn new(x0:i32,y0:i32)->Floor
        {
            let flr=Floor
            {
                f_x:x0,f_y:y0,

            };
            return flr;
        }
}
#[derive(Debug,Copy,Clone)]
pub(crate) struct Entity {
     e_x: i32,
     e_y: i32,
     dir: Direction,
}
#[derive(Debug,Copy,Clone,PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Neutral,
    Reload
}
pub struct Win {
    gl: GlGraphics,
    entity: Entity,
    floor:Floor

}
impl Win 
{
    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |c, gl| {
            let BACK_COLOR: Color =shexcolor("#000000",1.0);
            graphics::clear(BACK_COLOR, gl);
        });
        //ToDO: implement  entity
        self.entity.render(&mut self.gl, args);
        self.floor.render(&mut self.gl, args);
    }
    pub fn update(&mut self) {
        self.entity.update();
    }
    pub fn pressed(&mut self,btn:&Button)
    {   let keyinput=self.entity.dir.clone();
        self.entity.dir=match btn
        {   &Button::Keyboard(Key::D)
            if keyinput==Direction::Neutral=>Direction::Right,
            &Button::Keyboard(Key::A)
            if keyinput==Direction::Down=>Direction::Left,
            &Button::Keyboard(Key::W)
            if keyinput==Direction::Neutral=>Direction::Up,
            &Button::Keyboard(Key::S)
            if keyinput==Direction::Neutral=>Direction::Down,
            &Button::Keyboard(Key::D)
            if keyinput!=Direction::Neutral=>Direction::Right,
            &Button::Keyboard(Key::A)
            if keyinput!=Direction::Down=>Direction::Left,
            &Button::Keyboard(Key::W)
            if keyinput!=Direction::Neutral=>Direction::Up,
            &Button::Keyboard(Key::S)
            if keyinput!=Direction::Neutral=>Direction::Down,
            _=>keyinput
        };
        self.entity.dir=match btn
        {   
            &Button::Keyboard(Key::D) =>Direction::Right, 
            &Button::Keyboard(Key::A)=>Direction::Left, 
            &Button::Keyboard(Key::W)=>Direction::Up, 
            &Button::Keyboard(Key::S)=>Direction::Down, 
            _=>keyinput
        };
            
         
    }
    pub fn new(gl0:GlGraphics,x:i32,y:i32)->Win
    {   let d=Direction::Neutral;
        let f=Floor::new(0,0);
        let e=Entity::new(x,y,d);
        let win=Win
        {
            gl: gl0,
            entity:e,
            floor:f,
        };
        return win;
    }
}
// creation of entity

impl Entity 
{
    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) 
    {   
        let size=30_f64;
        // creation of square
        let mut _square =
            graphics::rectangle::square((self.e_x * 30) as f64, (self.e_y * 20) as f64,size);
        gl.draw(args.viewport(), |c, gl| {
            // do a transform
            let transform = c.transform;
            //do a rectangle
            let _white = graphics::color::WHITE;
            graphics::rectangle(_white, _square, transform, gl);
            //NOTE:you need to call this method
            //after after declaring the background
            //in Win.
        });
    }
    pub fn update(&mut self) {
        match self.dir
        {
            Direction::Up =>self.e_y-=1,
            Direction::Down =>self.e_y+=1,
            Direction::Left => self.e_x-=1,
            Direction::Right => self.e_x+=1,
            Direction::Neutral => self.e_x-=0,
            Direction::Reload => self.e_x=5,
        }   
    }
    pub fn new(x:i32,y:i32,d:Direction) ->Entity
    {
        let enty =Entity
        {
            e_x: x,
            e_y: y,
            dir: d,
        };
        return enty;
    }
}

