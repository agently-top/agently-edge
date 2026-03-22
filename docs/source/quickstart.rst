.. _quickstart:

==========
快速开始
==========

本指南帮助你在 10 分钟内运行第一个 Agent。

---

前置要求
========

- 已安装 agently Edge Runtime (:ref:`installation`)
- 已准备模型文件（GGUF 格式）

---

步骤 1: 准备模型
================

下载量化后的模型：

.. code-block:: bash

   # 示例：下载 Qwen2.5-1.5B-Instruct (Q4_K_M)
   wget https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct-GGUF/resolve/main/qwen2.5-1.5b-instruct-q4_k_m.gguf
   
   # 移动到模型目录
   sudo mkdir -p /opt/agently/models
   sudo mv qwen2.5-1.5b-instruct-q4_k_m.gguf /opt/agently/models/

---

步骤 2: 创建 Agent 配置
===========================

创建配置文件 ``agent.yaml``：

.. code-block:: yaml

   agent_id: hello-world
   name: Hello World Agent
   model:
     path: /opt/agently/models/qwen2.5-1.5b-instruct-q4_k_m.gguf
     context_length: 4096
     temperature: 0.7
   
   prompts:
     system: |
       你是一个友好的 AI 助手。
       用简洁、专业的语言回答问题。
     greeting: 你好！我是 agently Edge Agent，有什么可以帮你的吗？
   
   output:
     type: text

---

步骤 3: 运行 Agent
====================

.. code-block:: bash

   # 运行 Agent
   agently run agent.yaml
   
   # 输出：
   # [INFO] Starting runtime with config: hello-world
   # [INFO] Agent initialized
   # [INFO] Ready for input

---

步骤 4: 与 Agent 交互
=========================

在另一个终端发送消息：

.. code-block:: bash

   # 发送消息
   agently send hello-world "你好，请介绍一下自己"
   
   # 输出：
   # 你好！我是 agently Edge Agent，一个运行在边缘设备上的 AI 助手...

---

下一步
======

- 查看 :ref:`api` 了解完整 API
- 查看示例项目：https://github.com/agently-top/agently-edge/tree/main/examples
- 加入社区：https://discord.gg/clawd
