//! The main code.
use ftc::{
    ftc,
    glam::dvec2,
    hardware::{Direction, RunMode, config::config},
    pedro::Pose,
    prelude::*,
};

// pub mod pinpoint;

config! {
    CONFIG_NAME = "FTC2026-7000";
    HAS_EXP_HUB = false;

    static IMU = CTRL_HUB/EmbeddedIMU;
    static LEFT_FRONT = CTRL_HUB/Motor.GoBilda5204(0);
    static RIGHT_FRONT = CTRL_HUB/Motor.GoBilda5204(1);
    static LEFT_REAR = CTRL_HUB/Motor.GoBilda5204(2);
    static RIGHT_REAR = CTRL_HUB/Motor.GoBilda5204(3);
}

/// Driving subsystem.
#[derive(Debug)]
#[allow(clippy::missing_docs_in_private_items)]
pub struct Drive {
    left_front: DcMotor,
    right_front: DcMotor,
    left_rear: DcMotor,
    right_rear: DcMotor,
    /// Whether we're currently in fast mode.
    pub fast: bool,
}

impl Drive {
    /// Get a [`Drive`] with the default names and rotations.
    #[must_use]
    pub fn with_default_names(hardware: &Hardware) -> Self {
        let out = Drive {
            left_front: hardware.get(LEFT_FRONT),
            right_front: hardware.get(RIGHT_FRONT),
            left_rear: hardware.get(LEFT_REAR),
            right_rear: hardware.get(RIGHT_REAR),
            fast: false,
        };
        out.set_direction(Direction::Forward);
        out
    }
    /// Set the direction of the motors, inverting the right motors as needed.
    pub fn set_direction(&self, dir: Direction) {
        self.left_front.set_direction(-dir);
        self.left_rear.set_direction(-dir);
        self.right_front.set_direction(dir);
        self.right_rear.set_direction(dir);
    }
    /// Set the [`RunMode`] of all of the motors.
    pub fn set_mode(&self, mode: RunMode) {
        for motor in [
            &self.left_front,
            &self.right_front,
            &self.left_rear,
            &self.right_rear,
        ] {
            motor.set_mode(mode);
        }
    }
    /// Arcade drive. Pass in values from the gamepad.
    pub fn arcade_drive(&self, forward: f64, turn: f64, strafe: f64) {
        let turn = turn * 0.75;
        let forward = -forward;

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

/// Base autonomous
#[ftc(name = "Autonomous", linear, auto, config = FTC2026_7000)]
pub fn auto(ftc: &ftc::FtcContext) {
    let pedro = ftc.pedro(Pose::new_degrees(0.0, 0.0, 0.0));

    let path = pedro
        .line(pedro.pose(), Pose::new_degrees(10.0, 10.0, 45.0))
        .heading_face(dvec2(-10.0, 10.0))
        .curve([
            Pose::new_degrees(10.0, 10.0, 45.0),
            Pose::new_degrees(0.0, 20.0, 270.0),
        ]);

    ftc.telemetry().add_data("Status", "Initialized");
    ftc.telemetry().update();

    ftc.wait_for_start();

    ftc.telemetry().add_data("Status", "Running");
    ftc.telemetry().update();

    pedro.follow(path).then(|ftc| {
        ftc.telemetry().add_data("Status", "Complete");
        ftc.telemetry().update();
    });
}

/// Base teleop
#[ftc(name = "Teleop", linear, teleop, config = FTC2026_7000)]
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

        let (forward, turn, strafe) = (
            gamepad1.left_stick_y(),
            gamepad1.left_stick_x(),
            gamepad1.right_stick_x(),
        );

        ftc.telemetry().add_data("Forward", forward);
        ftc.telemetry().add_data("Turn", turn);
        ftc.telemetry().add_data("Strafe", strafe);

        motors.fast = gamepad1.left_trigger() > 0.75;
        motors.arcade_drive(forward, turn, strafe);
        ftc.telemetry().update();
    }
}

// /// State used in the iterative op mode. Essentially equivalent to adding properties to a class in
// /// java. Has to implement Default (which can be derived in most scenarios as you see below) and
// /// some other requirements the compiler will enforce.
// #[derive(Default)]
// struct IterativeState {
//     /// Devices implement Default by returning a null object of sorts that panics
//     /// if you use it, but comes in handy for stuff like this.
//     motor: DcMotor,
// }

// /// Example iterative op mode.
// #[ftc(
//     name = "Example: My Iterative Op Mode",
//     iterative,
//     teleop,
//     group = "Example",
// )]
// fn my_iterative_op_mode(iterative: &ftc::IterativeContext) {
//     iterative.init(|ftc: &ftc::FtcContext, state: &mut IterativeState| {
//         // equivalent to hardwareMap.get(DcMotor.class, "motor") in Java:
//         state.motor = ftc.hardware().get::<DcMotor>("motor");
//         state.motor.set_direction(ftc::hardware::Direction::Forward);

//         let gamepad1 = ftc.gamepad1();

//         gamepad1.on_a(
//             move |ftc, _| {
//                 ftc.telemetry().add_data("A", "Pressed");
//             },
//             PressEdge::Press,
//         );
//         gamepad1.on_a(
//             move |ftc, _| {
//                 ftc.telemetry().add_data("A", "Released");
//             },
//             PressEdge::Release,
//         );

//         ftc.telemetry().add_data("Status", "Initialized");
//         ftc.telemetry().update();
//     });

//     iterative.start(|_ftc, state: &mut IterativeState| {
//         state.motor.set_power(0.5);
//         std::thread::sleep(Duration::from_secs_f32(2.0));
//         state.motor.set_power(0.0);
//     });

//     iterative.stop(|ftc, _state: &mut IterativeState| {
//         // state has to have a type, so use the state type and ignore the value.
//         info!("Ran for {:?}!", ftc.runtime());
//     });

//     // attempting to call wait_for_start in a interative op mode will immediately return and
//     // print a warning
// }
