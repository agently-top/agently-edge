.. _installation:

============
安装指南
============

本文档介绍如何在不同平台上安装 agently Edge Runtime。

---

Linux 系统
==========

前置要求
--------

- 操作系统：Ubuntu 20.04+ / Debian 11+ / CentOS 8+
- 架构：x86_64 或 ARM64
- 内存：≥2GB
- 存储：≥16GB

安装方式 1: 下载二进制
---------------------------

.. code-block:: bash

   # 下载二进制文件
   wget https://github.com/agently-top/agently-edge/releases/download/v1.0.0/agently-linux-x64
   
   # 添加执行权限
   chmod +x agently-linux-x64
   
   # 移动到系统路径
   sudo mv agently-linux-x64 /usr/local/bin/agently
   
   # 验证安装
   agently --version

安装方式 2: 包管理
---------------------

**Ubuntu/Debian:**

.. code-block:: bash

   wget -qO - https://agently-top.github.io/agently-edge/gpg.key | sudo apt-key add -
   echo "deb [arch=amd64] https://agently-top.github.io/agently-edge/apt stable main" | sudo tee /etc/apt/sources.list.d/agently-edge.list
   sudo apt update
   sudo apt install agently-edge

**CentOS/RHEL:**

.. code-block:: bash

   sudo yum-config-manager --add-repo https://agently-top.github.io/agently-edge/rpm/agently-edge.repo
   sudo yum install agently-edge

验证安装
--------

.. code-block:: bash

   agently --version
   # 输出：agently-edge 0.1.0

---

Android 设备
============

前置要求
--------

- Android 10+
- 内存：≥2GB
- 存储：≥16GB

安装方式
--------

1. 下载 APK 文件
2. 在设备上安装 APK
3. 打开应用授予权限

.. note::

   Android 版本需要设备厂商预装或侧载 APK。

---

鸿蒙 Next
=========

前置要求
--------

- HarmonyOS NEXT
- 内存：≥2GB
- 存储：≥16GB

安装方式
--------

1. 通过鸿蒙应用市场搜索 "agently Edge"
2. 点击安装

.. note::

   鸿蒙版本正在开发中，敬请期待。

---

下一步
======

安装完成后，继续 :ref:`quickstart` 开始使用。
