# Design: Refined Port Selection Logic

## 1. 默认端口定义
选定 `9568` 作为默认的不常用端口。

## 2. 逻辑重构 (src/server.rs)

重写 `start` 函数中的端口确定部分：

```rust
let (listener, used_port) = if let Some(p) = port {
    // 显式指定：成功则用，失败则报错
    let addr = SocketAddr::from(([0, 0, 0, 0], p));
    (TcpListener::bind(addr).await?, p)
} else {
    // 自动选择：
    // Step 1: 尝试默认不常用端口
    let default_addr = SocketAddr::from(([0, 0, 0, 0], 9568));
    if let Ok(l) = TcpListener::bind(default_addr).await {
        (l, 9568)
    } else {
        // Step 2: 默认被占用，退避到随机逻辑
        let mut rng = rand::rng();
        loop {
            let p = rng.random_range(10000..=65535);
            let addr = SocketAddr::from(([0, 0, 0, 0], p));
            if let Ok(l) = TcpListener::bind(addr).await {
                break (l, p);
            }
        }
    }
};
```

## 3. 验证方案
- 第一次启动：应使用 9568。
- 开启第二个实例：由于 9568 被占，应使用随机端口。
- 显式指定 `--port 9568` 且被占：应直接报错。
