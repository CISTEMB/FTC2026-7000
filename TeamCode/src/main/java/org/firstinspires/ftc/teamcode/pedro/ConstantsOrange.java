package org.firstinspires.ftc.teamcode.pedro;

import com.pedropathing.algorithm.Foresight;
import com.pedropathing.algorithm.ForesightConfig;
import com.pedropathing.controllers.Controller;
import com.pedropathing.follower.Follower;
import com.pedropathing.math.Matrix;
import com.pedropathing.math.Vector2D;
import com.pedropathing.revhub.drivetrains.Mecanum;
import com.pedropathing.revhub.drivetrains.MecanumConfig;
import com.pedropathing.revhub.localizers.PinpointConfig;
import com.pedropathing.revhub.localizers.PinpointLocalizer;
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
//        c.xPodOffset.set(7.150401918906866); //auto generated
//        c.yPodOffset.set(-4.332212613323542); //auto generated
        c.xPodOffset.set(-6.0);
        c.yPodOffset.set(-6.375);
        c.xPodDirection.set(GoBildaPinpointDriver.EncoderDirection.REVERSED);
        c.yPodDirection.set(GoBildaPinpointDriver.EncoderDirection.FORWARD);
        c.globalDistanceUnit.set(DistanceUnit.INCH);
        c.offsetUnits.set(DistanceUnit.INCH);
    });
    public static ForesightConfig foresightConfig = new ForesightConfig(
            c -> {
                Controller primaryTranslationalForward = Controller.proportional(0.33369472318843063);
                Controller secondaryTranslationalForward = Controller.proportional(0.12329128373090421);
                Controller primaryTranslationalLateral = Controller.proportional(0.2791888109770366);
                Controller secondaryTranslationalLateral = Controller.proportional(0.10315280559359177);

                c.forwardTranslational.set(Controller.piecewise(secondaryTranslationalForward).put(2.5, primaryTranslationalForward));
                c.strafeTranslational.set(Controller.piecewise(secondaryTranslationalLateral).put(2.5, primaryTranslationalLateral));

                c.coast.set(Controller.proportionalFeedforward(0.017207728209505193));
                c.brake.set(Controller.proportionalFeedforward(0.014626568978079414));

                c.headingFeedback.set(Controller.proportional(5.681380386655871));
                c.headingBrakeCoefficients.set(Vector2D.cartesian(0.042647226135249704, 0.0064522017968365));

                c.linearBrakeCoefficients.set(Matrix.diag(0.05358735599395559, 0.051549721692975795));
                c.quadraticBrakeCoefficients.set(Matrix.diag(0.001154578213416536, 0.0013273842026506834));

                c.maxAchievableForwardVelocity.set(55.3412433442591);
                c.maxAchievableStrafeVelocity.set(45.03071702072183);
                c.naturalForwardDeceleration.set(39.10176252251308);
                c.naturalStrafeDeceleration.set(58.21751465351203);
            }
    );
    public static Follower create(HardwareMap h) {
        return new Follower(
                new PinpointLocalizer(h, localizerConfig),
                new Mecanum(h, drivetrainConfig),
                new Foresight(foresightConfig)
        );
    }
}