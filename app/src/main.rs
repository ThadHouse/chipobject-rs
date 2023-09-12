use std::{thread, time::Duration};

use chipobject_rs::{open_fpga, objects::{global::Global, hmb::Hmb, duty_cycle::{NUM_DUTY_CYCLE_INTERFACES, DutyCycle}}, ni_fpga::{RegisterRead, RegisterWrite, HmbAccess}};

fn main() -> anyhow::Result<()> {
    let (mut bitfile, session) = open_fpga()?;

    let globals = Global::take(&mut bitfile)?;
    let mut hmb = Hmb::take(&mut bitfile)?;

    for i in 0..NUM_DUTY_CYCLE_INTERFACES {
        let _dc = DutyCycle::take(&mut bitfile, i)?;
    }

    let mut c = hmb.config.read(&session)?;
    c.Enables.Timestamp = true;
    hmb.config.write(&session, &c)?;

    let mem = unsafe { session.open_hmb(hmb.memory_definition.name)? };


    loop {
        let result = globals.local_time_lower.read(&session)?;
        let ts: u32 = mem.read(0x3C0);
        println!("TS: {} {}", result, ts);
        thread::sleep(Duration::from_secs(1));
    }
}
