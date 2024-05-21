import time
import numpy as np
import cupy as cp

# 创建一个大数组
N = 10**7
x_cpu = np.random.rand(N).astype(np.float32)
x_gpu = cp.array(x_cpu)

# 使用 CPU 计算
start_time = time.time()
y_cpu = np.square(x_cpu)
cpu_time = time.time() - start_time
print(f"CPU time: {cpu_time} seconds")

# 使用 GPU 计算
start_time = time.time()
y_gpu = cp.square(x_gpu)
cp.cuda.Stream.null.synchronize()  # 确保 GPU 计算完成
gpu_time = time.time() - start_time
print(f"GPU time: {gpu_time} seconds")

# 检查结果是否相同
print(f"Results are equal: {np.allclose(cp.asnumpy(y_gpu), y_cpu)}")
