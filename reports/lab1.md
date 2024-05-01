# 2024-rCore-lab1-report
author:PantheraLeo14

## 我实现的功能

首先，我将所有的系统调用编号全部放入os/src/syscall/sysconfig.rs文件中，方便别的文件调用。

我在os/src/task/task.rs中的TaskControlBlock结构体中，新建了syscall_times，用以存储每个tasks的syscall次数

并且在os/src/task/mod.rs中完善TaskManager初始化，并且设置两个接口，一个为update_syscall_times，用以每个系统调用后增加syscall此数。另一个为get_syscall_times，获得次数。

然后在os/src/syscall/process.rs以及mod.rs中为每个系统调用调用update_syscall_times更新，然后在sys_task_info中使用get_time_ms获得时间，并且通过get_syscall_times，获得次数，设置task_info

## 问答题

### 1.三个程序出错行为
[rustsbi] RustSBI version 0.3.0-alpha.4, adapting to RISC-V SBI v1.0.0

[kernel] Loading app_0
[kernel] PageFault in application, kernel killed it.
用户的入口点为0x80400000，不能够向0x0写入

[kernel] Loading app_1
[kernel] IllegalInstruction in application, kernel killed it.
目前已经处于用户态，使用sret指令是从内核态返回用户态，这里不能执行，所以是非法的指令。

[kernel] Loading app_2
[kernel] IllegalInstruction in application, kernel killed it.
目前已经处于用户态，使用csrr指令无法读取sstatus内核态

### 2

1.
>a0是系统调用的返回值
两种使用场景：
一：操作作系统统完成系统调用用服务后，恢复被打断的应用程序Trap 上下文
二：开始运行第一个程序

2.
>t0、t1、t2：恢复sstatus，sepc，sscratch的值，不影响用户态应用程序的调用
sstatus：将CPU运行设置为U阶段
sepc：恢复Trap 发生之前执行的最后一条指令的地址
sscratch:描述 Trap 的原因

3.
>x2为sp寄存器，不能更改，因为我们要基于它来找到每个寄存器应该被保存到的正确的位置
x4为tp寄存器，除非我们手动出于一些特殊用途使用它，否则一般也不会被用到

4.
>现在 sp 指向用户栈， sscratch 指向内核栈

5.
>发生在sret指令，
该指令会执行以下功能:
CPU 会将当前的特权级按照 sstatus 的 SPP 字段设置为 U 或者 S ；
CPU 会跳转到 sepc 寄存器指向的那条指令，然后继续执行。


6.
>现在 sp 指向内核栈， sscratch 指向用户栈


7.
>call trap_handler

##  荣誉准则

在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

《无》

此外，我也参考了以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

《无》

我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

## 总结

第一遍读完文档和源码后不知道如何去做，在重新回头再读一次后豁然开朗，知道了lab如何设计，问答题基本都是ch2的内容，参考文档可以完成，总体难度不大。