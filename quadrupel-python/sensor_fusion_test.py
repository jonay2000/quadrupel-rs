from math import sin, pi
from scipy import signal
import matplotlib.pyplot as plt
from numpy.random import normal

# from scipy import pi
from scipy.fftpack import fft
from numpy.fft import fft, ifft
import numpy as np

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
    def __init__(self, c1=100, c2=10000):
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
dt= 1/856
t = 0

with open("flash_data.txt") as f:
    data = f.readlines()

data = [eval(x) for x in data]
data = [[x[0]/2**16,x[1]/2**16,x[2]/2**16] for x in data]


act_p = []
act_phi = []
measured_p = []
measured_phi = []
k_p = []
k_phi = []
dmp = []

for d in data[:]:
    t+=dt
    sphi,sp,dmp_item = d

    measured_p.append(sp)
    measured_phi.append(sphi)

    kp, kphi = kal.filter(sp, sphi, dt)

    k_phi.append(kphi)
    dmp.append(dmp_item)

sos = signal.butter(2, 5, 'low', fs=856, output='sos')
filtered = signal.sosfilt(sos, k_phi)

print(len(k_phi)/856)
# plt.plot(measured_p,label="mes_p")
# plt.plot(measured_phi,label="mes_phi")
plt.plot(k_phi, label="k_phi", color='r')
plt.plot(filtered, label="filtered", color='b')
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

plt.subplot(122)
plt.plot(t, ifft(X), 'r')
plt.xlabel('Time (s)')
plt.ylabel('Amplitude')
plt.tight_layout()
plt.show()