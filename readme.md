# bms-to-ffbeast
Exposes telemetry required for FFB effects to the FFBeast Commander from the BMS simulator

## How to use
Just download `.exe` file for the latest release and run it

It will automaticly connect to the BMS when it started and to the FFBeast Commander

It will also automatically turn off after you exit the BMS

## Know issues

Since it based on `FlightData` instead of `FlightData2` it uses only first engine RPMs for telemetry and there is no flaps position data (feel free to make PR fixing that)