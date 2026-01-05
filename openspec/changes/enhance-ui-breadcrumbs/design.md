# Design: Clickable Breadcrumbs UI

## 1. 路径解析逻辑

`DirectoryListing.current_path` 是相对于共享根目录的路径（如 `/Movies/Action`）。

生成面包屑的步骤：
1.  将路径按 `/` 分割。
2.  过滤掉空字符串（处理开头和结尾的斜杠）。
3.  初始化一个 `accumulator` 路径字符串（从 `/` 开始）。
4.  第一项始终是 `Home` -> `href='/'`。
5.  后续每一项 `part`:
    *   `accumulator = format!("{}/{}", accumulator, part)` (注意处理 `/` 重复)。
    *   生成 `<a href='{accumulator}'>{part}</a>`。

## 2. HTML 结构

```html
<div class='header'>
  <a href='/'>Home</a>
  <span class='separator'>/</span>
  <a href='/Movies'>Movies</a>
  <span class='separator'>/</span>
  <span>Action</span> <!-- 最后一项可以是纯文本或链接 -->
</div>
```

## 3. CSS 样式

*   `.header a`: 蓝色或自定义链接色，去除下划线。
*   `.header a:hover`: 增加下划线或背景色。
*   `.separator`: 灰色，左右增加 `margin`。
