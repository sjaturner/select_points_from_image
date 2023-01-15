# Picker, to pick the position of points on an image in a relatively convenient way

I wanted to pick the positions of points on images, and I wanted to
use Rust.  This program suits my needs but I have no doubt that there
are better alternatives. The selected points are sent to stdout as 
pairs of numbers, suitable for input to other programs.

## Building

Clone the repository, then run

    cargo build

## Running the program

I used the clap crate so there's some help available:

    target/debug/picker --help
    picker 0.0.0

    USAGE:
        picker [OPTIONS] --file <FILE>

    OPTIONS:
        -f, --file <FILE>            
        -g, --geometry <GEOMETRY>    [default: 1024x768]
        -h, --help                   Print help information
        -V, --version                Print version information

FILE is the file for which you'd like to find the pixel position of
certain features. I think that this will work with quite a lot of image
formats, thanks to the crates which I used.

The geometry is just the window size which you'd like the picker to fit into.

I'll warn you in advance, the mouse button behaviour is odd. It seems
fairly ergonomic, once you get used to it though.

    Left Button: Re-center the image on whereever the cursor is when you press this.
    Scroll wheel: Zoom around the center of the image
    Right Button: Emit the pixel position at the center of the image with unreasonable resolution.

There's an image in the assets directory which you can use. Try running this:

    target/debug/picker -f assets/rust.png 

You will see a picture of the Rust logo. There will be a pixel sized
red dot at the center.  Now, click at a point of interest on the image,
for example the lowest tooth on the outer cog of the logo. Note how that
snaps to the center of the screen - in a slightly jarring way :-/

Now, try zooming in on that by scrolling the mouse wheel. Did the image
get smaller? If so, try scrolling the other way. OK, so maybe your first
attempt to hit the point of the cog was a little off - try again! Click
on what you actually wanted!  By a series of refinements you'll end up
with the red dot positioned precisely over the point of interest.

Now click the Right mouse button. The central pixel position is written
to stdout. You can copy and paste that, or maybe just have everything
sent to a file. Keep scrolling and positioning until you have selected
all of the points which you want.

TBH it's easier to do than describe :-)
