/*
By: <Your Name Here>
Date: 2026-03-02
Program Details: <Program Description Here>
*/

mod modules;

use macroquad::prelude::*;
use crate::modules::grid::draw_grid;
use crate::modules::label::Label;
use crate::modules::text_button::TextButton;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "nurseryrhyme".to_string(),
        window_width: 1024,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut lbl_nursery = Label::new("Press a button to display one of the three nursery rhymes!", 100.0, 100.0, 25);
    lbl_nursery
    .with_fixed_size(710.0, 300.0)
    .with_border(BLACK, 5.0);

     let btn_twinkle = TextButton::new(
        100.0,
        400.0,
        200.0,
        50.0,
        "Twinkle Twinkle",
        BLUE,
        GREEN,
        28

        
    );

    let btn_humpty = TextButton::new(
        350.0,
        400.0,
        200.0,
        50.0,
        "Humpty Dumpty",
        BLUE,
        GREEN,
        28

        
    );

    let btn_lamb = TextButton::new(
        600.0,
        400.0,
        200.0,
        50.0,
        "Mary Had A Lamb",
        BLUE,
        GREEN,
        28

        
    );

    let btn_exit = TextButton::new(
        350.0,
        500.0,
        200.0,
        50.0,
        "Exit",
        RED,
        GREEN,
        28
       
    
    );

    
    loop {
        clear_background(WHITE);
        

        if btn_twinkle.click() {
            lbl_nursery.set_text("Twinkle, twinkle, little star,
How I wonder what you are,
Up above the world so high,
Like a diamond in the sky, twinkle, twinkle, little star,
How I wonder what you are.

");
            
        }

        if btn_humpty.click() {
            lbl_nursery.set_text("Humpty Dumpty sat on a wall,
Humpty Dumpty had a great fall,
All the kings horses and all the kings men,
Couldnt put Humpty together again.");
            
        }

        if btn_lamb.click() {
            lbl_nursery.set_text("Mary had a little lamb,
His fleece was white as snow,
And everywhere that Mary went,
The lamb was sure to go

He followed her to school one day,
Which was against the rule,
It made the children laugh and play,
To see a lamb at school.

And so the teacher turned him out,
But still he lingered near,
And waited patiently about,
Till Mary did appear.
What makes the lamb love Mary so?
The eager children cry;
Why, Mary loves the lamb, you know,
The teacher did reply.");
            
        }

        if btn_exit.click() {
            break;
        }




        lbl_nursery.draw();
        
        next_frame().await;
    }
}
