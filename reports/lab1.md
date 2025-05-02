# 实现的功能

在TaskManager中用一个数组保存app的系统调用次数。系统调用id为数组索引。如果有hashmap之类的数据结构能用更好，但是现在没有，先用数组。系统调用入口处给给对应的系统调用加1。在sys_trace中读取对应的值。

# 简答题

## 1
ch2b_bad_address: PageFault in application, kernel killed it.
ch2b_bad_instructions: IllegalInstruction in application, kernel killed it.
ch2b_bad_register: IllegalInstruction in application, kernel killed it.

sbi version: Prereleased 2024-03-24

## 2

### 2.1

刚进入 `__restore` 时，sp 代表kernel stack.

`__restore`的两种使用场景：1. 开始执行用户程序的时候，2. 从trap返回到用户程序的时候。

### 2.2

这几行代码处理了`sstatus`, `sepc`, `sscratch`这三个寄存器。这几行代码的意义是在回到用户态之前恢复这三个csr的值。

### 2.3

x2是sp，单独处理。x4是tp，用户不会用到。

 
