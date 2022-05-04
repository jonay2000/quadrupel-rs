from __future__ import annotations

import pygame
import pygame_widgets
from pygame_widgets.slider import Slider
from pygame_widgets.textbox import TextBox
import json
import threading
import typing
if typing.TYPE_CHECKING:
    from main import Serial
from msgs import motor_message, heartbeat

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
    "lift": 1,
    "roll": 1,
    "pitch": 1,
    "yaw": 1,
    "yaw_P": 1,
    "roll_pitch_P1": 1,
    "roll_pitch_P2": 1,
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
    "yaw_control": 4,
    "FullControl": 5,
    "raw": 6,
    "height_control": 7,
    "wireless": 8,
    "IndividualMotorControl": 9,
}

state_dictionary_reversed = {
    0: "Safe",
    1: "Panic",
    2: "Manual",
    3: "Calibration",
    4: "yaw_control",
    5: "FullControl",
    6: "raw",
    7: "height_control",
    8: "wireless",
    9: "IndividualMotorControl",
}

# Offsets to be added to the joystick input
keyboard_offsets = {
    "lift": 0,
    "roll": 0,
    "pitch": 0,
    "yaw": 0,
    "yaw_P": 0,
    "roll_pitch_P1": 0,
    "roll_pitch_P2": 0
}

