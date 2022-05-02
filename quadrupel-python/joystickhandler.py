import pygame
import json
import sys
import threading

# Roll: Axis 0
# Pitch: Axis 1
# Yaw: Axis 2
# Height: Axis 3

# Do we care about both press up and down or just down?
# Are we sure on all allowed transitions per state?
# Are we addidng the trimming on the computer side?

message_frequency = 0.001;  # In hertz

state_dictionary = {
    "Safe": 0,
    "Panic": 1,
    "manual": 2,
    "Calibration": 3,
    "yaw_control": 4,
    "FullControl": 5,
    "raw": 6,
    "height_control": 7,
    "wireless": 8
}

state_dictionary_reversed = {
    0: "Safe",
    1: "Panic",
    2: "manual",
    3: "Calibration",
    4: "yaw_control",
    5: "FullControl",
    6: "raw",
    7: "height_control",
    8: "wireless"
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

# TODO: Tune the offset step
# Increase/Decrease to be applied to the keyboard offset per key press
keyboard_offsets_step = {
    "lift": 1,
    "roll": 1,
    "pitch": 1,
    "yaw": 1,
    "yaw_P": 1,
    "roll_pitch_P1": 1,
    "roll_pitch_P2": 1
}

# TODO: Populate the dictionary
# Key is allowed to go to the states in the value array
allowed_state_transition = {
    "Safe": [
        state_dictionary["Safe"],
        state_dictionary["Panic"],
        state_dictionary["manual"],
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


class JoystickHandler:
    def __init__(self, screen, joystick):
        # Setup class variables
        self.screen = screen
        self.width = screen.get_width()
        self.height = screen.get_height()
        self.joystick = joystick

        # Get the number of each type of input from the joystick
        self.axisCount = joystick.get_numaxes()
        self.buttonCount = joystick.get_numbuttons()
        self.ballCount = joystick.get_numballs()
        self.hatCount = joystick.get_numhats()

        # Initialize arrays to keep track of the axis
        self.joyAxes = dict()
        for i in range(self.axisCount):
            self.joyAxes[i] = 0
        # self.plotAxes()
        self.joyButtons = dict()
        for i in range(self.buttonCount):
            self.joyButtons[i] = False

        self.current_state = state_dictionary["Safe"]

        # self.current_state = {
        #    "roll": (-1 * self.joystick.get_axis(0) + 1) * 50,
        #    "pitch": (self.joystick.get_axis(1) + 1) * 50,
        #    "yaw": (self.joystick.get_axis(2) + 1) * 50,
        #    "lift": (-1 * self.joystick.get_axis(3) + 1) * 50
        # }

    def run(self):
        running = True  # This is the main "loop running" variable -- set to false to exit the loop
        # print("axis:", self.axisCount, "button:", self.buttonCount, "hat:", self.hatCount, "ball:", self.ballCount)
        while running:  # Loop until "running" becomes false
            for event in pygame.event.get():  # Get all of the events from the queue
                if event.type == pygame.JOYAXISMOTION:  # Main axis movement
                    self.joyAxes[event.axis] = event.value
                    # print("Axis0:",self.joystick.get_axis(0),"Axis1:",self.joystick.get_axis(1),"Axis2:",self.joystick.get_axis(2),"Axis3",self.joystick.get_axis(3))

                elif event.type == pygame.JOYBUTTONDOWN:  # Buttons pressed
                    self.joyButtons[event.button] = True
                    print("Button", event.button + 1, "pressed down")
                    if event.button == 0:
                        print("Abort/Exit")

                elif event.type == pygame.JOYBUTTONUP:  # Buttons released
                    self.joyButtons[event.button] = False
                    print("Button", event.button + 1, "pressed up")

                elif event.type == pygame.JOYHATMOTION:
                    print("hat:", event.hat, "value:", event.value)

                elif event.type == pygame.KEYDOWN:
                    print("Button", event.key, "pressed down")

                    # if event.key == ord('0'):
                    #     print("Move to safe state")
                    # # elif (event.key-48) == 1:
                    # if event.key == ord('1'):
                    #     print("Move to panic state")
                    if event.key == 27:
                        print("Abort/Exit")

                    if event.key == ord('a'):
                        print("lift offset up")
                        keyboard_offsets["lift"] += keyboard_offsets_step["lift"]
                    if event.key == ord('z'):
                        print("lift offset down")
                        keyboard_offsets["lift"] -= keyboard_offsets_step["lift"]

                    if event.key == 1073741904:  # Left arrow key
                        print("roll offset up")
                        keyboard_offsets["roll"] += keyboard_offsets_step["roll"]
                    if event.key == 1073741903:  # Right arrow key
                        print("roll offset down")
                        keyboard_offsets["roll"] -= keyboard_offsets_step["roll"]

                    if event.key == 1073741905:  # Down arrow key
                        print("pitch offset up")
                        keyboard_offsets["pitch"] += keyboard_offsets_step["pitch"]
                    if event.key == 1073741906:  # Up arrow key
                        print("pitch offset down")
                        keyboard_offsets["pitch"] -= keyboard_offsets_step["pitch"]

                    if event.key == ord('w'):
                        print("yaw offset up")
                        keyboard_offsets["yaw"] += keyboard_offsets_step["yaw"]
                    if event.key == ord('q'):
                        print("ywa offset down")
                        keyboard_offsets["yaw"] -= keyboard_offsets_step["yaw"]

                    if event.key == ord('u'):
                        print("yaw control P offset up")
                        keyboard_offsets["yaw_P"] += keyboard_offsets_step["yaw_P"]
                    if event.key == ord('j'):
                        print("yaw control P offset down")
                        keyboard_offsets["yaw_P"] -= keyboard_offsets_step["yaw_P"]

                    if event.key == ord('i'):
                        print("roll/pitch P1 offset up")
                        keyboard_offsets["roll_pitch_P1"] += keyboard_offsets_step["roll_pitch_P1"]
                    if event.key == ord('k'):
                        print("roll/pitch P1 offset down")
                        keyboard_offsets["roll_pitch_P1"] -= keyboard_offsets_step["roll_pitch_P1"]

                    if event.key == ord('o'):
                        print("roll/pitch P2 offset up")
                        keyboard_offsets["roll_pitch_P2"] += keyboard_offsets_step["roll_pitch_P2"]
                    if event.key == ord('l'):
                        print("roll/pitch P2 offset down")
                        keyboard_offsets["roll_pitch_P2"] -= keyboard_offsets_step["roll_pitch_P2"]

                    if ord('0') <= event.key <= ord('9'):
                        if int(chr(event.key)) in allowed_state_transition[state_dictionary_reversed[self.current_state]]:
                            print("Move to state", state_dictionary_reversed[int(chr(event.key))])
                            self.current_state = int(chr(event.key))

    # TODO: set the message to be sent per state
    # Send the data to the drone based on the current state
    def send_data(self):
        threading.Timer(1 / message_frequency, self.send_data).start()
        print("Roll:", (-1 * self.joystick.get_axis(0) + 1) * 50, "Pitch:", (self.joystick.get_axis(1) + 1) * 50,
              "Yaw:",
              (self.joystick.get_axis(2) + 1) * 50, "Lift:", (-1 * self.joystick.get_axis(3) + 1) * 50)
        print(keyboard_offsets)
        print(self.current_state)

# Setup the joysticks
pygame.joystick.init()
stickCount = pygame.joystick.get_count()  # How many joysticks are connected?
for index in range(stickCount):  # Print the name of each joystick
    joystick = pygame.joystick.Joystick(index)
    print("{0}) {1}".format(index, joystick.get_name()))
# Get the user's selection, and exit if they just press enter
selected = input("Enter a joystick number or just Enter to exit:")
if selected == "": sys.exit
# Convert the selection into an integer
index = int(selected)
# Initialize the selected joystick
joystick = pygame.joystick.Joystick(index)
joystick.init()
# Initialize the pygame display
#
pygame.display.init()
screen = pygame.display.set_mode((800, 600))
pygame.display.set_caption("Joystick tester")
# # Initialize the display class
window = JoystickHandler(screen, joystick)
# Start the main loop
window.send_data()
window.run()
