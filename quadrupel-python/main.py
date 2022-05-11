import json
import queue
import threading
import time
from collections import deque
import os

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
from joystickhandler import main
import msgs


class Serial:
    def __init__(self, serport="/dev/ttyUSB0"):
        try:
            self.ser = serial.Serial(serport)
            self.ser.baudrate = 115200
        except Exception as e:
            print(traceback.format_exception(type(e), e, e.__traceback__))
            self.ser = None

        self.q = multiprocessing.Queue()
        multiprocessing.Process(target=self.read, args=(self.q,)).start()

        self.do_heartbeat = False

        threading.Thread(target=self.try_heartbeat).start()

    def try_heartbeat(self):
        while self.q.empty():
            time.sleep(0.1)
        print("start heartbeat")
        self.start_heartbeat()

    def stop_heartbeat(self):
        self.do_heartbeat = False

    def start_heartbeat(self):
        self.do_heartbeat = True
        threading.Timer(0.1, self.heartbeat).start()
        self.q = multiprocessing.Queue()
        multiprocessing.Process(target=self.read, args=(self.q,)).start()

    def heartbeat(self):
        self.send(msgs.heartbeat())

        if self.do_heartbeat:
            threading.Timer(0.1, self.heartbeat).start()

    def send(self, msg: str):
        try:
            if self.ser is not None:
                r = create_message_for_drone(msg)
                self.ser.write(r)
        except:
            print("failed to send", msg)
            os._exit(1)


    def get_latest_message(self) -> dict | None:
        try:
            return self.q.get(timeout=0.01)
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
                            elif (v := decoded_msg.get("StateInformation")) is not None:
                                # TODO: Decode 16-bit fixedpoint
                                print(f"State: {v}")
                            # TODO: Uncommenting this causes code to crash later?
                            # else:
                            #     q.put(decoded_msg)
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
