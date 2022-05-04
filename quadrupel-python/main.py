import json
import queue
import threading
import platform
from collections import deque

try:
    # run `build_python_bindings.sh` to create this library
    # noinspection PyUnresolvedReferences
    from quadrupel import parse_message_from_drone, create_message_for_drone
except ImportError:
    def parse_message_from_drone(msg):
        return bytearray()

    def create_message_for_drone(bytes):
        return ""

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
        except Exception as e:
            print(traceback.format_exception(type(e), e, e.__traceback__))
            self.ser = None

        threading.Timer(0.1, self.heartbeat).start()
        self.q = multiprocessing.Queue()
        multiprocessing.Process(target=self.read, args=(self.q,)).start()

        self.do_heartbeat = True

    def heartbeat(self):
        self.send(msgs.heartbeat())

        if self.do_heartbeat:
            threading.Timer(0.1, self.heartbeat).start()

    def send(self, msg: str):
        if self.ser is not None:
            r = create_message_for_drone(msg)
            self.ser.write(r)

    def get_latest_message(self) -> dict | None:
        try:
            return self.q.get(timeout=0.1)
        except queue.Empty:
            return None

    def read(self, q: multiprocessing.Queue):
        buf = []
        target_length = 0
        receiving = False
        incoming = deque()

        def read_more():
            r = self.ser.read()
            if r is not None:
                incoming.extend(r)

        def get_byte():
            read_more()
            if len(incoming) > 0:
                return incoming.popleft()
            else:
                return None

        if self.ser is not None:
            while True:
                if receiving and len(buf) == target_length:
                    receiving = False
                    try:
                        if len(buf) != 0:
                            msg, num = parse_message_from_drone(bytes(buf))
                            decoded_msg = json.loads(msg)

                            if (v := decoded_msg.get("Log")) is not None:
                                print(bytes(v).decode("utf-8"), end="")
                            else:
                                q.put(decoded_msg)
                    except Exception as e:
                        print(e)
                    buf = []
                    continue

                if (b := get_byte()) is not None:
                    if not receiving:
                        target_length = b
                        receiving = True
                    else:
                        buf.append(b)


if __name__ == '__main__':
    ser = Serial()
    main(ser)
