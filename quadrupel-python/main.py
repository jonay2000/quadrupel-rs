# run `build_python_bindings.sh` to create this library
# noinspection PyUnresolvedReferences
from quadrupel import parse_message_from_drone, create_message_for_drone
import serial
import time

class MessageQueue:
    def __init__(self):
        self.messages = []

    def send_at(self, n: int, msg: str):
        self.messages.append((n, create_message_for_drone(msg)))


def motor_message(motor: int, value: int) -> str:
    return f"""
    {{
        "MotorValue": {{
            "motor": "M{motor}",
            "value": {value} 
        }}
    }}
    """


if __name__ == '__main__':
    msgs = MessageQueue()
    msgs.send_at(3, """
    {
        "ChangeState": "IndividualMotorControl"
    }
    """)
    msgs.send_at(5, motor_message(0, 300))
    msgs.send_at(8, motor_message(1, 310))
    msgs.send_at(11, motor_message(2, 320))
    msgs.send_at(14, motor_message(3, 330))

    msgs.send_at(17, motor_message(0, 0))
    msgs.send_at(20, motor_message(1, 0))
    msgs.send_at(23, motor_message(2, 0))
    msgs.send_at(28, motor_message(3, 0))

    start = time.time()

    ser = serial.Serial("/dev/ttyUSB0")
    ser.baudrate = 115200

    while True:
        diff = time.time() - start

        for (t, msg) in msgs.messages:
            if t <= diff:
                print(f"SEND: {msg}")
                ser.write(msg)
                msgs.messages.remove((t, msg))

        rec = ser.read()
        if rec is not None and len(rec) > 0:
            for recb in rec:
                print(chr(recb), end="")
