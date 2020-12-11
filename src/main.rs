use bno055::Bno055;
use linux_embedded_hal::{Delay, I2cdev};
use mint::{Quaternion, EulerAngles};

fn main() {
    let dev = I2cdev::new("/dev/i2c-0").expect("An error while setting up i2c device");
    let mut delay = Delay {};
    let mut imu = Bno055::new(dev).with_alternative_address();
    imu.init(&mut delay).expect("Error initializing the IMU");
    
    imu.set_mode(bno055::BNO055OperationMode::NDOF, &mut delay)
        .expect("Error setting the IMU mode");

    loop {
        let quat: Quaternion<f32> = imu.quaternion().expect("Error getting the IMU quaternion");
        println!("Quaternion: {:?}", quat);
        
        let angles: EulerAngles<f32, ()> = imu.euler_angles().expect("Error getting IMU Euler angles");
        println!("Euler angles: {:?}", angles);
        
        std::thread::sleep(std::time::Duration::from_millis(200));
    }

}
