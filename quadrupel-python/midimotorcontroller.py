import mpd24midi
from tkinter import *


def mainloop_fun():
    global cur_vals, authority

    new_vals = controller.getSliders()

    for i in range(4):
        if new_vals[i] != cur_vals[i]:
            if authority[i]:
                print(f"update motor {i} with speed {new_vals[i]}")
                [w1, w2, w3, w4][i].set(new_vals[i])
                cur_vals[i] = new_vals[i]
            else:
                if abs(cur_vals[i] - new_vals[i]) < 10:
                    authority[i]=1
                    print(f"update motor {i} with speed {new_vals[i]}")
                    [w1, w2, w3, w4][i].set(new_vals[i])
                    cur_vals[i] = new_vals[i]
                    [w1, w2, w3, w4][i].config(bg="green")




    master.after(10, mainloop_fun)


def show_values():
    print("Sending:", w1.get(), w2.get(), w3.get(), w4.get())


def slider_control(n):
    v = [w1.get(), w2.get(), w3.get(), w4.get()][n]
    global cur_vals, authority
    print(f"update motor {n} with speed {v}")
    authority[n] = 0
    cur_vals[n] = v
    [w1,w2,w3,w4][n].config(bg="red")

def reset():
    global cur_vals, authority
    for x in range(4):
        [w1, w2, w3, w4][x].set(0)
        [w1, w2, w3, w4][x].config(bg="red")
        print(f"update motor {x} with speed 0")
    cur_vals = [0,0,0,0]
    authority = [0,0,0,0]




if __name__ == '__main__':
    authority = [1, 1, 1, 1]

    controller = mpd24midi.Mpd24()

    cur_vals = [0, 0, 0, 0, 0, 0]

    print("Init")

    while not all(x == 1000 for x in controller.getSliders()[:4]):
        pass

    while not all(x == 0 for x in controller.getSliders()[:4]):
        pass

    print("Armed!")

    master = Tk()
    w1 = Scale(master, from_=1000, to=0)
    w1.bind("<ButtonRelease-1>", lambda x: slider_control(0))
    w1.set(0)
    w1.grid(column=0, row=0)
    w2 = Scale(master, from_=1000, to=0)
    w2.bind("<ButtonRelease-1>", lambda x: slider_control(1))
    w2.set(0)
    w2.grid(column=1, row=0)
    w3 = Scale(master, from_=1000, to=0)
    w3.bind("<ButtonRelease-1>", lambda x: slider_control(2))
    w3.set(0)
    w3.grid(column=2, row=0)
    w4 = Scale(master, from_=1000, to=0)
    w4.bind("<ButtonRelease-1>", lambda x: slider_control(3))
    w4.set(0)
    w4.grid(column=3, row=0)
    Button(master, text='reset', command=reset).grid(column=1,row=1,columnspan=2)

    for x in [w1,w2,w3,w4]:
        x.config(bg="green")

    master.after(10,mainloop_fun)

    mainloop()
