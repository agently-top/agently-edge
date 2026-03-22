# 思源笔记 API 配置指南

## 第一步：在思源笔记客户端开启 HTTP API

1. 打开思源笔记客户端
2. 进入 **设置** → **关于** → **API**
3. 开启 **HTTP API** 开关
4. 设置 API 端口（默认 `6806`）
5. 生成并复制 **API Token**

## 第二步：获取笔记本 ID

运行以下命令获取笔记本列表：

```bash
cd /root/.openclaw/workspace/skills/siyuan-skill
SIYUAN_BASE_URL=http://<你的思源笔记 IP>:6806 \
SIYUAN_TOKEN=<你的 API Token> \
node siyuan.js notebooks
```

或者直接在浏览器访问（如果思源笔记开启了 API）：
```
http://<你的思源笔记 IP>:6806/api/notebook/lsNotebooks
```

## 第三步：配置环境变量

创建配置文件 `/root/.openclaw/workspace/.siyuan-env`：

```bash
# 思源笔记 API 地址
# 如果思源笔记在同一台机器，使用 127.0.0.1
# 如果在其他机器，使用对应 IP 地址
SIYUAN_BASE_URL=http://127.0.0.1:6806

# API Token（从思源笔记客户端获取）
SIYUAN_TOKEN=你的 API Token

# 默认笔记本 ID（从 notebooks 命令获取）
SIYUAN_DEFAULT_NOTEBOOK=你的笔记本 ID
```

## 第四步：测试连接

```bash
cd /root/.openclaw/workspace/skills/siyuan-skill
source /root/.openclaw/workspace/.siyuan-env
node siyuan.js notebooks
```

## 通过 OpenClaw 对话操作笔记

配置完成后，你可以直接对我说：

- "在思源笔记中创建一个新文档，标题是 XXX"
- "搜索思源笔记中关于 XXX 的内容"
- "列出我的笔记本"
- "查看 XXX 文档的内容"
- "更新 XXX 文档，添加以下内容..."

---

## 安全提示

⚠️ **重要**：
- API Token 相当于密码，不要泄露
- 建议思源笔记 API 只监听本地（127.0.0.1）
- 如果思源笔记在远程服务器，建议使用 SSH 隧道
