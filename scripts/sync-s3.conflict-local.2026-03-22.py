#!/usr/bin/env python3
"""
缤纷云 S3 双向同步脚本
同步 /root/.openclaw/workspace 与 S3 存储桶（双向）

冲突策略：保留两个版本
- 本地文件重命名为：filename.conflict-local.2026-03-19
- S3 文件下载到：filename.conflict-s3.2026-03-19
"""

import os
import sys
import boto3
from botocore.config import Config
from datetime import datetime
import hashlib
import json

# 加载凭证
from dotenv import load_dotenv
load_dotenv('/root/.openclaw/workspace/.s3-credentials.env')

# S3 配置
ACCESS_KEY = os.getenv('S3_ACCESS_KEY')
SECRET_KEY = os.getenv('S3_SECRET_KEY')
ENDPOINT = os.getenv('S3_ENDPOINT')
BUCKET = os.getenv('S3_BUCKET')
REGION = os.getenv('S3_REGION', 'cn-east-1')

# 本地工作区路径
WORKSPACE_DIR = '/root/.openclaw/workspace'
S3_PREFIX = 'workspace/'

# 排除的文件/文件夹
EXCLUDE_PATTERNS = [
    '.git/',
    'node_modules/',
    '.s3-credentials.env',
    '__pycache__/',
    '*.pyc',
    '.DS_Store',
    '.conflict-local.',
    '.conflict-s3.',
]

# 同步日志文件
SYNC_LOG = '/root/.openclaw/workspace/memory/s3-sync-log.md'

def should_exclude(path):
    """检查路径是否应该被排除"""
    for pattern in EXCLUDE_PATTERNS:
        if pattern in path:
            return True
    return False

def get_file_hash(filepath):
    """计算文件的 MD5 哈希"""
    hash_md5 = hashlib.md5()
    with open(filepath, "rb") as f:
        for chunk in iter(lambda: f.read(4096), b""):
            hash_md5.update(chunk)
    return hash_md5.hexdigest()

def get_s3_objects(s3_client, prefix=''):
    """列出 S3 上所有对象"""
    objects = []
    paginator = s3_client.get_paginator('list_objects_v2')
    
    for page in paginator.paginate(Bucket=BUCKET, Prefix=prefix):
        if 'Contents' in page:
            for obj in page['Contents']:
                objects.append(obj['Key'])
    
    return objects

def get_all_local_files():
    """获取本地所有文件及其相对路径"""
    files = {}
    for root, dirs, filenames in os.walk(WORKSPACE_DIR):
        # 过滤目录
        dirs[:] = [d for d in dirs if not should_exclude(os.path.join(root, d))]
        
        for filename in filenames:
            if should_exclude(filename):
                continue
            
            filepath = os.path.join(root, filename)
            relative_path = os.path.relpath(filepath, WORKSPACE_DIR)
            files[relative_path] = filepath
    
    return files

