from subprocess import Popen, PIPE
import numpy as np
import sys

process = Popen(["cargo", "build", "--release"], stdout=PIPE)
(output, err) = process.communicate()
exit_code = process.wait()

t_limit = 10
repeats = 10
bits = [ 128 * i for i in range(1, 128)]
data = {}

mode = 'key'

data[mode] = {}

stop = False

for bit in bits:
    data[mode][bit] = []

    t = 0
    t_median = 0

    for r in range(0, repeats):
        process = Popen(["../target/release/rsa_lixo", "--keysize", str(bit), "--key", "key", "--generate_key"], stdout=PIPE)
        (output, err) = process.communicate()
        exit_code = process.wait()

        t = float(output.strip())

        if t > t_limit and not stop:
            stop = True
            print("Stoping here")

            if bit in data[mode][bit]:
                data[mode].pop(bit,None)

            break
        else:
            data[mode][bit].append(t)

    print("%20s %5d %6.3f %6.3f" % (mode, bit, np.mean(data[mode][bit]), np.median(data[mode][bit])))
    sys.stdout.flush()

    if stop:
        break


for bit in bits:
    if bit in data[mode]:
        print("%20s %4d %6.3f %6.3f %s" % (mode, bit, np.mean(data[mode][bit]), np.median(data[mode][bit]), data[mode][bit]))
