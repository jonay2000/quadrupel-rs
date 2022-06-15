import random
from math import sin, pi
from scipy import signal
import matplotlib.pyplot as plt
from numpy.random import normal

# from scipy import pi
from scipy.fftpack import fft
from numpy.fft import fft, ifft
import numpy as np
from scipy.optimize import fmin


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
    def __init__(self,q_angle=0.001,q_bias=0.003,r_measure=0.03):
        self.r_measure = r_measure
        self.q_bias = q_bias
        self.q_angle = q_angle

        self.angle = 0
        self.bias = 0

        self.p = [[0,0],[0,0]]

    def filter(self, sp, sphi, dt):
        rate = sp - self.bias
        self.angle += dt*rate

        self.p[0][0] += dt * (dt*self.p[1][1]-self.p[0][1]- self.p[1][0] + self.q_angle)
        self.p[0][1] -= dt*self.p[1][1]
        self.p[1][0] -= dt*self.p[1][1]
        self.p[1][1] += self.q_bias*dt

        s = self.p[0][0] +self.r_measure
        k = (self.p[0][0]/s,self.p[1][0]/s)

        # y = round_dist(self.angle,sphi)
        y = sphi-self.angle

        self.angle += k[0]*y
        self.bias += k[1]*y

        p00 = self.p[0][0]
        p01 = self.p[0][1]

        self.p[0][0] -= k[0] * p00
        self.p[0][1] -= k[0] * p01
        self.p[1][0] -= k[1] * p00
        self.p[1][1] -= k[1] * p01

        return rate, self.angle

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
    def __init__(self, c1=100, c2=1000000):
        self.c1 = c1
        self.c2 = c2

        self.b = 0
        self.phi = 0


    def filter(self, sp, sphi, dt):
        # print(sp)
        # self.phi = (1-self.c1)*(self.phi+sp) + self.c1*sphi
        #
        # return sp, self.phi

        p = sp-self.b
        self.phi = self.phi + p*dt
        e = round_dist(sphi,self.phi)
        self.phi = self.phi - e/self.c1
        self.phi = (self.phi+pi)%(2*pi)-pi
        self.b = self.b+(e/dt)/self.c2
        return p, self.phi



drone = Drone()
# kal = Kalman(0.001,0.002,0.003)
#
# for _ in range(1000):
#     v = kal.filter(1,0.2,0.001)
#     print(v)
#     v = kal.filter(-1,-0.2,0.001)
#     print(v)







print("AAAA")

# basic_integrator = 0
dt= 1/420
t = 0

with open("flash_data (5).txt") as f:
    data = f.readlines()

# data = data[:5700]+data

data = [eval(x) for x in data]
# data = [[x[0]/2**16,x[1]/2**16+random.random(),x[2]/2**16,0] for x in data]
data = [[x[0]/2**16,-x[1]/2**16,x[2]/2**16,0] for x in data]


shift = 5
for x in range(len(data)-shift):
    data[x][3] = sum(data[x+i][2] for i in range(shift))/shift
shift = 3
for x in range(len(data)-shift):
    data[x][3] = data[x+shift][3]

# data = data[:-50]
data = data[200:10000]
# [ 0.0398288  -0.00017332  0.00222042]

# angles = [x[0] for x in data]
# myfilt = signal.butter(2, 100, 'low', fs=500, output='sos')
# filtered = signal.sosfilt(myfilt, angles)
# for x in range(len(angles)):
#     data[x][0]=filtered[x]


def funct(vars):
    # kal = Compl(vars[0],vars[1])
    kal = Kalman(*vars[:-1])
    qq = vars[-1]

    global act_p,act_phi,measured_p,measured_phi,k_p,k_phi,dmp

    act_p = []
    act_phi = []
    measured_p = []
    measured_phi = []
    k_p = []
    k_phi = []
    dmp = []

    t = 0

    for d in data[:]:
        t+=dt
        sphi,sp,old_dmp,dmp_item = d

        measured_p.append(sp)
        measured_phi.append(sphi)

        kp, kphi = kal.filter(sp*qq, sphi, dt)

        k_phi.append(kphi)
        dmp.append(dmp_item)

    return sum(abs(x[0]-x[1])**2 for x in zip(k_phi,dmp))

# sos = signal.butter(2, 5, 'low', fs=856, output='sos')
# filtered = signal.sosfilt(sos, k_phi)

#[ 0.00321373, -0.000167,    0.00483083]
# mini = fmin(funct,(10,50000))
mini = fmin(funct,(0.01, 0.003, 0.03,16/360*2*3.1415))
# mini = [ 3.32858877e-05, 5.51620221e-03, 6.48176954e-05]
# mini = [ 0.01, 0.003, 0.03]
print(mini)

print("err",funct(mini))
# funct((35.2,19945))

sos = signal.butter(2, 15, 'low', fs=856, output='sos')
filtered = signal.sosfilt(sos, k_phi)

print(len(k_phi)/856)
plt.plot([x*10 for x in measured_p],label="mes_p")
# plt.plot(measured_phi,label="mes_phi")
plt.plot(k_phi, label="k_phi", color='r')
# plt.plot(filtered, label="filtered", color='b')
plt.plot(dmp, label='dmp', color='g')
plt.legend()
plt.show()

# sample_rate = 856
# duration = len(k_phi)/856
# N = (duration - 0) * sample_rate
# time = np.linspace(0, round(duration), round(N))
#
# frequency = np.linspace (0.0, 512, int (N/2))
# freq_data = fft(k_phi)
# y = 2/N * np.abs (freq_data [0:np.int (N/2)])
#
# plt.plot(frequency, y)
# plt.title('Frequency domain Signal')
# plt.xlabel('Frequency in Hz')
# plt.ylabel('Amplitude')
# plt.show()

X = fft(k_phi)
N = len(X)
sr = 856
n = np.arange(N)
T = N/sr
freq = n/T
t = np.arange(0,len(k_phi)/856,1/sr)

plt.figure(figsize = (12, 6))
plt.subplot(121)

plt.stem(freq, np.abs(X), 'b', \
         markerfmt=" ", basefmt="-b")
plt.xlabel('Freq (Hz)')
plt.ylabel('FFT Amplitude |X(freq)|')
plt.xlim(0, 20)

# plt.subplot(122)
# plt.plot(t, ifft(X), 'r')
# plt.xlabel('Time (s)')
# plt.ylabel('Amplitude')
# plt.tight_layout()
# plt.show()