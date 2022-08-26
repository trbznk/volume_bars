import numpy as np

HEADER = 'open,high,low,close,volume'
HEADER_LEN = 26
BAR_SIZE = 5
VOLUME_COLUMN_INDEX = 4

def read_ohlcv(path):
    with open(path) as f:
        line = f.readline(HEADER_LEN)
        assert line == HEADER, "HEADER of csv file must be 'open,high,low,close,volume'"
    
    a = np.genfromtxt(path, delimiter=",", skip_header=1)
    return a


def init_new_bar():
    return np.array([np.nan, np.nan, np.nan, np.nan, 0])


bars = read_ohlcv("bars.csv")
bars[bars[:, VOLUME_COLUMN_INDEX] > BAR_SIZE, VOLUME_COLUMN_INDEX] = BAR_SIZE
print(bars)
print(bars[:, 4].sum())
print("-"*80)
volume_bars = np.zeros((0, 5))

new_bar = init_new_bar()
v_used = 0

i = 0
while i < bars.shape[0]:
    if new_bar[4] >= BAR_SIZE:
        new_bar[3] = bars[i][3]
        v_used -= new_bar[4]-BAR_SIZE
        new_bar[4] = BAR_SIZE
        volume_bars = np.append(volume_bars, new_bar.reshape(1, 5), axis=0)
        new_bar = init_new_bar()

    if v_used == bars[i][4]:
        if i == bars.shape[0]-1:
            if new_bar[4] > 0:
                new_bar[3] = bars[i][3]
                volume_bars = np.append(volume_bars, new_bar.reshape(1, 5), axis=0)
        i += 1
        v_used = 0
    else:
        new_bar[0] = bars[i][0] if np.isnan(new_bar[0]) else new_bar[0]
        new_bar[1] = bars[i][1] if bars[i][1] > new_bar[1] or np.isnan(new_bar[1]) else new_bar[1]
        new_bar[2] = bars[i][2] if bars[i][2] < new_bar[2] or np.isnan(new_bar[2]) else new_bar[2]
        new_bar[4] += bars[i][4]-v_used
        v_used += bars[i][4]-v_used



    
print(volume_bars)



