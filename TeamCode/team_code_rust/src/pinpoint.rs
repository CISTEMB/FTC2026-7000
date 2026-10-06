//! Wrapper type for `GoBildaPinpointDriver`.

use ftc::{call_method, device, enum_variant_into, hardware::{Direction, IntoJniObject}, jni};

device!(
    GoBildaPinpointDriver,
    JAVA_CLASS = "com.qualcomm.hardware.gobilda.GoBildaPinpointDriver";
    JNI_CLASS  = "com/qualcomm/hardware/gobilda/GoBildaPinpointDriver";
);

impl GoBildaPinpointDriver {
    /// Call this once per loop to read new data from the Odometry Computer. Data will only update once this is called.
    pub fn update(&self) {
        self.vm
            .attach_current_thread(|env| {
                call_method!(
                    env env,
                    self.object,
                    "update",
                    "()V",
                    []
                )?;
                jni::errors::Result::Ok(())
            })
            .unwrap();
    }
    /// Sets the odometry pod positions relative to the point that the odometry computer tracks
    /// around.
    ///
    /// The most common tracking position is the center of the robot.
    ///
    /// The X pod offset refers to how far sideways from the tracking point the X (forward) odometry
    /// pod is. Left of the center is a positive number, right of center is a negative number.
    ///
    /// The Y pod offset refers to how far forwards from the tracking point the Y (strafe) odometry
    /// pod is. forward of center is a positive number, backwards is a negative number.
    pub fn set_offsets(&self, x_offset: f64, y_offset: f64) {
        self.vm
            .attach_current_thread(|env| {
                call_method!(
                    env env,
                    self.object,
                    "setOffsets",
                    "(DD)V",
                    [x_offset, y_offset]
                )?;
                jni::errors::Result::Ok(())
            })
            .unwrap();
    }
    /// Recalibrates the Odometry Computer's internal IMU. **The robot MUST be stationary!**
    /// 
    /// Device takes a large number of samples, and uses those as the gyroscope zero-offset.
    /// This takes approximately 0.25 seconds.
    pub fn recalibrate_imu(&self) {
        self.vm
            .attach_current_thread(|env| {
                call_method!(
                    env env,
                    self.object,
                    "recalibrateIMU",
                    "()V",
                    []
                )?;
                jni::errors::Result::Ok(())
            })
            .unwrap();
    }
    /// Resets the current position to 0,0,0 and recalibrates the Odometry Computer's internal IMU.
    /// **The robot MUST be stationary!**
    /// 
    /// Device takes a large number of samples, and uses those as the gyroscope zero-offset.
    /// This takes approximately 0.25 seconds.
    pub fn reset_pos_and_imu(&self) {
        self.vm
            .attach_current_thread(|env| {
                call_method!(
                    env env,
                    self.object,
                    "resetPosAndIMU",
                    "()V",
                    []
                )?;
                jni::errors::Result::Ok(())
            })
            .unwrap();
    }
    /// Resets the current position to 0,0,0 and recalibrates the Odometry Computer's internal IMU.
    /// **The robot MUST be stationary!**
    /// 
    /// Device takes a large number of samples, and uses those as the gyroscope zero-offset.
    /// This takes approximately 0.25 seconds.
    pub fn set_encoder_directions(&self, x: Direction, y: Direction) {
        self.vm
            .attach_current_thread(|env| {
                let x = InternalDirection::from(x).into_jni_object(env);
                let y = InternalDirection::from(y).into_jni_object(env);
                call_method!(
                    env env,
                    self.object,
                    "setEncoderDirections",
                    "(Lcom/qualcomm/hardware/gobilda/GoBildaPinpointDriver$EncoderDirection;Lcom/qualcomm/hardware/gobilda/GoBildaPinpointDriver$EncoderDirection;)V",
                    [&x, &y]
                )?;
                jni::errors::Result::Ok(())
            })
            .unwrap();
    }
}

#[allow(clippy::missing_docs_in_private_items)]
#[derive(Clone, Copy, Debug)]
enum InternalDirection {
    Forward,
    Reverse,
}

impl From<Direction> for InternalDirection {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Forward => InternalDirection::Forward,
            Direction::Reverse => InternalDirection::Reverse,
        }
    }
}

enum_variant_into! {
    InternalDirection,
    "com/qualcomm/hardware/gobilda/GoBildaPinpointDriver$EncoderDirection",
    "com.qualcomm.hardware.gobilda.GoBildaPinpointDriver.EncoderDirection",
    Forward,
    Reverse,
}
