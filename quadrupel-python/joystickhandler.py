from __future__ import annotations

import pygame
import pygame_widgets
from pygame_widgets.button import Button
from pygame_widgets.slider import Slider
from pygame_widgets.textbox import TextBox
import json
import threading
import typing
import os
from drone_visual import Drone
from pathlib import Path
import yaml

FILE_PATH = Path(os.path.dirname(os.path.realpath(__file__)))

if typing.TYPE_CHECKING:
    from main import Serial
from msgs import *

# Roll: Axis 0
# Pitch: Axis 1
# Yaw: Axis 2
# Height: Axis 3
# serial.get_latest_message()
# ty: Motor 0 relative up/down
# gh: Motor 1 relative up/down
# vb: Motor 2 relative up/down
# df: Motor 3 relative up/down

print_debug = False
message_frequency = 100  # In hertz

# TODO: Tune the offset step
# Increase/Decrease to be applied to the keyboard offset per key press
keyboard_offsets_step = {
    "lift": 10000,
    "roll": 10000,
    "pitch": 10000,
    "yaw": 10000,
    "M0": 1,
    "M1": 1,
    "M2": 1,
    "M3": 1
}

state_dictionary = {
    "Safe": 0,
    "Panic": 1,
    "Manual": 2,
    "Calibration": 3,
    "YawControl": 4,
    "FullControl": 5,
    "raw": 6,
    "height_control": 7,
    "wireless": 8,
    "IndividualMotorControl": 9,
}

name_dictionary = {
    "Safe": "safe",
    "Panic": "panic",
    "Manual": "manual",
    "Calibration": "calibration",
    "YawControl": "yaw control",
    "FullControl": "full control",
    "raw": "raw",
    "height_control": "height control",
    "wireless": "wireless",
    "IndividualMotorControl": "individual motor",
}

state_dictionary_reversed: dict[int, str] = {value: key for key, value in state_dictionary.items()}

# Offsets to be added to the joystick input
keyboard_offsets = {
    "lift": 0,
    "roll": 0,
    "pitch": 0,
    "yaw": 0,
}

# Key is allowed to go to the states in the value array
allowed_state_transition = {
    "Safe": [
        state_dictionary["Safe"],
        state_dictionary["Panic"],
        state_dictionary["Manual"],
        state_dictionary["Calibration"],
        state_dictionary["FullControl"],
        state_dictionary["IndividualMotorControl"],
        state_dictionary["YawControl"]
    ],
    "Panic": [],
    "Manual": [
        state_dictionary["Safe"]
    ],
    "Calibration": [
        state_dictionary["Safe"]
    ],
    "YawControl": [
        state_dictionary["Safe"]
    ],
    "FullControl": [
        state_dictionary["Safe"]
    ],
    "IndividualMotorControl": [
        state_dictionary["Safe"]
    ],
    "raw": [
        state_dictionary["Safe"]
    ],
    "height_control": [
        state_dictionary["Safe"]
    ],
    "wireless": [
        state_dictionary["Safe"]
    ]
}

message_control_parameters = {
    "TunePID": {
        "yaw_P": 0,
        "yaw_I": 0,
        "yaw_D": 0,
        "yaw_CAP": 0,
        "pitch_P": 0,
        "pitch_I": 0,
        "pitch_D": 0,
        "pitch_CAP": 0,
        "roll_P": 0,
        "roll_I": 0,
        "roll_D": 0,
        "roll_CAP": 0,
        "height_P": 0,
        "height_I": 0,
        "height_D": 0,
        "height_CAP": 0,
        "c1": 0,
        "c2": 0,
    }
}

message_individual_relative_control = {
    "MotorValueRel": {
        "motor": 0,
        "value": 0
    }
}
PID_MULTIPLIER = 2 ** 16

c_background = (0x81, 0x8D, 0x92)
c_visual = (0x58, 0x6A, 0x6A)


