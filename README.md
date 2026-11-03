# 2026_10_31_rust_esp32_to_uart_bidirectional

Allows an ESP32-S3 XIAO to communicate bidirectionally over UART with a device such as a Micro:bit, using UDP.
In the broadcast version with the ESP32-S3 WROOM, you don't need to listen to the device:
https://github.com/EloiStree/2026_10_31_rust_esp32_to_uart_broadcast/

But here, we are dealing with a case where we want one connection to one device that can provide telemetry.
[<img width="7680" height="4320" alt="image" src="https://github.com/user-attachments/assets/5b6a9936-be55-4f73-99ae-1e77e992bffe" />](https://wiki.seeedstudio.com/xiao_esp32s3_getting_started/)
<img width="4080" height="3072" alt="image" src="https://github.com/user-attachments/assets/c2819819-e687-46ae-9bd3-f35c767c228b" />


https://wiki.seeedstudio.com/xiao_esp32s3_getting_started/

The idea is to send information over UDP using the device's Wi-Fi mDNS address. The device then broadcasts UDP messages back to all devices communicating with it, using a configured port.
It is designed to act as a Wi-Fi hub between learning environments such as Godot (or other programming languages) and Micro:bit-based mini robots, such as the KS4036.
