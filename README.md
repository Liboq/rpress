# rust simple example

rust blog

## introduce

正在进行中
使用 rust 实现一个博客生成工具

## config

在`config.toml`中定义配置文件，

```toml
navbar=[{text ="md编写",link="/markdown/md-edit",children=[]}]
base = "./rpress"
version = "1.0.0"
title = "rpress"
host = "0.0.0.0"
port = 3000
dest = "docs"
description="666"
source="markdown"
enviroment="dev"
```

navbar: 暂未开发
base：github 地址名
version：暂未开发
host : 暂未开发
port : 暂未开发
dest：文件输出地址
description：站点描述（暂未开发）
source：markdown 目录
enviroment：环境为 dev 时本地开发 可以预览`127.0.0.1:7878`,如果需要放到 github 上则需要为 prod

## use

终端运行如下命令

```cargo
cargo run --bin rpress
cargo run --bin hot
```

## feature

markdown-anchor

## reference

`https://github.com/Candlest/rigos`
