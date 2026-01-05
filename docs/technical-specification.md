
# Air - 轻量级局域网文件共享 CLI 技术文档

## 1. 系统概述

`air` 是一个命令行工具，用于快速将本地目录转换为 HTTP 文件服务器。它强调安全性、极简的 UI 和对断点续传（Range Request）的支持。

### 1.1 核心功能
*   **指令启动**：通过 `air` 命令启动。
*   **目录映射**：将指定文件夹（默认为当前工作目录）映射为 Web 根目录。
*   **极简 Web UI**：以列表形式展示文件/文件夹，仅显示名称、类型（图标区分）和大小。
*   **断点续传**：支持 HTTP `Range` 头，允许大文件分段下载或视频流式播放。
*   **安全沙箱**：严格限制访问范围，禁止路径穿越（Path Traversal）。
*   **终端监控**：启动时显示局域网 IP，运行时实时打印访问者 IP 和请求状态。

---

## 2. CLI 交互设计

### 2.1 启动参数
程序应解析以下命令行参数：

```bash
air [path] [flags]
```

*   **Positional Argument (可选)**:
    *   `path`: 要共享的目标文件夹路径。如果未提供，默认为当前目录 (`.`)。
*   **Flags**:
    *   `-p, --port <int>`: 指定监听端口。默认: `8000`（若被占用则自动递增或随机）。
    *   `-h, --help`: 显示帮助信息。

### 2.2 启动输出 (Startup Banner)
启动成功后，必须获取本机在局域网（LAN）中的非回环 IPv4 地址并显示：

```text
User defined path: /Users/username/Downloads/Movies
Security Check: SANDBOX ENABLED 🔒

Air is serving at:
  ➜  Local:   http://localhost:8000
  ➜  Network: http://192.168.1.5:8000

Hit CTRL-C to stop the server
```

### 2.3 运行时日志 (Runtime Logging)
所有 HTTP 请求必须在终端实时输出，格式如下：
`[时间] [客户端IP] [HTTP方法] [路径] [状态码] [耗时]`

**示例：**
```text
[14:20:01] 192.168.1.15 GET /folder/test.mp4 206 (Partial Content) - 50ms
[14:20:05] 192.168.1.15 GET /favicon.ico 404 (Not Found) - 2ms
```

---

## 3. 安全实现细则 (核心)

为防止恶意访问系统文件（如 `/etc/passwd`），必须在处理任何请求前执行以下路径清洗逻辑：

1.  **解析根目录**：程序启动时，获取共享目录的**绝对路径**，记为 `ROOT_PATH`。
2.  **路径拼接**：对于请求路径 `REQ_PATH`，计算 `TARGET_PATH = filepath.Join(ROOT_PATH, REQ_PATH)`。
3.  **防穿越检查 (Jail Check)**：
    *   如果 `TARGET_PATH` 不以 `ROOT_PATH` 开头，立即拒绝 (403 Forbidden)。
    *   必须解析所有符号链接（Symlinks）。如果符号链接指向 `ROOT_PATH` 之外，必须拒绝访问或将其视为普通文件不予跟随。
4.  **隐藏文件过滤**：默认不展示以 `.` 开头的文件（可选配置）。

---

## 4. Web UI 与数据结构

为了让 OpenAPI 能够完美描述，我们将 UI 渲染与数据获取分离，或者采用服务端渲染 HTML。为了实现的灵活性，本文档定义**API 优先**模式，Web 界面通过 AJAX 调用 API 或由服务器直接渲染 HTML。

**UI 界面要求：**
*   **Header**：显示当前路径面包屑（如: `Home > Movies > Action`）。
*   **List**：
    *   **Folder**：点击进入下一级目录。
    *   **File**：点击触发浏览器下载行为。
*   **Meta**：显示文件大小（格式化为 KB, MB, GB）。

---

## 5. OpenAPI Specification (v3.0.3)

以下 YAML 可直接导入 Swagger Editor 或代码生成工具。它定义了文件浏览和下载的核心协议。

```yaml
openapi: 3.0.3
info:
  title: Air File Server API
  description: A secure, lightweight local file sharing API supporting range requests.
  version: 1.0.0
servers:
  - url: /
    description: Local Air Server

paths:
  /{path}:
    get:
      summary: Serve file content or directory listing
      description: >
        Smart endpoint. If the path points to a directory, it returns a JSON listing (or HTML view).
        If it points to a file, it streams the file content with Range support.
      parameters:
        - in: path
          name: path
          schema:
            type: string
            default: ""
          required: true
          description: Relative path from the shared root directory. Use root slash for base dir.
          allowReserved: true
      responses:
        '200':
          description: Directory listing or Full File Content
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/DirectoryListing'
            text/html:
              schema:
                type: string
                description: HTML Web Interface for the directory
            application/octet-stream:
              schema:
                type: string
                format: binary
        '206':
          description: Partial Content (For Range Requests/Resumable Downloads)
          headers:
            Content-Range:
              description: Indicates the position of the partial content in the full file.
              schema:
                type: string
                example: "bytes 0-1023/2048"
            Content-Length:
              schema:
                type: integer
            Accept-Ranges:
              schema:
                type: string
                example: "bytes"
          content:
            application/octet-stream:
              schema:
                type: string
                format: binary
        '403':
          description: Forbidden (Security violation, trying to access outside root)
        '404':
          description: File or Directory not found

components:
  schemas:
    DirectoryListing:
      type: object
      properties:
        currentPath:
          type: string
          description: The current relative path being viewed
        items:
          type: array
          items:
            $ref: '#/components/schemas/FileEntry'
    
    FileEntry:
      type: object
      properties:
        name:
          type: string
          example: "vacation.mp4"
        isDir:
          type: boolean
          example: false
        size:
          type: integer
          description: Size in bytes
          example: 10485760
        modTime:
          type: string
          format: date-time
```

---

## 6. 实现逻辑伪代码 (供开发者参考)

### 6.1 HTTP Handler 核心逻辑

```go
function handleRequest(w ResponseWriter, r Request) {
    // 1. 获取客户端 IP 用于日志
    clientIP = r.RemoteAddr
    
    // 2. 路径清洗
    safePath = sanitizePath(baseDir, r.URL.Path)
    if error {
        log(clientIP, "403 Forbidden")
        return 403
    }

    // 3. 判断文件类型
    fileInfo = getStat(safePath)
    
    if fileInfo.IsDir() {
        // 如果请求头 Accept 包含 application/json，返回 JSON
        // 否则返回 HTML 模板渲染的页面
        renderDirectory(w, safePath)
        log(clientIP, "200 OK (Dir)")
    } else {
        // 4. 文件下载与断点续传处理
        serveFile(w, r, safePath) 
        // serveFile 内部需要自动处理 Range 头，返回 200 或 206
        log(clientIP, "200/206 (File Download)")
    }
}
```

### 6.2 HTML 模板要求 (简约风格)
生成的 HTML 应包含以下 CSS 样式逻辑：
*   **布局**：最大宽度 800px，居中。
*   **字体**：系统无衬线字体 (San Francisco, Segoe UI, Roboto)。
*   **列表**：去除列表样式，增加 padding，增加鼠标悬停背景变色。
*   **图标**：使用 emoji 或简单的 SVG 区分文件夹 (📁) 和文件 (📄)。

## 7. 总结
开发者在实现时，应首先搭建基于 OpenAPI 定义的 HTTP 服务，然后实现 CLI 参数解析与 IP 获取逻辑，最后通过中间件层加入安全沙箱检查与日志记录。`Range` 支持通常可利用语言标准库中的 `FileServer` 或类似功能实现。
