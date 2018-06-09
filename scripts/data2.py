import matplotlib.pyplot as plt

x = [ 128, 256, 384, 512, 640, 768, 896, 1024, 1152, 1280, 1408, 1536, 1664, 1792, 1920, 2048, 2176, 2304, 2432, 2560 ]
median = [0.333, 0.180, 0.273, 0.534, 0.584, 0.196, 0.204, 0.263, 0.287, 0.663, 0.466, 0.581, 0.868, 1.085, 1.377, 1.683, 1.594, 2.084, 2.011, 2.848]
mean = [ 0.373, 0.366, 0.284, 0.483, 0.564, 0.330, 0.253, 0.319, 0.399, 0.783, 0.802, 0.703, 0.904, 1.414, 2.131, 1.567, 1.796, 2.182, 2.258, 2.847]


plt.plot(x, median, label='Quebra de chave')

#plt.legend()

plt.xlabel("N of Bits")
plt.ylabel("Time (S)")

plt.grid(True)

plt.show()