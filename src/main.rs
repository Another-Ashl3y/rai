mod neurons;
mod display;

use std::io::{self, Write};
use crossterm::execute;
use crossterm::style::Stylize;
use crossterm::terminal;
use crossterm::terminal::{BeginSynchronizedUpdate, EndSynchronizedUpdate, DisableLineWrap};
use neurons::Network;

fn main() -> io::Result<()> {
    // AI
    let mut network = Network::new(1, 10);
    network.set_inputs(vec![0.5]);


    // Display stuff    
    let mut size = terminal::size()?;
    let mut stdout = io::stdout();
    let mut message_box = display::Box::new(display::Vec2::new(0, 0),display::Vec2::new(size.0 as i64-61, size.1 as i64));
    let mut update_box = display::Box::new(display::Vec2::new(message_box.size.x + 1, 0),display::Vec2::new(60, 10));
    let mut extra_box = display::Box::new(display::Vec2::new(message_box.size.x + 1, 10),display::Vec2::new(60, size.1 as i64 - 10));



    execute!(io::stdout(), DisableLineWrap)?;
    let mut n = 0;
    while n < 10000 {
        execute!(io::stdout(), BeginSynchronizedUpdate)?;
    
        // display::clear(&stdout)?;
        let new_size = terminal::size()?;
        if new_size != size {
            size = new_size;
            message_box.resize(display::Vec2::new(size.0 as i64-61, size.1 as i64));
            update_box.resize(display::Vec2::new(60, 10));
            extra_box.resize(display::Vec2::new(60, size.1 as i64 - 10));
            update_box.reposition(display::Vec2::new(message_box.size.x + 1, 0));
            extra_box.reposition(display::Vec2::new(message_box.size.x + 1, 10));
            display::clear(&stdout)?;
        }
        message_box.draw(&stdout)?;
        update_box.draw(&stdout)?;
        extra_box.draw(&stdout)?;
        stdout.flush()?;

        message_box.add_string(network.clone().get_outputs().iter().map(|x| x.to_string()).collect::<String>().green().to_string());
        // message_box.add_string((n as f64).sin().to_string().red().to_string());
        // update_box.add_string((n as f64).tan().to_string().white().to_string());
        // extra_box.add_string((n as f64).cos().to_string().yellow().to_string());
        n += 1;

        execute!(io::stdout(), EndSynchronizedUpdate)?;
    }

    Ok(())
}



// use std::fs::File;
// use std::io::{prelude::*, BufReader};
// use std::{time::Duration,thread::sleep};

    // let file = File::open("conv_list.txt")?;
    // let reader = BufReader::new(file);

    // for line in reader.lines() {
    //     let s = line.unwrap();
    //     message_box.add_string(s.clone().cyan().to_string());
    //     update_box.add_string(s.clone().cyan().to_string());
    //     extra_box.add_string(s.clone().cyan().to_string());
    // }