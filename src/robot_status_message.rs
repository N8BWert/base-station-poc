use bitfield_struct::bitfield;

const BATTERY_SCALE_FACTOR: f64 = 0.09884;

#[repr(C)]
pub struct RobotStatusMessage {

}

#[bitfield(u8)]
pub struct MotorBallKickStatus {
    #[bits(5)]
    motor_errors: usize,
    #[bits(1)]
    ball_sense_status: usize,
    #[bits(1)]
    kick_status: usize,
    #[bits(1)]
    kick_healthy: usize,
}

#[bitfield(bool)]
pub struct FPGAStatus {
    #[bits(1)]
    fpga_status: usize,
}

#[bitfield(u128)]
pub struct RobotStatusMessage {
    #[bits(6)]
    uid: u8,
    #[bits(8)]
    battery_voltage: u8,
    #[bits(5)]
    motor_errors: usize,
    #[bits(1)]
    ball_sense_status: usize,
    #[bits(1)]
    kick_status: usize,
    #[bits(1)]
    kick_healthy: usize,
    #[bits(1)]
    fpga_status: usize,
    #[bits(288)]
    encoder_deltas: [u16; 18], 
}