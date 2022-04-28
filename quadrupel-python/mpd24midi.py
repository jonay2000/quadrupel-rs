from pygame import midi

midi.init()


class Mpd24:
    def __init__(self):
        self.sliders = [0 for _ in range(6)]
        print(midi.get_default_input_id())
        self.i = midi.Input(midi.get_default_input_id())

    def getSliders(self):

        while (self.i.poll()):

            r = self.i.read(1)[0][0]
            if r[0] == 177:
                try:
                    self.sliders[r[1] - 1] = r[2]
                except:
                    print("plz stop touching other buttons!!!")

        return [x*1000//127 for x in self.sliders]


def init():
    global controller
    controller = Mpd24()
