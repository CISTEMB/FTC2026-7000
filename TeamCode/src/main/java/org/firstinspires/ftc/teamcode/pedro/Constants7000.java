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

public class Constants7000 {
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
//        c.xPodOffset.set( 0.336566346836841); //auto generated
//        c.yPodOffset.set(-8.692875058632197); // auto generated
        c.xPodOffset.set(-6.0);
        c.yPodOffset.set(-6.375);

        c.xPodDirection.set(GoBildaPinpointDriver.EncoderDirection.FORWARD);
        c.yPodDirection.set(GoBildaPinpointDriver.EncoderDirection.FORWARD);
        c.globalDistanceUnit.set(DistanceUnit.INCH);
        c.offsetUnits.set(DistanceUnit.INCH);
    });
    public static ForesightConfig foresightConfig = new ForesightConfig(
            c -> {
                Controller primaryTranslationalForward = Controller.proportional(0.3918312174942862);
                Controller secondaryTranslationalForward = Controller.proportional(0.14477116494117998);
                Controller primaryTranslationalLateral = Controller.proportional(0.5397495398307645);
                Controller secondaryTranslationalLateral = Controller.proportional(0.19942303259414254);

                c.forwardTranslational.set(Controller.piecewise(secondaryTranslationalForward).put(2.5, primaryTranslationalForward));
                c.strafeTranslational.set(Controller.piecewise(secondaryTranslationalLateral).put(2.5, primaryTranslationalLateral));

                c.coast.set(Controller.proportionalFeedforward(0.020693958473160225));
                c.brake.set(Controller.proportionalFeedforward(0.01758986470218619));

                c.headingFeedback.set(Controller.proportional(5.7324023424988235));
                c.headingBrakeCoefficients.set(Vector2D.cartesian(0.06080787961955257, 0.001759479514903095));

                c.linearBrakeCoefficients.set(Matrix.diag(0.06143106995287689, 0.06951510119075396));
                c.quadraticBrakeCoefficients.set(Matrix.diag(0.0011768467936952292, 0.0010955075705613756));

                c.maxAchievableForwardVelocity.set(53.012925862592205);
                c.maxAchievableStrafeVelocity.set(43.12341928828356);
                c.naturalForwardDeceleration.set(43.74996831817726);
                c.naturalStrafeDeceleration.set(73.53237976292893);
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