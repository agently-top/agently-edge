# TOOLS.md - Local Notes

Skills define _how_ tools work. This file is for _your_ specifics — the stuff that's unique to your setup.

## What Goes Here

Things like:

- Camera names and locations
- SSH hosts and aliases
- Preferred voices for TTS
- Speaker/room names
- Device nicknames
- Anything environment-specific

## Examples

```markdown
### Cameras

- living-room → Main area, 180° wide angle
- front-door → Entrance, motion-triggered

### SSH

- home-server → 192.168.1.100, user: admin

### TTS

- Preferred voice: "Nova" (warm, slightly British)
- Default speaker: Kitchen HomePod
```

## Why Separate?

Skills are shared. Your setup is yours. Keeping them apart means you can update skills without losing your notes, and share skills without leaking your infrastructure.

---

Add whatever helps you do your job. This is your cheat sheet.

---

## ☁️ 缤纷云 S3 同步

- **Endpoint**: `http://s3.bitiful.net/`
- **Bucket**: `siyuan3639`
- **Region**: `cn-east-1`
- **同步路径**: `workspace/`
- **脚本**: `scripts/sync-s3.py`
- **模式**: 双向同步（上传 + 下载）
- **冲突策略**: 保留两个版本（`.conflict-local.日期` 和 `.conflict-s3.日期`）
- **定时**: 每天 02:00 (Asia/Shanghai)
- **手动触发**: 说"同步到缤纷云"或"备份 workspace"
- **日志**: `memory/s3-sync-log.md`
