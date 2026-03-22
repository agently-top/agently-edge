# 缤纷云 S3 同步日志

## 配置信息

- **服务商**: 缤纷云 (Binfen Cloud)
- **Endpoint**: http://s3.bitiful.net/
- **Bucket**: siyuan3639
- **Region**: cn-east-1
- **同步目录**: `/root/.openclaw/workspace`
- **S3 路径**: `workspace/`

## 同步脚本

- **位置**: `/root/.openclaw/workspace/scripts/sync-s3.py`
- **凭证**: `/root/.openclaw/workspace/.s3-credentials.env`

## 定时任务

- **频率**: 每天凌晨 2:00 (Asia/Shanghai)
- **Cron ID**: `5a268671-2eda-4157-a61f-06f549434ff4`

## 同步记录

### 2026-03-19 09:01:55 - 首次同步

```
上传：130 文件
跳过：1 文件
错误：0 文件
```

---

## 手动同步

随时说以下命令触发手动同步：
- "同步到缤纷云"
- "备份 workspace"
- "同步到 S3"

### 2026-03-22 02:00:38 - 双向同步

```
上传：27 文件
下载：2 文件
冲突：3 文件
跳过：3164 文件
错误：0 文件
```

**冲突文件:**
- `scripts/sync-s3.py` → 本地：`scripts/sync-s3.conflict-local.2026-03-22.py`, S3: `scripts/sync-s3.conflict-s3.2026-03-22.py`
- `skills/.skills_store_lock.json` → 本地：`skills/.skills_store_lock.conflict-local.2026-03-22.json`, S3: `skills/.skills_store_lock.conflict-s3.2026-03-22.json`
- `.clawhub/lock.json` → 本地：`.clawhub/lock.conflict-local.2026-03-22.json`, S3: `.clawhub/lock.conflict-s3.2026-03-22.json`

