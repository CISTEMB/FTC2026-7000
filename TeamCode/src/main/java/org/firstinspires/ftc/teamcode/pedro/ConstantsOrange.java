package org.firstinspires.ftc.teamcode.pedro;

import com.pedropathing.algorithm.ForesightConfig;
import com.pedropathing.controllers.Controller;
import com.pedropathing.follower.Follower;
import com.pedropathing.math.Matrix;
import com.pedropathing.math.Vector2D;
import com.pedropathing.revhub.drivetrains.MecanumConfig;
import com.pedropathing.revhub.localizers.PinpointConfig;
import com.qualcomm.hardware.gobilda.GoBildaPinpointDriver;
import com.qualcomm.robotcore.hardware.DcMotorSimple;
import com.qualcomm.robotcore.hardware.HardwareMap;

import org.firstinspires.ftc.robotcore.external.navigation.DistanceUnit;

public class ConstantsOrange {
    public static MecanumConfig drivetrainConfig = new MecanumConfig(c -> {
        c.frontLeftName.set("LEFT_FRONT");
        c.frontRightName.set("RIGHT_FRONT");
        c.backLeftName.set("LEFT_REAR");
        c.backRightName.set("RIGHT_REAR");
        c.frontLeftDirection.set(DcMotorSimple.Direction.REVERSE);
        c.frontRightDirection.set(DcMotorSimple.Direction.FORWARD);
        c.backLeftDirection.set(DcMotorSimple.Direction.REVERSE);
        c.backRightDirection.set(DcMotorSimple.Direction.FORWARD);
    });
    public static PinpointConfig localizerConfig = new PinpointConfig(c -> {
        c.name.set("PINPOINT");
        c.podType.set(GoBildaPinpointDriver.GoBildaOdometryPods.goBILDA_SWINGARM_POD);
        c.xPodOffset.set(7.150401918906866);
        c.yPodOffset.set(-4.332212613323542);
        c.xPodDirection.set(GoBildaPinpointDriver.EncoderDirection.FORWARD);
        c.yPodDirection.set(GoBildaPinpointDriver.EncoderDirection.FORWARD);
        c.globalDistanceUnit.set(DistanceUnit.INCH);
        c.offsetUnits.set(DistanceUnit.INCH);
    });
    public static ForesightConfig foresightConfig = new ForesightConfig(
            c -> {
                Controller primaryTranslationalForward = Controller.proportional(0.32056536773229805);
                Controller secondaryTranslationalForward = Controller.proportional(0.11844033771270224);
                Controller primaryTranslationalLateral = Controller.proportional(0.6153556206814876);
                Controller secondaryTranslationalLateral = Controller.proportional(0.22735745923679732);

                c.forwardTranslational.set(Controller.piecewise(secondaryTranslationalForward).put(2.5, primaryTranslationalForward));
                c.strafeTranslational.set(Controller.piecewise(secondaryTranslationalLateral).put(2.5, primaryTranslationalLateral));

                c.coast.set(Controller.proportionalFeedforward(0.018556154150148507));
                c.brake.set(Controller.proportionalFeedforward(0.01577273102762623));

                c.headingFeedback.set(Controller.proportional(7.401907353063674));
                c.headingBrakeCoefficients.set(Vector2D.cartesian(0.0478259125198541, 0.011524921961293238));

                c.linearBrakeCoefficients.set(Matrix.diag(0.07598991633305714, 0.3829801930485818));
                c.quadraticBrakeCoefficients.set(Matrix.diag(0.0011895714119907722, -0.007984559099847353));

                c.maxAchievableForwardVelocity.set(55.763849124682714);
                c.maxAchievableStrafeVelocity.set(41.0363995528179);
                c.naturalForwardDeceleration.set(35.78527264062068);
                c.naturalStrafeDeceleration.set(58.011161397357355);
            }
    );
    public static Follower create(HardwareMap h) {
//        return new Follower(localizerConfig, drivetrainConfig, foresightConfig);
        return null;
    }
}