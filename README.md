# Philips Hue library in Rust

[![Build Status](https://travis-ci.org/kali/hue.rs.svg?branch=master)](https://travis-ci.org/kali/hue.rs)
[![CC](https://creativecommons.org/images/deed/cc_icon_black_x2.png)](https://creativecommons.org/licenses/by-sa/4.0/)
[![CCBY](https://creativecommons.org/images/deed/attribution_icon_black_x2.png)](https://creativecommons.org/licenses/by-sa/4.0/)
[![CCSA](https://creativecommons.org/images/deed/sa_black_x2.png)](https://creativecommons.org/licenses/by-sa/4.0/)

The easiest way to communicate with your Hue bridge using Rust.

## Current Features
 - discover bridge by querying philips hue website
 - compatible with all devices on Philips Hue Bridge (lights, sensors)
 - list devices with their state
 - simple actions on devices (on, off or bri/hue/sat, transition time on lights)
