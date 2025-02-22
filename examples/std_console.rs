use std::{
    io::{stdout, Write},
    mem,
    thread::sleep,
    time::{Duration, Instant},
};

use toroid::Donut;

fn main() {
    const WIDTH: u8 = 80;
    const HEIGHT: u8 = 22;
    const SIZE: usize = (WIDTH as usize) * (HEIGHT as usize);

    // Create the donut instance and pre allocate buffers
    let mut donut = Donut::<WIDTH, HEIGHT>::new();
    let mut output = [' '; SIZE];
    let mut zbuf = [0.0_f32; SIZE];

    // Clear the terminal
    print!("\x1B[2J");
    let stdout = stdout();
    let mut handle = stdout.lock();

    loop {
        // Start timing this frame
        let start = Instant::now();

        // Render the donut into output and depth buffers
        donut.render_frame_in_place(&mut output, &mut zbuf);

        // Reset cursor to top left
        write!(handle, "\x1B[H").unwrap();

        // Write output buffer
        for line in output.chunks(WIDTH as usize) {
            for &ch in line {
                handle.write_all(&[ch as u8]).unwrap();
            }

            handle.write_all(b"\n").unwrap();
        }

        // Write the status line with FPS and memory usage.
        writeln!(
            handle,
            "\nFPS: {:>5.1} | Approx Mem: {}",
            1.0 / start.elapsed().as_secs_f32().max(0.0001),
            {
                let u =
                    mem::size_of_val(&output) + mem::size_of_val(&zbuf) + mem::size_of_val(&donut);

                if u < 1024 {
                    format!("{} bytes", u)
                } else if u < 1024 * 1024 {
                    format!("{:.1} KB", u as f32 / 1024.0)
                } else {
                    format!("{:.1} MB", u as f32 / (1024.0 * 1024.0))
                }
            }
        )
        .unwrap();

        handle.flush().unwrap();

        // Rotate for the next frame and sleep briefly.
        donut.rotate(0.07, 0.03);
        sleep(Duration::from_millis(20));
    }
}
