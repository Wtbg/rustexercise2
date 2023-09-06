# README

## Introduce

本仓库是ZJU短学期Rust课程的第三次课的作业，包含三个子任务，分别是

1. 使用宏实现一个哈希表，底层调用系统库的`std::colloction::HashMap`

2. 实现一个堆栈，底层使用Vec，完成堆栈的基础功能

3. 实现一个MyRc，类似标准库中的Rc

项目的文件结构如下：

![tree.png](https://img1.imgtp.com/2023/09/06/LigD2RVF.png)

其中在`src`文件夹中三个文件中顾名思义，包含三个不同的实现，在`main.rs`中包含一个空的`main`函数和三个实现对应的测试代码。



## Improvement

对课件中的hash_map做了一些改进，像`Rust`中struct的定义中的一样，可以可选的选择最后一个项中是否使用都好，原实现中必须有逗号，不够优雅
现在允许用法：

```rust
let my_map = hash_map!{
    "key1" => "value1",
    "key2" => "value2",
};
```

同时允许：

```rust
let my_map = hash_map!{
    "key1" => "value1",
    "key2" => "value2"
};

```



## Test Result

测试通过

![test.png](https://img1.imgtp.com/2023/09/06/06ugPFFG.png)

代码无警告

![clippy.png](https://img1.imgtp.com/2023/09/06/l2EMmFoO.png)

## 

## Beg Score

乞求助教给给高分qwq
