---
datetime=2023-11-03 10:32:00Z

url_name="rpress"

title="rpress"

tags= ["rust"]

category= "工具"
---

# rpress

一个简单的建站工具

## my

本人对写 markdown 来建站有着多次经验，使用 vue 写过自己的博客网站和后台管理系统，vuepress,vitepress 等等，然后关注到了 notion-next，使用 这个建站 也挺有趣的,主题配置等等，bug 也还是有挺多的，开始对于网站上的 bug，对其修复还是挺有意思的，但是，感觉还是差点意思。

## rust

上网冲浪的时候，看到 rust 这个语言，很多前端框架在使用 rust 重写一部分功能,所以 我也开始学习，b 站学习完后，准备做个小项目，然后在 b 站看到了 rigos(作者还是一个高中生，太牛了)，我就对其挺感兴趣，但是，我不知道如何来通过他的代码来配置文件生成，所以，我就根据他的 cargo 依赖来学习，分析代码，然后就一步步完善，生成站点（虽然不是很好），使用 rust 来操作文件还真是挺舒服的

## 遇到的问题

使用 markdown 这个依赖 无法对 h1..生成 id，看 github 上需要写个插件，有点麻烦，本来是要写一个 anchor 的，由于无法做到 标题生成 id,只能下次完善的时候希望 markdown 可以配置这个 options!
暂时热更新需要开启两个才能完成，终端运行如下命令

```cargo
cargo run --bin rpress
cargo run --bin hot
```

即可实现热更新

## 不足

rust 学的还不够好，面临找工作，暂时 不学习 rust 了，下阶段面试冲刺复习！,当然，还是使用 rpress 来记录自己的复习
