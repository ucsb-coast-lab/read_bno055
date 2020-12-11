## read_bno055

This is a code sample for reading values from a Bosch BNO055 Absolute Orientation Sensor, with code in both Rust and Python.

They can be run using
```bash
# Python, including dependency installations
$ sudo apt-get install python3-libgpiod
$ sudo pip3 install adafruit-circuitpython-bno055
$ sudo python3 read_bno055.py
# Rust, which should package all the necessary deps
$ cargo build 
$ sudo ./target/debug/read_bno055 
```


Please note the use of the `sudo`, which provides the heightened permissions required to open up the i2c connection (both example with fail otherwise), and review the code before running. 

Both samples were tested on an Odroid-C4 board, using the Adafruit 9-DOF Breakout Board (listing [here](https://www.adafruit.com/product/4646), docs [here](https://buildmedia.readthedocs.org/media/pdf/adafruit-circuitpython-bno055/latest/adafruit-circuitpython-bno055.pdf)) over an I2C connection on pins 3/5, based on the layout schematic [here](https://wiki.odroid.com/odroid-c4/application_note/gpio/pwm). 

---

To help examine the board set-up, it may be helpful to install some handy utilities using `$ sudo apt install i2c-tools`. In particular, we'll look into using the [`i2cdump`](https://manpages.debian.org/buster/i2c-tools/i2cdump.8.en.html) command to make double check that we are at least reading in some bytes from the sensor. 

Per the marking on the BNO055, we should see that the default address for the I2C device is 0x28, and of the two existing I2C devices in the Odroid-C4's device tree (`/dev/i2c-x`), we have options for `i2c-0` or `i2c-1`. If we're using these pins, we should see the data on stream 0, which we can check using the command `$ sudo i2cdump 0 0x28`, which should output a bit field that *isn't* all X's. You can compare this to the substituting the 0 for 1 (which will change the I2C connection), which won't be outputting any data. 

Once these are verified, both examples should be working with that address. 