class JoystickHandler:
    def __init__(self, screen, ser, joystick=None):
        # Setup class variables
        self.current_state_raw = False
        self.current_state_height = False
        self.screen = screen
        self.width = screen.get_width()
        self.height = screen.get_height()
        self.joystick = joystick
        self.new_joystick_input = False
        self.new_pid_input = False
        self.current_state = "Safe"
        self.ser = ser

        self.initialize_ui()

        # Get the number of each type of input from the joystick
        self.joyButtons = dict()

        self.yaw = 0
        self.pitch = 0
        self.roll = 0
        self.lift = 0

        self.reported_battery_voltage = 0
        self.reported_mode = "Safe"
        self.reported_height = 0
        self.reported_motor_values = [0] * 4
        self.reported_i_buildup = [0] * 4
        self.reported_iteration_freq = 0
        self.mode_changed = 0
        self.reported_ypr = [0] * 3

        setup_path = FILE_PATH / "setup.yml"
        print(setup_path)
        if setup_path.exists():
            with open(setup_path, "r") as f:
                res = yaml.safe_load(f)
                current = res["current"]
                setup = res["setups"][current]

                keyboard_offsets["roll"] = setup["roll_trim"]
                keyboard_offsets["pitch"] = setup["pitch_trim"]
                keyboard_offsets["yaw"] = setup["yaw_trim"]
                keyboard_offsets["lift"] = setup["height_trim"]

                self.new_pid_input = True

                for name, i in self.textboxes.items():
                    value = setup[name]
                    i.setText(value)

                    try:
                        flt_v = float(value)
                    except ValueError:
                        print("bad float value")
                        return

                    int_v = int(flt_v * PID_MULTIPLIER)
                    self.new_pid_input = True
                    print(f"set {name} to {flt_v} ({int_v})")

                    message_control_parameters["TunePID"][name] = int_v
                    message_control_parameters["TunePID"][name.replace("pitch", "roll")] = int_v
                    print(message_control_parameters)

    def initialize_ui(self):
        pygame_widgets.WidgetHandler.getWidgets().clear()

        screen = self.screen
        width = screen.get_width()
        height = screen.get_height()

        fontsize = int(30 + width / 150)

        half_width = width // 2
        half_height = height // 2

        slider_border = 160
        slider_width = half_width - slider_border * 2
        slider_spacing = max(50, (half_height // 4) - (half_height // 8))
        slider_height = int(slider_spacing * 0.4)
        label_border = 10
        label_height = int(fontsize * 1.3)
        label_width = slider_border - 4 * label_border

        self.slider0 = Slider(screen, slider_border, slider_spacing, slider_width, slider_height, min=0, max=1000,
                              step=1, handleColour=(255, 255, 255),
                              initial=0)
        self.output0 = TextBox(screen, label_border, slider_spacing, label_width, label_height, fontSize=fontsize)

        self.slider1 = Slider(screen, slider_border, 2 * slider_spacing, slider_width, slider_height, min=0, max=1000,
                              step=1, handleColour=(255, 255, 255),
                              initial=0)
        self.output1 = TextBox(screen, label_border, 2 * slider_spacing, label_width, label_height, fontSize=fontsize)

        self.slider2 = Slider(screen, slider_border, 3 * slider_spacing, slider_width, slider_height, min=0, max=1000,
                              step=1, handleColour=(255, 255, 255),
                              initial=0)
        self.output2 = TextBox(screen, label_border, 3 * slider_spacing, label_width, label_height, fontSize=fontsize)

        self.slider3 = Slider(screen, slider_border, 4 * slider_spacing, slider_width, slider_height, min=0, max=1000,
                              step=1, handleColour=(255, 255, 255),
                              initial=0)
        self.output3 = TextBox(screen, label_border, 4 * slider_spacing, label_width, label_height, fontSize=fontsize)

        b_offset = half_width / 8
        b_width = b_offset
        b_start = half_width / 4
        b_height = half_height / 8
        b_v_start = half_height + b_height * 4
        b_v_distance = int(b_height * 1.5)

        self.submit_button = Button(screen, width / 8, b_v_start + b_v_distance, 200, 50, fontSize=fontsize,
                                    text="submit", onClick=self.submit)

        self.label4 = TextBox(screen, b_start, b_v_start, 50, 50, fontSize=fontsize)
        self.label4.disable()
        self.label4.setText("P")

        self.label4 = TextBox(screen, b_start + b_width, b_v_start, 50, 50, fontSize=fontsize)
        self.label4.disable()
        self.label4.setText("I")

        self.label4 = TextBox(screen, b_start + b_width * 2, b_v_start, 50, 50, fontSize=fontsize)
        self.label4.disable()
        self.label4.setText("D")

        self.label4 = TextBox(screen, b_start + b_width * 3, b_v_start, 100, 50, fontSize=fontsize)
        self.label4.disable()
        self.label4.setText("CAP")

        self.label1 = TextBox(screen, b_start - b_width, b_v_start - b_v_distance, 130, 70, fontSize=fontsize)
        self.label1.disable()
        self.label1.setText("height")

        self.height_P_tb = TextBox(screen, b_start, b_v_start - b_v_distance, 80, 80, fontSize=fontsize)
        self.height_I_tb = TextBox(screen, b_start + b_width, b_v_start - b_v_distance, 80, 80, fontSize=fontsize)
        self.height_D_tb = TextBox(screen, b_start + b_width * 2, b_v_start - b_v_distance, 80, 80, fontSize=fontsize)
        self.height_CAP_tb = TextBox(screen, b_start + b_width * 3, b_v_start - b_v_distance, 80, 80, fontSize=fontsize)

        self.label2 = TextBox(screen, b_start - b_width, b_v_start - b_v_distance * 2, 130, 70, fontSize=fontsize)
        self.label2.disable()
        self.label2.setText("pitch")

        self.pitch_P_tb = TextBox(screen, b_start, b_v_start - b_v_distance * 2, 80, 80, fontSize=fontsize)
        self.pitch_I_tb = TextBox(screen, b_start + b_width, b_v_start - b_v_distance * 2, 80, 80, fontSize=fontsize)
        self.pitch_D_tb = TextBox(screen, b_start + b_width * 2, b_v_start - b_v_distance * 2, 80, 80,
                                  fontSize=fontsize)
        self.pitch_CAP_tb = TextBox(screen, b_start + b_width * 3, b_v_start - b_v_distance * 2, 80, 80,
                                    fontSize=fontsize)

        self.label3 = TextBox(screen, b_start - b_width, b_v_start - b_v_distance * 3, 130, 70, fontSize=fontsize)
        self.label3.disable()
        self.label3.setText("yaw")

        self.yaw_P_tb = TextBox(screen, b_start, b_v_start - b_v_distance * 3, 80, 80, fontSize=fontsize)
        self.yaw_I_tb = TextBox(screen, b_start + b_width, b_v_start - b_v_distance * 3, 80, 80, fontSize=fontsize)
        self.yaw_D_tb = TextBox(screen, b_start + b_width * 2, b_v_start - b_v_distance * 3, 80, 80, fontSize=fontsize)
        self.yaw_CAP_tb = TextBox(screen, b_start + b_width * 3, b_v_start - b_v_distance * 3, 80, 80,
                                  fontSize=fontsize)


        self.label4 = TextBox(screen, b_start - b_width, b_v_start - b_v_distance * 4, 80, 70, fontSize=fontsize)
        self.label4.disable()
        self.label4.setText("c1:")
        self.label4 = TextBox(screen, b_start + b_width, b_v_start - b_v_distance * 4, 80, 70, fontSize=fontsize)
        self.label4.disable()
        self.label4.setText("c2:")

        self.c1_tb = TextBox(screen, b_start , b_v_start - b_v_distance * 4, 80, 80, fontSize=fontsize)
        self.c2_tb = TextBox(screen, b_start + b_width * 2, b_v_start - b_v_distance * 4, 80, 80, fontSize=fontsize)

        self.previous_motor0 = 0
        self.previous_motor1 = 0
        self.previous_motor2 = 0
        self.previous_motor3 = 0

        self.output0.disable()
        self.output1.disable()
        self.output2.disable()
        self.output3.disable()


        self.stats = []
        for i in range(8):
            bw = int(half_width / 2.5)
            b = TextBox(screen, half_width + (bw - 20) * (i // 4), half_height + (i % 4) * int(fontsize * 1.3) + 40, bw,
                        int(fontsize * 1.3), fontSize=fontsize)
            b.disable()
            self.stats.append(
                b
            )

        def transition(state: int):
            def f():
                self.change_state(state, None)
                self.initialize_ui()

            return f

        def send_message(msg: str):
            def f():
                self.ser.send(f"\"{msg}\"")

            return f

        buttons = []
        allowed = [i for i in allowed_state_transition[self.current_state] if
                   state_dictionary_reversed[i] != self.current_state]
        i = 0
        for i in range(8):
            if i >= len(allowed):
                break

            txt = name_dictionary[state_dictionary_reversed[allowed[i]]]

            b = Button(screen, half_width + 450 * (i // 4),
                       half_height + (i % 4) * int(fontsize * 1.3) + 40 + 5 * int(fontsize * 1.3), 400,
                       int(fontsize * 1.3), fontSize=fontsize, text=txt)
            b.onClick = transition(allowed[i])

        for txt in ["FlashStopRecording", "FlashStartRecording", "FlashRead"]:
            b = Button(screen, half_width + 450 * (i // 4),
                       half_height + (i % 4) * int(fontsize * 1.3) + 40 + 5 * int(fontsize * 1.3), 400,
                       int(fontsize * 1.3), fontSize=fontsize, text=txt)
            b.onClick = send_message(txt)
            i += 1

        self.drone_visual = Drone(screen, (half_width, half_height), (half_width, 0))

        self.textboxes = {i.replace("_tb", ""): x for i in self.__dict__ if
                          isinstance(x := getattr(self, i), TextBox) and not x._disabled and "tb" in i}
        for k, v in self.textboxes.items():
            res = message_control_parameters['TunePID'][k] / PID_MULTIPLIER
            if res < 0:
                v.setText(f"{res:.02f}")
            else:
                v.setText(f"{res:.00f}")

    def submit(self):
        for name, i in self.textboxes.items():
            value = i.getText()

            print(name, value)

            try:
                flt_v = float(value)
            except ValueError:
                print("bad float value")
                return

            int_v = int(flt_v * PID_MULTIPLIER)
            self.new_pid_input = True
            print(f"set {name} to {flt_v} ({int_v})")

            message_control_parameters["TunePID"][name] = int_v
            message_control_parameters["TunePID"][name.replace("pitch", "roll")] = int_v
            print(message_control_parameters)

    def tb_not_selected(self):
        res = not any(i.selected for i in self.textboxes.values())
        print(res)
        return res

    def run(self):
        running = True  # This is the main "loop running" variable -- set to false to exit the loop

        if print_debug:
            print("axis:", self.joystick.get_numaxes(), "button:", self.joystick.get_numbuttons(), "hat:",
                  self.joystick.get_numhats(), "ball:", self.joystick.get_numballs())

        while running:  # Loop until "running" becomes false
            prev_state = self.current_state

            self.output0.setText(self.slider0.getValue())
            self.output1.setText(self.slider1.getValue())
            self.output2.setText(self.slider2.getValue())
            self.output3.setText(self.slider3.getValue())
            events = pygame.event.get()

            if os.path.exists(FILE_PATH / "messages.txt"):
                os.rename(FILE_PATH / "messages.txt", FILE_PATH / "messages_cp.txt")
                with open(FILE_PATH / "messages_cp.txt", "r") as f:
                    for str_msg in f.readlines():
                        try:
                            msg = json.loads(str_msg)
                        except Exception as e:
                            print(e)
                            continue
                        print(msg)

                        if (v := msg.get("StateInformation")) is not None:
                            if self.mode_changed <= 0:
                                self.current_state = v["state"]
                                self.current_state_height = v["height_mode"]
                                self.current_state_raw = v["raw_mode"]
                            else:
                                self.mode_changed -= 1

                            self.reported_height = v["height"]
                            self.reported_battery_voltage = v["battery"] / 100
                            self.reported_iteration_freq = 1_000_000 / v["dt"]
                            self.reported_i_buildup = v["i_buildup"]
                            self.reported_ypr = [i / (1 << 16) for i in v["sensor_ypr"]]
                        else:
                            print("msg: ", msg)

            approved_events = []

            for event in events:  # Get all of the events from the queue

                if event.type != pygame.KEYDOWN:
                    approved_events.append(event)
                elif event.type == pygame.KEYDOWN and not (
                        ord('a') <= event.key <= ord('z') or ord('A') <= event.key <= ord('Z') or event.key == 27):
                    approved_events.append(event)

                if event.type == pygame.JOYAXISMOTION:  # Main axis movement
                    self.new_joystick_input = True
                    if print_debug:
                        print(
                            "Axis0:", self.joystick.get_axis(0),
                            "Axis1:", self.joystick.get_axis(1),
                            "Axis2:", self.joystick.get_axis(2),
                            "Axis3", self.joystick.get_axis(3)
                        )

                elif event.type == pygame.JOYBUTTONDOWN:  # Buttons pressed
                    self.joyButtons[event.button] = True
                    if print_debug:
                        print("Button", event.button + 1, "pressed down")
                    if event.button == 0:
                        if print_debug:
                            print("Abort/Exit")
                        self.ser.send(change_state(state_dictionary_reversed[1]))

                elif event.type == pygame.JOYBUTTONUP:  # Buttons released
                    self.joyButtons[event.button] = False
                    if print_debug:
                        print("Button", event.button + 1, "pressed up")

                elif event.type == pygame.JOYHATMOTION:
                    if print_debug:
                        print("hat:", event.hat, "value:", event.value)
                elif event.type == pygame.QUIT:
                    os._exit(0)

                elif event.type == pygame.VIDEORESIZE:
                    self.width = self.screen.get_width()
                    self.height = self.screen.get_height()
                    self.initialize_ui()

                elif event.type == pygame.KEYDOWN:
                    if print_debug:
                        print("Button", event.key, "pressed down")
                    if event.key == 27 or (event.key == ord('1') and self.tb_not_selected()):
                        print("Abort/Exit")
                        self.ser.send(change_state(state_dictionary_reversed[1]))

                    if event.key == ord('a'):
                        if print_debug:
                            print("lift offset up")
                        keyboard_offsets["lift"] += keyboard_offsets_step["lift"]
                        self.new_joystick_input = True
                    if event.key == ord('z'):
                        if print_debug:
                            print("lift offset down")
                        keyboard_offsets["lift"] -= keyboard_offsets_step["lift"]
                        self.new_joystick_input = True

                    if event.key == 1073741904:  # Left arrow key
                        if print_debug: print("roll offset up")
                        keyboard_offsets["roll"] += keyboard_offsets_step["roll"]
                        self.new_joystick_input = True
                        print(keyboard_offsets)
                    if event.key == 1073741903:  # Right arrow key
                        if print_debug: print("roll offset down")
                        keyboard_offsets["roll"] -= keyboard_offsets_step["roll"]
                        self.new_joystick_input = True
                        print(keyboard_offsets)

                    if event.key == 1073741905:  # Down arrow key
                        if print_debug: print("pitch offset up")
                        keyboard_offsets["pitch"] += keyboard_offsets_step["pitch"]
                        self.new_joystick_input = True
                        print(keyboard_offsets)
                    if event.key == 1073741906:  # Up arrow key
                        if print_debug: print("pitch offset down")
                        keyboard_offsets["pitch"] -= keyboard_offsets_step["pitch"]
                        self.new_joystick_input = True
                        print(keyboard_offsets)

                    if event.key == ord('w'):
                        if print_debug:
                            print("yaw offset up")
                        keyboard_offsets["yaw"] += keyboard_offsets_step["yaw"]
                        self.new_joystick_input = True
                        print(keyboard_offsets)
                    if event.key == ord('q'):
                        if print_debug:
                            print("yaw offset down")
                        keyboard_offsets["yaw"] -= keyboard_offsets_step["yaw"]
                        self.new_joystick_input = True
                        print(keyboard_offsets)

                    if event.key == ord('u'):
                        if print_debug: print("yaw control P offset up")
                        message_control_parameters["TunePID"]["yaw_P"] += keyboard_offsets_step["yaw_P"]
                        self.ser.send(json.dumps(message_control_parameters))
                    if event.key == ord('j'):
                        if print_debug: print("yaw control P offset down")
                        message_control_parameters["TunePID"]["yaw_P"] -= keyboard_offsets_step["yaw_P"]
                        self.ser.send(json.dumps(message_control_parameters))

                    if event.key == ord('i'):
                        if print_debug: print("roll/pitch P1 offset up")
                        message_control_parameters["TunePID"]["roll_P"] += keyboard_offsets_step["roll_P"]
                        self.ser.send(json.dumps(message_control_parameters))
                    if event.key == ord('k'):
                        if print_debug: print("roll/pitch P1 offset down")
                        message_control_parameters["TunePID"]["roll_P"] -= keyboard_offsets_step["roll_P"]
                        self.ser.send(json.dumps(message_control_parameters))

                    if event.key == ord('o'):
                        if print_debug: print("roll/pitch P2 offset up")
                        message_control_parameters["TunePID"]["yaw_P"] += keyboard_offsets_step["yaw_P"]
                        self.ser.send(json.dumps(message_control_parameters))
                    if event.key == ord('l'):
                        if print_debug: print("roll/pitch P2 offset down")
                        message_control_parameters["TunePID"]["yaw_P"] -= keyboard_offsets_step["yaw_P"]
                        self.ser.send(json.dumps(message_control_parameters))

                    if event.key == ord('y'):
                        if print_debug: print("M0 offset up")
                        message_individual_relative_control["MotorValueRel"]["motor"] = 0
                        message_individual_relative_control["MotorValueRel"]["value"] = 1
                        self.ser.send(json.dumps(message_individual_relative_control))
                    if event.key == ord('t'):
                        if print_debug: print("M0 offset down")
                        message_individual_relative_control["MotorValueRel"]["motor"] = 0
                        message_individual_relative_control["MotorValueRel"]["value"] = -1
                        self.ser.send(json.dumps(message_individual_relative_control))

                    if event.key == ord('h'):
                        if print_debug: print("M1 offset up")
                        message_individual_relative_control["MotorValueRel"]["motor"] = 1
                        message_individual_relative_control["MotorValueRel"]["value"] = 1
                        self.ser.send(json.dumps(message_individual_relative_control))
                    if event.key == ord('g'):
                        if print_debug: print("M1 offset down")
                        message_individual_relative_control["MotorValueRel"]["motor"] = 1
                        message_individual_relative_control["MotorValueRel"]["value"] = -1
                        self.ser.send(json.dumps(message_individual_relative_control))

                    if event.key == ord('b'):
                        if print_debug: print("M2 offset up")
                        message_individual_relative_control["MotorValueRel"]["motor"] = 2
                        message_individual_relative_control["MotorValueRel"]["value"] = 1
                        self.ser.send(json.dumps(message_individual_relative_control))
                    if event.key == ord('v'):
                        if print_debug: print("M2 offset down")
                        message_individual_relative_control["MotorValueRel"]["motor"] = 2
                        message_individual_relative_control["MotorValueRel"]["value"] = -1
                        self.ser.send(json.dumps(message_individual_relative_control))

                    if event.key == ord('f'):
                        if print_debug: print("M3 offset up")
                        message_individual_relative_control["MotorValueRel"]["motor"] = 3
                        message_individual_relative_control["MotorValueRel"]["value"] = 1
                        self.ser.send(json.dumps(message_individual_relative_control))
                    if event.key == ord('d'):
                        if print_debug: print("M3 offset down")
                        message_individual_relative_control["MotorValueRel"]["motor"] = 3
                        message_individual_relative_control["MotorValueRel"]["value"] = -1
                        self.ser.send(json.dumps(message_individual_relative_control))

                    if ord('0') <= event.key <= ord('9'):
                        if print_debug:
                            print("Change to state", state_dictionary_reversed[int(chr(event.key))])
                        self.change_state(int(chr(event.key)), event)

                    if event.key == ord('['):
                        if print_debug: print("land")
                        self.ser.send(auto_land())

            if self.current_state == "Panic":
                self.screen.fill((220, 50, 50))
            else:
                self.screen.fill(c_background)

            pygame.draw.rect(self.screen, c_visual, (self.width // 2, 0, self.width // 2, self.height // 2), 0, 0, 0, 0,
                             10)
            self.output0.setText("M0: " + str(self.slider0.getValue()))
            self.output1.setText("M1: " + str(self.slider1.getValue()))
            self.output2.setText("M2: " + str(self.slider2.getValue()))
            self.output3.setText("M3: " + str(self.slider3.getValue()))
            self.drone_visual.rot = self.reported_ypr
            self.drone_visual.draw()

            self.stats[0].setText(f"Voltage: {self.reported_battery_voltage:.2f}V")
            self.stats[1].setText(f"Freq: {self.reported_iteration_freq:.2f}")
            self.stats[2].setText(f"height: {self.reported_height:.2f}")

            flag_h = "H" if self.current_state_height else ""
            flag_r = "R" if self.current_state_raw else ""
            self.stats[3].setText(f"mode: {name_dictionary[self.current_state]} {flag_h}{flag_r}")
            self.stats[4].setText(
                f"i: {self.reported_i_buildup[0]:.2f} {self.reported_i_buildup[1]:.2f} {self.reported_i_buildup[2]:.2f} {self.reported_i_buildup[3]:.2f}")

            self.stats[5].setText(
                f"yprl: {self.yaw / 5000:.2f} {self.pitch / 5000:.2f} {self.roll / 5000:.2f} {self.lift / 10000:.2f}")
            self.stats[6].setText(
                f"trim: {keyboard_offsets['roll']:.0f} {keyboard_offsets['pitch']:.0f} {keyboard_offsets['yaw']:.0f} {keyboard_offsets['lift']:.0f}"
            )

            if self.can_change_mode():
                self.stats[5].colour = (50, 220, 50)
            else:
                self.stats[5].colour = (220, 50, 50)

            pygame_widgets.update(approved_events)
            pygame.display.flip()

            if self.current_state != prev_state:
                self.initialize_ui()

    def can_change_mode(self):
        ypr_margin = 30_000
        lift_margin = 5_000

        if self.joystick is None:
            return False

        return (-ypr_margin <= ((-1 * self.joystick.get_axis(0)) * pow(2, 19)) <= ypr_margin
                and -ypr_margin <= ((self.joystick.get_axis(1)) * pow(2, 19)) <= ypr_margin
                and -ypr_margin <= ((self.joystick.get_axis(2)) * pow(2, 19)) <= ypr_margin
                and ((-1 * self.joystick.get_axis(3) + 1) * pow(2, 19)) <= lift_margin)

    def change_state(self, state: int, event=None):
        if event is not None and event.key == ord('7'):  # Specific behavior for height control toggle
            self.ser.send(toggle_height_control())
        elif self.can_change_mode() and self.tb_not_selected():
            if event is not None and event.key == ord('6'):
                self.ser.send(toggle_raw_mode())
            else:
                state = state_dictionary_reversed[state]
                self.current_state = state
                self.ser.send(change_state(state))
            self.mode_changed = 2

        # TODO

    # Send the data to the drone periodically based on joystick changes
    def send_data(self, ser):
        threading.Timer(1 / message_frequency, self.send_data, args=(ser,)).start()
        if self.new_joystick_input and self.joystick is not None:
            if -40000 <= ((-1 * self.joystick.get_axis(0)) * pow(2, 19)) + keyboard_offsets["roll"] <= 40000:
                self.roll = 0
            else:
                self.roll = round((-1 * self.joystick.get_axis(0)) * pow(2, 19)) + keyboard_offsets["roll"]
            if -40000 <= ((self.joystick.get_axis(1)) * pow(2, 19)) + keyboard_offsets["pitch"] <= 40000:
                self.pitch = 0
            else:
                self.pitch = round((self.joystick.get_axis(1)) * pow(2, 19)) + keyboard_offsets["pitch"]

            if -40000 <= ((self.joystick.get_axis(2)) * pow(2, 19)) + keyboard_offsets["yaw"] <= 40000:
                self.yaw = 0
            else:
                self.yaw = round((self.joystick.get_axis(2)) * pow(2, 19)) + \
                           keyboard_offsets["yaw"]
            if ((-1 * self.joystick.get_axis(3) + 1) * pow(2, 19)) + keyboard_offsets["lift"] <= 50000:
                self.lift = 0
            else:
                self.lift = round((-1 * self.joystick.get_axis(3) + 1) * pow(2, 19)) + keyboard_offsets["lift"] - 50000
            ser.send(joystick_message(self.yaw, self.pitch, self.roll, self.lift))
            self.new_joystick_input = False
        elif self.slider0.getValue() != self.previous_motor0:
            ser.send(motor_message(0, self.slider0.getValue()))
            self.previous_motor0 = self.slider0.getValue()
        elif self.slider1.getValue() != self.previous_motor1:
            ser.send(motor_message(1, self.slider1.getValue()))
            self.previous_motor1 = self.slider1.getValue()
        elif self.slider2.getValue() != self.previous_motor2:

            ser.send(motor_message(2, self.slider2.getValue()))
            self.previous_motor2 = self.slider2.getValue()
        elif self.slider3.getValue() != self.previous_motor3:
            ser.send(motor_message(3, self.slider3.getValue()))
            self.previous_motor3 = self.slider3.getValue()
        elif self.new_pid_input:
            print("send pid update")
            self.new_pid_input = False
            ser.send(json.dumps(message_control_parameters))
        else:
            ser.send(heartbeat())


def main(ser):
    # Setup the joysticks
    pygame.init()
    pygame.joystick.init()
    stickCount = pygame.joystick.get_count()  # How many joysticks are connected?
    for index in range(stickCount):  # Print the name of each joystick
        joystick = pygame.joystick.Joystick(index)
        print("{0}) {1}".format(index, joystick.get_name()))
    # Get the user's selection, and exit if they just press enter
    if stickCount == 0:
        selected = None
    elif stickCount == 1:
        selected = 0
    else:
        selected = input("Enter a joystick number or just Enter to exit:")

    if selected is not None:
        if selected == "":
            os._exit(0)
        # Convert the selection into an integer
        index = int(selected)
        # Initialize the selected joystick
        joystick = pygame.joystick.Joystick(index)
        joystick.init()
    else:
        joystick = None

    pygame.init()
    screen = pygame.display.set_mode((0, 0), pygame.WINDOWMAXIMIZED | pygame.RESIZABLE)
    pygame.display.set_caption("Joystick tester")
    # # Initialize the display class
    window = JoystickHandler(screen, ser, joystick)

    # Start the main loop
    window.send_data(ser)
    window.run()
