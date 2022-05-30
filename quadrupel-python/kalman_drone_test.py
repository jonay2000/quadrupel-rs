from math import sin, pi
from scipy import signal
import matplotlib.pyplot as plt
from numpy.random import normal


class Drone:
    def __init__(self, gyro_err=0.1, acc_err=0.05):
        self.rot = 0
        self.rot_spd = 0
        self.gyro_err = gyro_err
        self.acc_err = acc_err

    def measure(self):
        return normal(self.rot_spd,self.gyro_err),normal(self.rot,self.acc_err)

    def sim(self,dt):
        self.rot+=self.rot_spd*dt
        self.rot = (self.rot+pi)%(2*pi)-pi

class Kalman:
    def __init__(self):
        pass

def round_dist(state, goal):

    neutral = abs(goal - state)
    left = abs(goal - state + 2 * pi)
    right = abs(goal - state - 2 * pi)

    if neutral < left and neutral < right :
        return goal - state
    elif left < right :
        return goal - state + 2 * pi
    else:
        return goal - state - 2 * pi


class Compl:
    def __init__(self, c1=500, c2=10000):
        self.c1 = c1
        self.c2 = c2

        self.b = 0
        self.phi = 0


    def filter(self, sp, sphi, dt):
        p = sp-self.b
        self.phi = self.phi + p*dt
        e = round_dist(sphi,self.phi)
        self.phi = self.phi - e/self.c1
        self.phi = (self.phi+pi)%(2*pi)-pi
        self.b = self.b+(e/dt)/self.c2
        return p, self.phi



drone = Drone()
kal = Compl()

# basic_integrator = 0
dt= 1/1260
t = 0

with open("data_drone.txt") as f:
    data = f.readlines()

data = [eval(x) for x in data]
data = [[x[0]/2**16,x[1]/2**16] for x in data]


act_p = []
act_phi = []
measured_p = []
measured_phi = []
k_p = []
k_phi = []

for d in data[:]:
    t+=dt
    sp,sphi = d

    measured_p.append(sp)
    measured_phi.append(sphi)

    kp, kphi = kal.filter(sp, sphi, dt)

    k_phi.append(kphi)

butterworth_order = 2
cutoff_freq = 5
sampling_freq = 1260

sos = signal.butter(butterworth_order, cutoff_freq, 'low', fs=sampling_freq, output='sos')
filtered = signal.sosfilt(sos, k_phi)

# plt.plot(measured_p,label="mes_p")
# plt.plot(measured_phi,label="mes_phi")
plt.plot(k_phi, label="k_phi", color='r')
plt.plot(filtered, label="filtered", color='g')
plt.legend()
plt.show()

# for _ in range(5):
#
#     act_p = []
#     act_phi = []
#     measured_p = []
#     measured_phi = []
#     k_p = []
#     k_phi = []
#     basic_integrator_plt = []
#
#
#
#     for _ in range(100):
#         t+=dt
#         drone.rot_spd = 0.1
#         drone.sim(dt)
#         act_p.append(drone.rot_spd)
#         act_phi.append(drone.rot)
#         sp, sphi = drone.measure()
#         kp, kphi = kal.filter(sp,sphi,dt)
#         k_p.append(kp)
#         k_phi.append(kphi)
#         basic_integrator+=sp*dt
#         basic_integrator_plt.append(basic_integrator)
#         measured_p.append(sp)
#         measured_phi.append(sphi)
#
#     # plt.plot(act_p,label="act_p")
#     plt.plot(act_phi,label="act_phi")
#     # plt.plot(measured_p,label="mes_p")
#     plt.plot(measured_phi,label="mes_phi")
#     # plt.plot(basic_integrator_plt,label="basic_int")
#
#     # plt.plot(k_p, label="k_p")
#     plt.plot(k_phi, label="k_phi")
#
#     plt.legend()
#
#     plt.show()
#
#     for _ in range(20000):
#         t+=dt
#         drone.rot_spd = sin(t/10)
#         drone.sim(dt)
#         sp, sphi = drone.measure()
#         kp, kphi = kal.filter(sp, sphi, dt)
#         basic_integrator+=sp*dt
#

# import matplotlib.pyplot as plt
# with open('flash_data.txt', 'r') as f:
#     lines = f.readlines()
#     x1 = [float(line.split(", ")[0]) for line in lines]
#     x2 = [float(line.split(", ")[1]) for line in lines]
#     x3 = [float(line.split(", ")[2]) for line in lines]
#
# # with open('output.txt', 'r') as f:
# #     lines = f.readlines()
# #     y = [float(line.split()[0]) for line in lines]
# plt.plot(x1)
# plt.plot(x2)
# plt.plot(x3)
# # plt.plot(y)
# plt.show()