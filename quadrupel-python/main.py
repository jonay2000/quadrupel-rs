# run `build_python_bindings.sh` to create this library
# noinspection PyUnresolvedReferences
import threading

from quadrupel import parse_message_from_drone, create_message_for_drone

import traceback
import serial
import multiprocessing
from midimotorcontroller import main
import msgs


class Serial:
    def __init__(self, serport="/dev/ttyUSB0"):
        try:
            self.ser = serial.Serial(serport)
            self.ser.baudrate = 115200
        except:
            print(traceback.format_exception())
            self.ser = None

        threading.Timer(0.1, self.heartbeat).start()

        self.do_heartbeat = True

    def heartbeat(self):
        self.send(msgs.heartbeat())

        if not self.do_heartbeat:
            threading.Timer(0.1, self.heartbeat).start()

    def send(self, msg: str):
        if self.ser is not None:
            self.ser.write(create_message_for_drone(msg))

    def read(self):
        if self.ser is not None:
            while True:
                rec = self.ser.read()
                if rec is not None and len(rec) > 0:
                    for recb in rec:
                        print(chr(recb), end="")


if __name__ == '__main__':
    ser = Serial()

    multiprocessing.Process(target=ser.read).start()
    main(ser)

    # while True:
    #     diff = time.time() - start
    #
    #     for (t, msg) in msgs.messages:
    #         if t <= diff:
    #             print(f"SEND: {msg}")
    #             ser.write(msg)
    #             msgs.messages.remove((t, msg))
    #

