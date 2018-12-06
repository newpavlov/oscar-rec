#!/usr/bin/env python3
import numpy as np
import matplotlib.pyplot as plt

import fileinput

DATA_LEN = 100

plt.ion()

fig = plt.figure()
plt.grid()
ax = fig.add_subplot(111)

lines = None
data = []

k = 0

for line in fileinput.input():
    d = [float(i) for i in line.replace('\n', '').split('\t')]
    if k != 40:
        k += 1
        continue
    else:
        k = 0

    if lines is None:
        lines = []
        data = []
        for i in range(len(d)):
            data.append(np.zeros(DATA_LEN))
            lines.append(ax.plot(data[i])[0])
    for i in range(len(d)):
        data[i] = np.roll(data[i], 1)
        data[i][0] = d[i]
        lines[i].set_ydata(data[i])
    ax.relim()
    ax.autoscale_view()
    plt.pause(0.0001)
