---
comments: true
---

# 2.5. &nbsp; 小结

### 算法效率评估

- 时间效率和空间效率是评价算法性能的两个关键维度。
- 我们可以通过实际测试来评估算法效率，但难以消除测试环境的影响，且会耗费大量计算资源。
- 复杂度分析可以克服实际测试的弊端，分析结果适用于所有运行平台，并且能够揭示算法在不同数据规模下的效率。

### 时间复杂度

- 时间复杂度用于衡量算法运行时间随数据量增长的趋势，可以有效评估算法效率，但在某些情况下可能失效，如在输入数据量较小或时间复杂度相同时，无法精确对比算法效率的优劣。
- 最差时间复杂度使用大 $O$ 符号表示，即函数渐进上界，反映当 $n$ 趋向正无穷时，$T(n)$ 的增长级别。
- 推算时间复杂度分为两步，首先统计计算操作数量，然后判断渐进上界。
- 常见时间复杂度从小到大排列有 $O(1)$ , $O(\log n)$ , $O(n)$ , $O(n \log n)$ , $O(n^2)$ , $O(2^n)$ , $O(n!)$ 等。
- 某些算法的时间复杂度非固定，而是与输入数据的分布有关。时间复杂度分为最差、最佳、平均时间复杂度，最佳时间复杂度几乎不用，因为输入数据一般需要满足严格条件才能达到最佳情况。
- 平均时间复杂度反映算法在随机数据输入下的运行效率，最接近实际应用中的算法性能。计算平均时间复杂度需要统计输入数据分布以及综合后的数学期望。

### 空间复杂度

- 类似于时间复杂度，空间复杂度用于衡量算法占用空间随数据量增长的趋势。
- 算法运行过程中的相关内存空间可分为输入空间、暂存空间、输出空间。通常情况下，输入空间不计入空间复杂度计算。暂存空间可分为指令空间、数据空间、栈帧空间，其中栈帧空间通常仅在递归函数中影响空间复杂度。
- 我们通常只关注最差空间复杂度，即统计算法在最差输入数据和最差运行时间点下的空间复杂度。
- 常见空间复杂度从小到大排列有 $O(1)$ , $O(\log n)$ , $O(n)$ , $O(n^2)$ , $O(2^n)$ 等。