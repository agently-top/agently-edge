.. _api:

=============
API 参考
=============

运行时 API
==========

.. c:function:: void* agently_runtime_create(const char* agent_id, const char* model_path)

   创建运行时实例。
   
   :param agent_id: Agent 唯一标识
   :param model_path: 模型文件路径
   :return: 运行时指针，失败返回 NULL

.. c:function:: int agently_runtime_start(void* runtime)

   启动运行时。
   
   :param runtime: 运行时指针
   :return: 0 成功，-1 失败

.. c:function:: void agently_runtime_destroy(void* runtime)

   销毁运行时实例。
   
   :param runtime: 运行时指针

---

命令行接口
==========

agently run
-----------

运行 Agent。

.. code-block:: bash

   agently run <config.yaml>

agently send
------------

发送消息到 Agent。

.. code-block:: bash

   agently send <agent_id> <message>

agently status
--------------

查看 Agent 状态。

.. code-block:: bash

   agently status [agent_id]
