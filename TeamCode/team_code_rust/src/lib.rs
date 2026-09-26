//! The main code.
use std::time::Duration;

use ftc::{PressEdge, ftc, hardware::{Direction, RunMode}, prelude::*};

pub mod pinpoint;

/// Driving subsystem.
#[derive(Debug)]
pub struct Drive {
    left_front: DcMotor,
    right_front: DcMotor,
    left_rear: DcMotor,
    right_rear: DcMotor,
    /// Whether we're currently in fast mode.
    fast: bool,
}

impl Drive {
    /// Get a [`Drive`] with the default names.
    #[must_use]
    pub fn with_default_names(hardware: &Hardware) -> Self {
        let out = Drive {
            left_front: hardware.get("leftFront"),
            right_front: hardware.get("rightFront"),
            left_rear: hardware.get("leftRear"),
            right_rear: hardware.get("rightRear"),
            fast: false,
        };
        out.right_front.set_direction(Direction::Reverse);
        out.right_rear.set_direction(Direction::Reverse);
        out
    }
    /// Set the [`RunMode`] of all of the motors.
    pub fn set_mode(&self, mode: RunMode) {
        for motor in [&self.left_front, &self.right_front, &self.left_rear, &self.right_rear] {
            motor.set_mode(mode);
        }
    }
    /// Arcade drive. Pass in values from the gamepad.
    pub fn arcade_drive(&self, forward: f64, turn: f64, strafe: f64) {
        let turn = turn * 0.75;
        let strafe = -strafe;

        // Calculate speed for each motor
        let left_front = forward + turn + strafe;
        let right_front = forward - turn - strafe;
        let left_rear = forward + turn - strafe;
        let right_rear = forward - turn + strafe;

        self.set_mode(RunMode::RunWithoutEncoder);

        let multiple = if self.fast { 2.0 } else { 1.0 };

        self.left_front.set_power(left_front * multiple);
        self.right_front.set_power(right_front * multiple);
        self.left_rear.set_power(left_rear * multiple);
        self.right_rear.set_power(right_rear * multiple);
    }
}

/// Base teleop
#[ftc(name = "Teleop", linear, teleop)]
pub fn teleop(ftc: &ftc::FtcContext) {
    let hardware = ftc.hardware();

    let mut motors = Drive::with_default_names(&hardware);

    ftc.telemetry().add_data("Status", "Initialized");
    ftc.telemetry().update();

    ftc.wait_for_start();

    // you can store it here, or just call the method directly in the body; storing it once is
    // slightly more efficient
    let gamepad1 = ftc.gamepad1();

    while ftc.running() {
        ftc.telemetry().add_data("Status", "Running");

        let (forward, turn, strafe) = (gamepad1.left_stick_y(), gamepad1.left_stick_x(), gamepad1.right_stick_x());

        ftc.telemetry().add_data("Forward", forward);
        ftc.telemetry().add_data("Turn", turn);
        ftc.telemetry().add_data("Strafe", strafe);

        motors.fast = gamepad1.left_trigger() > 0.75;
        motors.arcade_drive(forward, turn, strafe);
        ftc.telemetry().update();
    }
}

/// State used in the iterative op mode. Essentially equivalent to adding properties to a class in
/// java. Has to implement Default (which can be derived in most scenarios as you see below) and
/// some other requirements the compiler will enforce.
#[derive(Default)]
struct IterativeState {
    /// Devices implement Default by returning a null object of sorts that panics
    /// if you use it, but comes in handy for stuff like this.
    motor: DcMotor,
}

/// Example iterative op mode.
#[ftc(
    name = "Example: My Iterative Op Mode",
    iterative,
    teleop,
    group = "Example",
    disabled,
)]
fn my_iterative_op_mode(iterative: &ftc::IterativeContext) {
    iterative.init(|ftc: &ftc::FtcContext, state: &mut IterativeState| {
        // equivalent to hardwareMap.get(DcMotor.class, "motor") in Java:
        state.motor = ftc.hardware().get::<DcMotor>("motor");
        state.motor.set_direction(ftc::hardware::Direction::Forward);

        let gamepad1 = ftc.gamepad1();

        gamepad1.on_a(
            move |ftc, _| {
                ftc.telemetry().add_data("A", "Pressed");
            },
            PressEdge::Press,
        );
        gamepad1.on_a(
            move |ftc, _| {
                ftc.telemetry().add_data("A", "Released");
            },
            PressEdge::Release,
        );

        ftc.telemetry().add_data("Status", "Initialized");
        ftc.telemetry().update();
    });

    iterative.start(|_ftc, state: &mut IterativeState| {
        state.motor.set_power(0.5);
        std::thread::sleep(Duration::from_secs_f32(2.0));
        state.motor.set_power(0.0);
    });

    iterative.stop(|ftc, _state: &mut IterativeState| {
        // state has to have a type, so use the state type and ignore the value.
        info!("Ran for {:?}!", ftc.runtime());
    });

    // attempting to call wait_for_start in a interative op mode will immediately return and
    // print a warning
}
