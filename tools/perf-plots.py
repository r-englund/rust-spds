import matplotlib.pyplot as plt
from matplotlib.colors import ListedColormap
from matplotlib import cm
import numpy as np
import math



def set_tics(p,x,y):
    p.set_xticks(np.arange(len(x)))
    p.set_yticks(np.arange(len(y)))
    p.set_xticklabels(x)
    p.set_yticklabels(y)
    plt.setp(p.get_xticklabels(), rotation=-45, ha="right",va="center",
        rotation_mode="anchor")


def get_data():
    return np.genfromtxt('../perf1.csv', delimiter=';',names=True, autostrip=True , dtype=[
        ('num_points', 'uint'),
        ('find_amount', 'uint'),
        ('find_frac', 'f8'),
        ('method', 'S19'),
        ('duration', 'u8'),
    ])
# num_points ; find_amount ; find_frac ; method ; duration


def plot():


    plt1.clear()
    plt2.clear()
    plt3.clear()
    plt4.clear()

    lines = {}

    uniq_num_points = np.unique(data["num_points"])
    uniq_methods = np.unique(data["method"])
    uniq_find_frac = np.unique(data["find_frac"])


    heatmap = np.zeros((len(uniq_num_points),len(uniq_find_frac)))
    heatmap_w_sort = np.zeros((len(uniq_num_points),len(uniq_find_frac)))
    heatmap_cmp = np.zeros((len(uniq_num_points),len(uniq_find_frac)))



    for num_points , find_amount , find_frac , method , duration in data:
        key = hash((
            method,num_points
        ))
        key = "{} {}".format(method, num_points)
        if key not in lines:
            lines[key] = {
                'x' : [],
                'y' : [],
            }
        
        lines[key]['x'].append(find_frac)
        lines[key]['y'].append(math.log(duration))

        index_p = np.where(uniq_num_points == num_points)[0]
        index_f = np.where(uniq_find_frac == find_frac)[0]
        #d = duration
        d = math.log(duration)
        #d = duration / num_points
        if "sort" in str(method):
            heatmap_w_sort[index_p,index_f] = d#math.log(duration)
        else:
            heatmap[index_p,index_f] = d# math.log(duration)
        

    for name, line in lines.items():
        plt1.plot(line["x"], line["y"], label = name)

    neg = cm.get_cmap('winter', 256)
    pos = cm.get_cmap('autumn', 256)
    better_cm = np.zeros((513,4))
    better_cm[:256,:] = neg(np.linspace(0,1,256))
    better_cm[257:,:] = pos(np.linspace(0,1,256))
    better_cm[256,:] = [0,0,0,1]

    heatmap_cmp = heatmap - heatmap_w_sort
    range_max = max(np.max(heatmap_cmp), abs(np.min(heatmap_cmp)))


    im = plt2.matshow(heatmap_cmp, cmap=ListedColormap(better_cm),vmin=-range_max,vmax=range_max)
    plt2.set_title("Difference (red-yellow: with sort)")

    joint_min = min(np.min(heatmap), np.min(heatmap_w_sort))
    joint_max = min(np.max(heatmap), np.max(heatmap_w_sort))


    plt3.set_title("No sort")
    plt3.matshow(heatmap,vmin=joint_min,vmax=joint_max)
    plt4.matshow(heatmap_w_sort,vmin=joint_min,vmax=joint_max)
    plt4.set_title("With sort")

    set_tics(plt2,uniq_find_frac,uniq_num_points)
    set_tics(plt3,uniq_find_frac,uniq_num_points)
    set_tics(plt4,uniq_find_frac,uniq_num_points)



fig, ((plt1,plt2),(plt3,plt4)) = plt.subplots(2,2)

is_running = True
def on_close(e):
    global is_running
    is_running = False
    print("asdf<")

#plt.ion()
fig.canvas.mpl_connect('close_event', on_close) # listen to close event

data = get_data()
shape = data.shape
plot()

plt.show(block=True)

while is_running:
    new_data = get_data()
    if new_data.shape != shape:
        data = new_data
        shape = data.shape
        plot()

    plt.pause(0.1)



