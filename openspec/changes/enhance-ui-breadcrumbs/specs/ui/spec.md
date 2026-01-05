# UI Requirements

## MODIFIED Requirements

### Requirement: 目录列表页面路径展示
Web UI MUST 使用可点击的面包屑（Breadcrumbs）展示当前路径，取代静态文本。

#### Scenario: 根目录展示
当用户位于 `/` 时：
- 面包屑显示为 `Home`。
- `Home` 可点击（指向 `/`）。

#### Scenario: 深层目录展示
当用户位于 `/Movies/Action` 时：
- 面包屑显示为 `Home / Movies / Action`。
- `Home` 指向 `/`。
- `Movies` 指向 `/Movies`。
- `Action` 指向 `/Movies/Action` 或作为当前位置展示。

#### Scenario: 面包屑交互
用户点击面包屑中的 `Movies`：
- 浏览器跳转到 `http://<ip>:<port>/Movies`。
- 页面展示 `/Movies` 目录的内容。
