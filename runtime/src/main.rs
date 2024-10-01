mod controller;
mod robot;

use anyhow::Result;
use robot::Robot;
use controller::StandingControllerPPO;

#[tokio::main]
async fn main() -> Result<()> {
    let mut robot = Robot::new();
    let mut controller = StandingControllerPID::new()?;
    controller.run().await
}


