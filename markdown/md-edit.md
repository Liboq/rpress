---
title= "编写一个markdown-it-navbar库"

datetime= 2022-09-07 10:32:00Z

category= 'vue'

tags=['插件',"组件"]

url_name='md-edit'
---

## 起因

在做自己博客网站的时候，需要做一个根据页面内容来显示标题导航的功能，例如根据 h1,h2,h3 来导航文章内容，文章内容变化标题导航的 menu 的 active 类也会切换，所以我就想做一个相关的库，方便有相同需求的朋友。

## 如何实现

### 获取 menu

首先我们需要根据拿到页面的内容，我最开始是后端直接传文章内容过来，所以最开始一版只通过 markdown 的内容来渲染标题导航的 menu，后面优化可以通过获取元素内容来渲染 menu，代码如下

```javascript
// 根据 props 是否 根据 元素html内容来显示
// props.container 表示 元素的标识 例如‘class''id'
import MarkdownIt from "markdown-it";
const transTextToHtml = () => {
  if (props.container) {
    html.value = contentText.value;
  } else {
    html.value = MarkdownIt().render(props.content);
  }
};
// 获取menu的text数组
const getMenuText = () => {
  menuText.value = [];
  menu.value = html.value.match(regExe) || [];
  menuText.value.pop();
  menu.value.forEach((item: string) => {
    let s = "";
    let index;
    let reg = new RegExp(/<h\d(([\s\S])*?)>/, "g");
    s = item.replace(/<\/h\d>/, "").replace(reg, "");
    index = props.target.indexOf(item.split("")[1] + item.split("")[2]);
    if (index === -1) {
      return;
    }
    if (s.indexOf("</span>") !== -1) {
      s = s.replace("</span>", "").replace(/<span(([\s\S])*?)>/g, ""); // 过滤其他标签
    }
    if (s.indexOf("</a>") !== -1) {
      s = s.replace("</a>", "").replace(/<a(([\s\S])*?)>/g, ""); // 过滤其他标签
    }

    menuText.value.push({ text: s, level: index + 1 });
  });
};
```

### 获取实时内容

由于最初的内容是后端传过来是死的，所以开始并没有对内容做监听，后来需要适配编写 md 内容的时候实时获取 menu，所以就需要监听内容的变化来实时渲染 menu，相关代码如下：

```javascript
const contentText = computed(() => {
  if (!props.container) {
    return props.content;
  }
  return document.querySelector(props.container)!.innerHTML || "";
});
if (props.isWatched) {
  watch(contentText, () => {
    transTextToHtml();
    getMenuText();
  });
}
```

### 获取 active 的标题

根据滚动距离获取显示在最上面的标题来渲染 active 的 menuItem,在这里有个地方需要注意，有可能滚动的不是 document 而是某个元素，所以在这里需要判断一下，获取正确的 offsetTop

```javascript
const anchorClass = computed(() => {
  if (typeof props.classes === "string") {
    return {
      "anchor-container": true,
      "anchor-list": !props.classes,
      [`${props.classes}`]: props.classes,
    };
  } else {
    return {
      "anchor-container": true,
      ...props.classes,
    };
  }
});
const winScroll = throttle((e) => {
  let offsetTop
  if(props.scrollEL == "document"){
    offsetTop = 0
    scrollTop.value= document.documentElement.scrollTop || document.body.scrollTop
  }else{
    scrollTop.value = e.target.scrollTop
    offsetTop = e.target.offsetTop

  }
  for (let value of menuText.value) {
    if (document.getElementById(slugify(value.text))) {
      const entry = document.getElementById(slugify(value.text)) as HTMLElement;
      if (entry.offsetTop - 1-offsetTop <= scrollTop.value) {
        indexActive.value = menuText.value.indexOf(value);
      }
    }
  }
}, 10);
const getActiveIndex = () => {
  let scrollel;
  if (props.scrollEL == "document") {
    scrollel = document;
  } else {
    scrollel = document.querySelector(props.scrollEL);
  }
  scrollel!.addEventListener("scroll", winScroll, false);

};
```

### 监听路由变化

在给组件库加上我这个库时发现路由一发生改变，menu 的内容没有变动，发现是由于没有监听路由变化来重新渲染 menu，所以加上了一个监听路由的功能,由于浏览器没有对 pushState 和 replaceState 事件的监听，所以在监听浏览器变化的时候，需要给事件添加监听，然后根据是否需要监听路由的添加浏览器的监听事件

```javascript
const pushState = window.history.pushState;
window.history.pushState = function (
  state: State,
  title: string,
  url?: string | null
): void {
  pushState.apply(window.history, [state, title, url]);
  const event: any = new Event("pushstate");
  event.state = state;
  event.title = title;
  event.url = url;
  window.dispatchEvent(event);
};

const replaceState = window.history.replaceState;
window.history.replaceState = function (
  state: State,
  title: string,
  url?: string | null
): void {
  replaceState.apply(window.history, [state, title, url]);
  const event: any = new Event("replacestate");
  event.state = state;
  event.title = title;
  event.url = url;
  window.dispatchEvent(event);
};
onMounted(() => {
  if (props.route) {
    window.addEventListener("pushstate", deferGetMenu);
    window.addEventListener("popstate", deferGetMenu);
  }
});
```

下面是我的 github 仓库[markdwon-it-navbar](https://github.com/Liboq/markdown-it-navbar),欢迎 jym 给我提 issues🤓
