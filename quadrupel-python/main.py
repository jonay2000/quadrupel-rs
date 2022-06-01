import json
import queue
import threading
import time
from collections import deque
import os
from pathlib import Path

FILE_PATH = Path(os.path.dirname(os.path.realpath(__file__)))
if os.path.exists(FILE_PATH / "messages.txt"):
    os.remove(FILE_PATH / "messages.txt")
if os.path.exists(FILE_PATH / "messages_cp.txt"):
    os.remove(FILE_PATH / "messages_cp.txt")


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
            self.ser = serial.Serial(serport, baudrate=115200)
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
        lost_count = False

        def read_more():
            try:
                r = self.ser.read()
            except Exception as e:
                print(e)
                return None
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
                            if msg[0] == 0xab:
                                msg = msg[1:]
                            decoded_msg = json.loads(msg)

                            if (v := decoded_msg.get("Log")) is not None:
                                print(bytes(v).decode("utf-8"), end="")
                            # elif (v := decoded_msg.get("StateInformation")) is not None:
                            #     TODO: Decode 16-bit fixedpoint
                                # print(f"State: {v}")
                            elif (v := decoded_msg.get("FlashPacket")) is not None:
                                print(f"Flash packet: {v}")
                                with open("flash_data.txt", "a") as f:
                                    print(v["Data"], file=f)
                            # TODO: Uncommenting this causes code to crash later?
                            else:
                                with open(FILE_PATH / "messages.txt", "a") as f:
                                    f.writelines([msg])
                                # print(decoded_msg)
                    except Exception as e:
                        print(e)
                        lost_count = True
                    buf = []
                    continue

                if (b := get_byte()) is not None and not lost_count:
                    if not receiving:
                        if b == 0xab:
                            b = get_byte()

                        target_length = b
                        receiving = True
                    else:
                        buf.append(b)
                else:
                    print("lost count")
                    while (b := get_byte()) != 0xab:
                        if b is not None:
                            print(f"{b:02x}", end=" ")
                        pass
                    lost_count = False


if __name__ == '__main__':
    ser = Serial()
    main(ser)