def sync_bidirectional(dry_run=False):
    """双向同步 workspace 和 S3"""
    
    # 创建 S3 客户端
    s3_client = boto3.client(
        's3',
        endpoint_url=ENDPOINT,
        aws_access_key_id=ACCESS_KEY,
        aws_secret_access_key=SECRET_KEY,
        region_name=REGION,
        config=Config(s3={'addressing_style': 'path'})
    )
    
    timestamp = datetime.now().strftime('%Y-%m-%d %H:%M:%S')
    date_stamp = datetime.now().strftime('%Y-%m-%d')
    
    print(f"🔄 开始双向同步到缤纷云 S3")
    print(f"   存储桶：{BUCKET}")
    print(f"   端点：{ENDPOINT}")
    print(f"   本地目录：{WORKSPACE_DIR}")
    print(f"   S3 前缀：{S3_PREFIX}")
    print(f"   模式：{'干运行' if dry_run else '实际同步'}")
    print(f"   冲突策略：保留两个版本")
    print()
    
    # 统计
    uploaded = 0
    downloaded = 0
    conflicts = 0
    skipped = 0
    errors = 0
    
    # 日志记录
    log_entries = {
        'uploaded': [],
        'downloaded': [],
        'conflicts': [],
        'skipped': [],
        'errors': []
    }
    
    # 获取本地和 S3 文件列表
    print("📊 扫描文件...")
    local_files = get_all_local_files()
    s3_keys = get_s3_objects(s3_client, S3_PREFIX)
    
    # 去除前缀，方便比较
    s3_files = {key.replace(S3_PREFIX, '', 1): key for key in s3_keys}
    
    print(f"   本地文件：{len(local_files)}")
    print(f"   S3 文件：{len(s3_files)}")
    print()
    
    all_paths = set(local_files.keys()) | set(s3_files.keys())
    
    for relative_path in all_paths:
        local_path = local_files.get(relative_path)
        s3_key = s3_files.get(relative_path)
        s3_full_key = f"{S3_PREFIX}{relative_path}"
        
        try:
            # 场景 1: 本地有，S3 无 → 上传
            if local_path and not s3_key:
                if dry_run:
                    print(f"📤 将上传：{relative_path}")
                    uploaded += 1
                    log_entries['uploaded'].append(relative_path)
                else:
                    s3_client.upload_file(local_path, BUCKET, s3_full_key)
                    print(f"✅ 已上传：{relative_path}")
                    uploaded += 1
                    log_entries['uploaded'].append(relative_path)
            
            # 场景 2: S3 有，本地无 → 下载
            elif s3_key and not local_path:
                if dry_run:
                    print(f"📥 将下载：{relative_path}")
                    downloaded += 1
                    log_entries['downloaded'].append(relative_path)
                else:
                    # 确保本地目录存在
                    local_full_path = os.path.join(WORKSPACE_DIR, relative_path)
                    os.makedirs(os.path.dirname(local_full_path), exist_ok=True)
                    s3_client.download_file(BUCKET, s3_full_key, local_full_path)
                    print(f"✅ 已下载：{relative_path}")
                    downloaded += 1
                    log_entries['downloaded'].append(relative_path)
            
            # 场景 3: 两边都有 → 比较
            elif local_path and s3_key:
                local_hash = get_file_hash(local_path)
                
                try:
                    # 获取 S3 文件的 ETag（MD5）
                    remote_obj = s3_client.head_object(Bucket=BUCKET, Key=s3_full_key)
                    remote_etag = remote_obj.get('ETag', '').strip('"')
                    
                    if local_hash == remote_etag:
                        # 内容相同，跳过
                        if not dry_run:
                            print(f"⏭️  跳过 (未变化): {relative_path}")
                        skipped += 1
                        log_entries['skipped'].append(relative_path)
                    else:
                        # 内容不同，冲突！
                        print(f"⚠️  冲突：{relative_path}")
                        
                        if dry_run:
                            conflicts += 1
                            log_entries['conflicts'].append(relative_path)
                        else:
                            # 保留两个版本
                            base, ext = os.path.splitext(relative_path)
                            
                            # 重命名本地文件
                            local_conflict_path = f"{base}.conflict-local.{date_stamp}{ext}"
                            os.rename(local_path, os.path.join(WORKSPACE_DIR, local_conflict_path))
                            print(f"   📁 本地版本 → {local_conflict_path}")
                            
                            # 下载 S3 版本
                            s3_conflict_path = f"{base}.conflict-s3.{date_stamp}{ext}"
                            s3_client.download_file(BUCKET, s3_full_key, os.path.join(WORKSPACE_DIR, s3_conflict_path))
                            print(f"   📁 S3 版本 → {s3_conflict_path}")
                            
                            conflicts += 1
                            log_entries['conflicts'].append({
                                'file': relative_path,
                                'local_backup': local_conflict_path,
                                's3_backup': s3_conflict_path
                            })
                            
                except Exception as e:
                    print(f"⚠️  无法比较 S3 文件：{e}")
                    errors += 1
                    log_entries['errors'].append(f"{relative_path}: {e}")
        
        except Exception as e:
            print(f"❌ 错误 {relative_path}: {e}")
            errors += 1
            log_entries['errors'].append(f"{relative_path}: {e}")
    
    # 打印统计
    print()
    print("=" * 60)
    print(f"双向同步完成!")
    print(f"  上传：{uploaded} 文件")
    print(f"  下载：{downloaded} 文件")
    print(f"  冲突：{conflicts} 文件（已保留两个版本）")
    print(f"  跳过：{skipped} 文件")
    print(f"  错误：{errors} 文件")
    print(f"  时间：{timestamp}")
    print("=" * 60)
    
    # 更新日志文件
    if not dry_run:
        update_sync_log(timestamp, uploaded, downloaded, conflicts, skipped, errors, log_entries)
    
    return uploaded, downloaded, conflicts, skipped, errors

def update_sync_log(timestamp, uploaded, downloaded, conflicts, skipped, errors, entries):
    """更新同步日志文件"""
    try:
        with open(SYNC_LOG, 'a', encoding='utf-8') as f:
            f.write(f"\n### {timestamp} - 双向同步\n\n")
            f.write(f"```\n")
            f.write(f"上传：{uploaded} 文件\n")
            f.write(f"下载：{downloaded} 文件\n")
            f.write(f"冲突：{conflicts} 文件\n")
            f.write(f"跳过：{skipped} 文件\n")
            f.write(f"错误：{errors} 文件\n")
            f.write(f"```\n\n")
            
            if entries['conflicts']:
                f.write("**冲突文件:**\n")
                for c in entries['conflicts']:
                    if isinstance(c, dict):
                        f.write(f"- `{c['file']}` → 本地：`{c['local_backup']}`, S3: `{c['s3_backup']}`\n")
                    else:
                        f.write(f"- `{c}`\n")
                f.write("\n")
    except Exception as e:
        print(f"⚠️  无法更新日志：{e}")

if __name__ == '__main__':
    dry_run = '--dry-run' in sys.argv
    sync_bidirectional(dry_run=dry_run)
