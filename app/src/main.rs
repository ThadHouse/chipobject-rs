use std::{thread, time::Duration};

use wpilib_hal_rs::{fpga_time, initialize};

fn main() -> anyhow::Result<()> {
    initialize()?;

    loop {
        let ts = fpga_time()?;
        println!("{}", ts);
        thread::sleep(Duration::from_secs(1))
    }
}