# TODO: Populate the dictionary
# Key is allowed to go to the states in the value array
allowed_state_transition = {
    "Safe": [
        state_dictionary["Safe"],
        state_dictionary["Panic"],
        state_dictionary["Manual"],
        state_dictionary["Calibration"]
    ],
    "Panic": [
        state_dictionary["Safe"]
    ],
    "manual": [
        state_dictionary["Safe"]
    ],
    "Calibration": [
        state_dictionary["Safe"]
    ],
    "yaw_control": [
        state_dictionary["Safe"]
    ],
    "FullControl": [
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

message_joystick = {
    "TargetAttitude": {
        "roll": 0,
        "pitch": 0,
        "yaw": 0,
        "lift": 0
    }
}

message_control_parameters = {
    "TunePID": {
        "yaw_P": 0,
        "roll_pitch_P1": 0,
        "roll_pitch_P2": 0
    }
}

message_state_change = {
    "ChangeState": "Safe"
}

message_individual_relative_control = {
    "MotorValueRel": {
        "motor": 0,
        "value": 0
    }
}


class JoystickHandler:
    def __init__(self, screen, joystick=None):
        # Setup class variables
        self.screen = screen
        self.width = screen.get_width()
        self.height = screen.get_height()
        self.joystick = joystick
        self.new_joystick_input = False

        # pygame.font.init()
        # self.labelFont = pygame.font.SysFont("", 15, False, False)

        self.slider0 = Slider(screen, 100, 100, 800, 40, min=0, max=1000, step=1, handleColour=(255, 255, 255), initial=0)
        self.output0 = TextBox(screen, 475, 200, 100, 50, fontSize=30)
        self.slider1 = Slider(screen, 100, 300, 800, 40, min=0, max=1000, step=1, handleColour=(255, 255, 255), initial=0)
        self.output1 = TextBox(screen, 475, 400, 100, 50, fontSize=30)
        self.slider2 = Slider(screen, 100, 500, 800, 40, min=0, max=1000, step=1, handleColour=(255, 255, 255), initial=0)
        self.output2 = TextBox(screen, 475, 600, 100, 50, fontSize=30)
        self.slider3 = Slider(screen, 100, 700, 800, 40, min=0, max=1000, step=1, handleColour=(255, 255, 255), initial=0)
        self.output3 = TextBox(screen, 475, 800, 100, 50, fontSize=30)

        self.previous_motor0 = 0
        self.previous_motor1 = 0
        self.previous_motor2 = 0
        self.previous_motor3 = 0

        self.output0.disable()
        self.output1.disable()
        self.output2.disable()
        self.output3.disable()

        # Get the number of each type of input from the joystick
        self.joyButtons = dict()
        self.current_state = state_dictionary["Safe"]

    def run(self, ser: Serial):
        running = True  # This is the main "loop running" variable -- set to false to exit the loop

        if print_debug:
            print("axis:", self.joystick.get_numaxes(), "button:", self.joystick.get_numbuttons(), "hat:", self.joystick.get_numhats(), "ball:", self.joystick.get_numballs())

        while running:  # Loop until "running" becomes false
            self.output0.setText(self.slider0.getValue())
            self.output1.setText(self.slider1.getValue())
            self.output2.setText(self.slider2.getValue())
            self.output3.setText(self.slider3.getValue())
            events = pygame.event.get()

            input_dict = ser.get_latest_message()
            if input_dict is not None:
                print(input_dict)

            for event in events:  # Get all of the events from the queue
                if event.type == pygame.JOYAXISMOTION:  # Main axis movement
                    # self.joyAxes[event.axis] = event.value
                    self.new_joystick_input = True
                    if print_debug: print("Axis0:",self.joystick.get_axis(0),"Axis1:",self.joystick.get_axis(1),"Axis2:",self.joystick.get_axis(2),"Axis3",self.joystick.get_axis(3))

                elif event.type == pygame.JOYBUTTONDOWN:  # Buttons pressed
                    self.joyButtons[event.button] = True
                    if print_debug: print("Button", event.button + 1, "pressed down")
                    if event.button == 0:
                        if print_debug:  print("Abort/Exit")
                        message_state_change["ChangeState"] = state_dictionary_reversed[1]
                        ser.send(json.dumps(message_state_change))

                elif event.type == pygame.JOYBUTTONUP:  # Buttons released
                    self.joyButtons[event.button] = False
                    if print_debug: print("Button", event.button + 1, "pressed up")

                elif event.type == pygame.JOYHATMOTION:
                    if print_debug: print("hat:", event.hat, "value:", event.value)

                elif event.type == pygame.KEYDOWN:
                    if print_debug: print("Button", event.key, "pressed down")

                    # if event.key == ord('0'):
                    #     print("Move to safe state")
                    # # elif (event.key-48) == 1:
                    # if event.key == ord('1'):
                    #     print("Move to panic state")
                    if event.key == 27 or event.key == ord('1'):
                        if print_debug:  print("Abort/Exit")
                        message_state_change["ChangeState"] = state_dictionary_reversed[1]
                        ser.send(json.dumps(message_state_change))

                    if event.key == ord('a'):
                        if print_debug: print("lift offset up")
                        keyboard_offsets["lift"] += keyboard_offsets_step["lift"]
                        self.new_joystick_input = True
                    if event.key == ord('z'):
                        if print_debug: print("lift offset down")
                        keyboard_offsets["lift"] -= keyboard_offsets_step["lift"]
                        self.new_joystick_input = True

                    if event.key == 1073741904:  # Left arrow key
                        if print_debug: print("roll offset up")
                        keyboard_offsets["roll"] += keyboard_offsets_step["roll"]
                        self.new_joystick_input = True
                    if event.key == 1073741903:  # Right arrow key
                        if print_debug: print("roll offset down")
                        keyboard_offsets["roll"] -= keyboard_offsets_step["roll"]
                        self.new_joystick_input = True

                    if event.key == 1073741905:  # Down arrow key
                        if print_debug: print("pitch offset up")
                        keyboard_offsets["pitch"] += keyboard_offsets_step["pitch"]
                        self.new_joystick_input = True
                    if event.key == 1073741906:  # Up arrow key
                        if print_debug: print("pitch offset down")
                        keyboard_offsets["pitch"] -= keyboard_offsets_step["pitch"]
                        self.new_joystick_input = True

                    if event.key == ord('w'):
                        if print_debug: print("yaw offset up")
                        keyboard_offsets["yaw"] += keyboard_offsets_step["yaw"]
                        self.new_joystick_input = True
                    if event.key == ord('q'):
                        if print_debug: print("yaw offset down")
                        keyboard_offsets["yaw"] -= keyboard_offsets_step["yaw"]
                        self.new_joystick_input = True

                    if event.key == ord('u'):
                        if print_debug: print("yaw control P offset up")
                        message_control_parameters["TunePID"]["yaw_P"] += keyboard_offsets_step["yaw_P"]
                        ser.send(json.dumps(message_control_parameters))
                    if event.key == ord('j'):
                        if print_debug: print("yaw control P offset down")
                        message_control_parameters["TunePID"]["yaw_P"] -= keyboard_offsets_step["yaw_P"]
                        ser.send(json.dumps(message_control_parameters))

                    if event.key == ord('i'):
                        if print_debug: print("roll/pitch P1 offset up")
                        message_control_parameters["TunePID"]["roll_pitch_P1"] += keyboard_offsets_step["roll_pitch_P1"]
                        ser.send(json.dumps(message_control_parameters))
                    if event.key == ord('k'):
                        if print_debug: print("roll/pitch P1 offset down")
                        message_control_parameters["TunePID"]["roll_pitch_P1"] -= keyboard_offsets_step["roll_pitch_P1"]
                        ser.send(json.dumps(message_control_parameters))

                    if event.key == ord('o'):
                        if print_debug: print("roll/pitch P2 offset up")
                        message_control_parameters["TunePID"]["roll_pitch_P2"] += keyboard_offsets_step["roll_pitch_P2"]
                        ser.send(json.dumps(message_control_parameters))
                    if event.key == ord('l'):
                        if print_debug: print("roll/pitch P2 offset down")
                        message_control_parameters["TunePID"]["roll_pitch_P2"] -= keyboard_offsets_step["roll_pitch_P2"]
                        ser.send(json.dumps(message_control_parameters))

                    if event.key == ord('y'):
                        if print_debug: print("M0 offset up")
                        message_individual_relative_control["MotorValueRel"]["motor"] = 0
                        message_individual_relative_control["MotorValueRel"]["value"] = 1
                        ser.send(json.dumps(message_individual_relative_control))
                    if event.key == ord('t'):
                        if print_debug: print("M0 offset down")
                        message_individual_relative_control["MotorValueRel"]["motor"] = 0
                        message_individual_relative_control["MotorValueRel"]["value"] = -1
                        ser.send(json.dumps(message_individual_relative_control))

                    if event.key == ord('h'):
                        if print_debug: print("M1 offset up")
                        message_individual_relative_control["MotorValueRel"]["motor"] = 1
                        message_individual_relative_control["MotorValueRel"]["value"] = 1
                        ser.send(json.dumps(message_individual_relative_control))
                    if event.key == ord('g'):
                        if print_debug: print("M1 offset down")
                        message_individual_relative_control["MotorValueRel"]["motor"] = 1
                        message_individual_relative_control["MotorValueRel"]["value"] = -1
                        ser.send(json.dumps(message_individual_relative_control))

                    if event.key == ord('b'):
                        if print_debug: print("M2 offset up")
                        message_individual_relative_control["MotorValueRel"]["motor"] = 2
                        message_individual_relative_control["MotorValueRel"]["value"] = 1
                        ser.send(json.dumps(message_individual_relative_control))
                    if event.key == ord('v'):
                        if print_debug: print("M2 offset down")
                        message_individual_relative_control["MotorValueRel"]["motor"] = 2
                        message_individual_relative_control["MotorValueRel"]["value"] = -1
                        ser.send(json.dumps(message_individual_relative_control))

                    if event.key == ord('f'):
                        if print_debug: print("M3 offset up")
                        message_individual_relative_control["MotorValueRel"]["motor"] = 3
                        message_individual_relative_control["MotorValueRel"]["value"] = 1
                        ser.send(json.dumps(message_individual_relative_control))
                    if event.key == ord('d'):
                        if print_debug: print("M3 offset down")
                        message_individual_relative_control["MotorValueRel"]["motor"] = 3
                        message_individual_relative_control["MotorValueRel"]["value"] = -1
                        ser.send(json.dumps(message_individual_relative_control))

                    if ord('0') <= event.key <= ord('9'):
                        if print_debug:
                            print("Change to state", state_dictionary_reversed[int(chr(event.key))])
                        message_state_change["ChangeState"] = state_dictionary_reversed[int(chr(event.key))]
                        ser.send(json.dumps(message_state_change))

                        if print_debug:
                            print("Change to state", state_dictionary_reversed[int(chr(event.key))])
                        if ((-1 * self.joystick.get_axis(0)) * pow(2, 19)) + keyboard_offsets["roll"] <= 20000 \
                                and ((self.joystick.get_axis(1)) * pow(2, 19)) + keyboard_offsets["pitch"] <= 20000 \
                                and ((self.joystick.get_axis(2)) * pow(2, 19)) + keyboard_offsets["yaw"] <= 2000 \
                                and ((-1 * self.joystick.get_axis(3) + 1) * pow(2, 19)) + keyboard_offsets["lift"] <= 50000:
                            message_state_change["ChangeState"] = state_dictionary_reversed[int(chr(event.key))]
                            print(json.dumps(message_state_change))

            self.screen.fill((0, 0, 0))
            self.output0.setText("M0: " + str(self.slider0.getValue()))
            self.output1.setText("M1: " + str(self.slider1.getValue()))
            self.output2.setText("M2: " + str(self.slider2.getValue()))
            self.output3.setText("M3: " + str(self.slider3.getValue()))
            pygame_widgets.update(events)
            pygame.display.update()

    # Send the data to the drone periodically based on joystick changes
    def send_data(self, ser):
        threading.Timer(1 / message_frequency, self.send_data, args=(ser, )).start()
        if self.new_joystick_input:
            message_joystick["TargetAttitude"]["roll"] = round((-1 * self.joystick.get_axis(0)) * pow(2,19)) + keyboard_offsets["roll"]
            message_joystick["TargetAttitude"]["pitch"] = round((self.joystick.get_axis(1)) * pow(2,19)) + keyboard_offsets["pitch"]
            message_joystick["TargetAttitude"]["yaw"] = round((self.joystick.get_axis(2)) * pow(2,19)) + keyboard_offsets["yaw"]
            message_joystick["TargetAttitude"]["lift"] = round((-1 * self.joystick.get_axis(3) + 1) * pow(2,19)) + keyboard_offsets["lift"]
            ser.send(json.dumps(message_joystick))
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
    # selected = input("Enter a joystick number or just Enter to exit:")
    selected = 0
    if selected == "": exit()
    # Convert the selection into an integer
    index = int(selected)
    # Initialize the selected joystick
    joystick = pygame.joystick.Joystick(index)
    joystick.init()

    pygame.init()
    screen = pygame.display.set_mode((1000, 1000))
    pygame.display.set_caption("Joystick tester")
    # # Initialize the display class
    window = JoystickHandler(screen, joystick)

    # Start the main loop
    window.send_data(ser)
    window.run(ser)
