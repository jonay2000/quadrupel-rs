# run `build_python_bindings.sh` to create this library
# noinspection PyUnresolvedReferences
from quadrupel import parse_message_from_drone, create_message_for_drone
import serial

if __name__ == '__main__':
    ser = serial.Serial("/dev/ttyUSB0")
    ser.baudrate = 115200
    write = create_message_for_drone("""
    {
        "TargetYaw": 10
    }
    """)
    print(f"SEND: {write}")
    ser.write(write)

    while True:
        rec = ser.read()
        if rec is not None and len(rec) > 0:
            for recb in rec:
                print("REC: " + str(recb))
