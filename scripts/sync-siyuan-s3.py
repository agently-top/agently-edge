#!/usr/bin/env python3
"""
思源笔记 S3 同步脚本
从缤纷云 S3 同步思源笔记数据到本地
"""

import boto3
import os
import sys
from pathlib import Path
from datetime import datetime

# S3 配置
S3_CONFIG = {
    'endpoint_url': 'http://s3.bitiful.net/',
    'aws_access_key_id': 'dvs6dRVSHEfmWFencClLFw5m',
    'aws_secret_access_key': 'kpg1VmNFK4R1Fj0mz6gCsqklfdw8fnG',
    'region_name': 'cn-east-1',
}
S3_BUCKET = 'siyuan3639'

# 本地目标目录
LOCAL_SIYUAN_DIR = Path('/root/.openclaw/workspace/siyuan-notebook')

def sync_from_s3():
    """从 S3 同步思源笔记数据到本地"""
    
    # 创建 S3 客户端
    s3 = boto3.client('s3', **S3_CONFIG)
    
    # 创建本地目录
    LOCAL_SIYUAN_DIR.mkdir(parents=True, exist_ok=True)
    
    print(f"📥 开始同步思源笔记到：{LOCAL_SIYUAN_DIR}")
    print(f"📦 S3 Bucket: {S3_BUCKET}/repo/")
    print("-" * 60)
    
    # 统计
    downloaded = 0
    skipped = 0
    errors = 0
    total_bytes = 0
    
    # 列出并下载所有 repo 目录下的文件
    paginator = s3.get_paginator('list_objects_v2')
    
    for page in paginator.paginate(Bucket=S3_BUCKET, Prefix='repo/'):
        if 'Contents' not in page:
            continue
            
        for obj in page['Contents']:
            key = obj['Key']
            local_path = LOCAL_SIYUAN_DIR / key
            
            # 创建目录
            local_path.parent.mkdir(parents=True, exist_ok=True)
            
            # 检查文件是否已存在且大小相同
            if local_path.exists():
                local_size = local_path.stat().st_size
                if local_size == obj['Size']:
                    skipped += 1
                    continue
            
            # 下载文件
            try:
                s3.download_file(S3_BUCKET, key, str(local_path))
                downloaded += 1
                total_bytes += obj['Size']
                
                # 进度显示
                if downloaded % 50 == 0:
                    print(f"  已下载：{downloaded} 文件 ({total_bytes / 1024 / 1024:.1f} MB)")
                    
            except Exception as e:
                errors += 1
                print(f"  ❌ 下载失败 {key}: {e}")
    
    print("-" * 60)
    print(f"✅ 同步完成!")
    print(f"   📥 新下载：{downloaded} 文件")
    print(f"   ⏭️  已跳过：{skipped} 文件")
    print(f"   ❌ 错误：{errors} 文件")
    print(f"   💾 总计：{total_bytes / 1024 / 1024:.1f} MB")
    print(f"   📁 本地路径：{LOCAL_SIYUAN_DIR}")

if __name__ == '__main__':
    sync_from_s3()
