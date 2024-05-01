# 2024-rCore-lab1-report
author:PantheraLeo14

## 我实现的功能

首先，我将所有的系统调用编号全部放入os/src/syscall/sysconfig.rs文件中，方便别的文件调用。

我在os/src/task/task.rs中的TaskControlBlock结构体中，新建了syscall_times，用以存储每个tasks的syscall次数

并且在os/src/task/mod.rs中完善TaskManager初始化，并且设置两个接口，一个为update_syscall_times，用以每个系统调用后增加syscall此数。另一个为get_syscall_times，获得次数。

然后在os/src/syscall/process.rs以及mod.rs中为每个系统调用调用update_syscall_times更新，然后在sys_task_info中使用get_time_ms获得时间，并且通过get_syscall_times，获得次数，设置task_info

## 问答题



##  荣誉准则

在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

《无》

此外，我也参考了以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

《无》

我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。