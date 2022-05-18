from math import cos, sin, pi

import numpy as np
import pygame

Rz = lambda a: np.array([[cos(a), -sin(a), 0], [sin(a), cos(a), 0], [0, 0, 1]])  # yaw
Ry = lambda b: np.array([[cos(b), 0, sin(b)], [0, 1, 0], [-sin(b), 0, cos(b)]])  # pitch
Rx = lambda y: np.array([[1, 0, 0], [0, cos(y), -sin(y)], [0, sin(y), cos(y)]])  # roll
# Rt = lambda a,b,y: np.matmul(np.matmul(Rz(a),Ry(b)),Rx(y))
Rt = lambda a, b, y: np.matmul(np.matmul(Ry(-a), Rz(b)), Rx(y))


class Drone:
    def __init__(self, screen, screen_size, off):
        self.pos = np.array([0, 0, -100])
        self.rot = [0, 0, 0]
        self.plot = []
        self.screen_size = screen_size
        self.off = off
        self.screen = screen

        self.clickx = None
        self.clicky = None
        self.startx = None
        self.starty = None

    def inbounds(self):
        mousex, mousey = pygame.mouse.get_pos()
        return (
                self.off[0] < mousex < self.off[0] + self.screen_size[0] and
                self.off[1] < mousey < self.off[1] + self.screen_size[1]
        )

    def draw(self):
        if pygame.mouse.get_pressed()[0] and self.clickx is None and self.clicky is None and self.inbounds():
            self.clickx, self.clicky = pygame.mouse.get_pos()
            self.startx = self.pos[0]
            self.starty = self.pos[1]
        elif not pygame.mouse.get_pressed()[0] or not self.inbounds():
            self.clickx, self.clicky = None, None

        if self.clickx is not None and self.clicky is not None:
            mousex, mousey = pygame.mouse.get_pos()
            dx = self.clickx - mousex
            dy = self.clicky - mousey

            self.pos[0] = self.startx + dx / 6
            self.pos[1] = self.starty + dy / 6

        for x in self.plot:
            x.remove()
        self.plot = []

        arm_length = 20
        # arms = [np.array([0, 1, 0]),np.array([1, 0, 0]),np.array([0, -1, 0]),np.array([-1, 0, 0]),np.array([0, 0, 0.4])]
        arms = [np.array([1, 0, 0]), np.array([0, 0, 1]), np.array([-1, 0, 0]), np.array([0, 0, -1]),
                np.array([0, 0.3, 0])]
        arms = map(lambda x: x * arm_length, arms)
        arms = list(map(lambda x: np.matmul(Rt(*self.rot), x) + self.pos, arms))

        # arm1 = np.array([-1, 0, 0]) * arm_length
        # arm1 = np.matmul(Rt(*self.rot),arm1)+self.pos

        # print(arm1)
        for ind, x in enumerate(arms):
            col = [(255, 0, 200), (255, 0, 0), (255, 0, 0), (255, 0, 0), (0, 255, 0)][ind]
            col = self.shade(col, x, self.pos, 0.03)
            # print(col)
            self.plot_line(self.pos, x, col)
            # self.plot.append(plt.plot(*zip(self.pos,x),color='blue')[0])
        # self.plot.append(ax.scatter(*arms[0],marker="o",color='red'))
        # print(arms[0])

    def convert_to_2d(self, point):
        return [point[0] * (point[2] / 30) + self.screen_size[0] / 2,
                point[1] * (point[2] / 30) + self.screen_size[1] / 2]

    def plot_line(self, x, y, col=(0, 0, 255)):
        pos = self.rescale(self.convert_to_2d(x), self.screen_size,
                           tuple(map(lambda i: i / 2, self.off))), self.rescale(self.convert_to_2d(y),
                                                                                self.screen_size,
                                                                                tuple(map(lambda i: i / 2, self.off)))
        pygame.draw.line(self.screen, col, *pos)

    def shade(self, col, pos, base_pos, r):
        v = 1 + (base_pos[2] - pos[2]) * r
        col = map(lambda x: min(255, max(0, (x - 20) * v)), col)
        return list(col)

    def rescale(self, pos, screen_width, off):
        return pos[0] / 640 * screen_width[0] + off[0], pos[1] / 480 * screen_width[1] + off[1]


if __name__ == '__main__':
    screen = pygame.display.set_mode((640, 480))
    running = 1

    drone = Drone(screen, (640, 480), [0, 0])

    while running:
        event = pygame.event.poll()
        if event.type == pygame.QUIT:
            running = 0

        screen.fill((0, 0, 0))

        drone.draw()

        drone.rot[0] += 0.01
        # drone.rot[2] += 0.003

        # pygame.draw.line(screen, (0, 0, 255), (0, 0), (639, 479))
        # pygame.draw.aaline(screen, (0, 0, 255), (639, 0), (0, 479))
        pygame.display.flip()
