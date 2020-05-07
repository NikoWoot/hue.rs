# Philips Hue library in Rust

[![Build Status](https://travis-ci.org/kali/hue.rs.svg?branch=master)](https://travis-ci.org/kali/hue.rs)
[![CC](https://search.creativecommons.org/static/img/cc_icon.svg)
[![CCBY](https://search.creativecommons.org/static/img/cc-by_icon.svg)
[![CCSA](https://search.creativecommons.org/static/img/cc-sa_icon.svg)

The easiest way to communicate with your Hue bridge using Rust.

## Current Features
 - discover bridge by querying philips hue website
 - compatible with all devices on Philips Hue Bridge (lights, sensors)
 - list devices with their state
 - simple actions on devices (on, off or bri/hue/sat, transition time on lights)
