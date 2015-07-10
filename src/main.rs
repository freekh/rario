extern crate gl;
extern crate glutin;
extern crate libc;

use glutin::{Event, ElementState, MouseCursor};

fn main() {
	let window = glutin::Window::new().unwrap();
	window.set_title("Glutin Test");

    unsafe {
    	window.make_current()
    		.ok()
    		.expect("Window.make_current() failed :(");

        gl::load_with(|symbol| window.get_proc_address(symbol));
        gl::ClearColor(0.25, 0.25, 0.25, 1.0);
    }

    let cursors = [MouseCursor::Default, MouseCursor::Crosshair, MouseCursor::Hand, MouseCursor::Arrow, MouseCursor::Move, MouseCursor::Text, MouseCursor::Wait, MouseCursor::Help, MouseCursor::Progress, MouseCursor::NotAllowed, MouseCursor::ContextMenu, MouseCursor::NoneCursor, MouseCursor::Cell, MouseCursor::VerticalText, MouseCursor::Alias, MouseCursor::Copy, MouseCursor::NoDrop, MouseCursor::Grab, MouseCursor::Grabbing, MouseCursor::AllScroll, MouseCursor::ZoomIn, MouseCursor::ZoomOut, MouseCursor::EResize, MouseCursor::NResize, MouseCursor::NeResize, MouseCursor::NwResize, MouseCursor::SResize, MouseCursor::SeResize, MouseCursor::SwResize, MouseCursor::WResize, MouseCursor::EwResize, MouseCursor::NsResize, MouseCursor::NeswResize, MouseCursor::NwseResize, MouseCursor::ColResize, MouseCursor::RowResize];
    let mut cursor_idx = 0;

    for event in window.wait_events() {
        unsafe {
        	gl::Clear(gl::COLOR_BUFFER_BIT);
        };

        window.swap_buffers()
        	.ok()
    		.expect("Window.swap_buffers() failed :(");
    	

        match event {
        	a @ Event::KeyboardInput(_, _, _) => {
                println!("{:?}", a);
            },

            a @ Event::MouseMoved(_) => {
                // println!("{:?}", a);
            },

            Event::Closed => break,
            _ => ()
        }
    }
}
