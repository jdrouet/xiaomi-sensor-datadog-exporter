# Xiaomi Sensor Datadog Exporter

This comes with [this article](https://hackaday.com/2020/12/08/exploring-custom-firmware-on-xiaomi-thermometers/) that explains how to inject a custom firmware into the Xiaomi Thermometers.

With this service, you'll be able to listen for the bluetooth messages sent from the sensors and forward them to Datadog in order to monitor whatever you want.

## Building

```bash
# for current platform
docker build --tag xiaomi-sensor-exporter .
# for multi arch building
docker build -f multiarch.Dockerfile --platform linux/amd64,linux/arm/v8 --tag xiaomi-sensor-datadog-exporter .
```

