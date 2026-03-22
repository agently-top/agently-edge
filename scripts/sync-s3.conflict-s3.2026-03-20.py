#!/usr/bin/env python3
"""
缤纷云 S3 同步脚本
同步 /root/.openclaw/workspace 到 S3 存储桶
"""

import os
import sys
import boto3
from botocore.config import Config
from datetime import datetime
import hashlib

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

# 排除的文件/文件夹
EXCLUDE_PATTERNS = [
    '.git/',
    'node_modules/',
    '.s3-credentials.env',
    '__pycache__/',
    '*.pyc',
    '.DS_Store',
]

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

def sync_to_s3(dry_run=False):
    """同步 workspace 到 S3"""
    
    # 创建 S3 客户端
    s3_client = boto3.client(
        's3',
        endpoint_url=ENDPOINT,
        aws_access_key_id=ACCESS_KEY,
        aws_secret_access_key=SECRET_KEY,
        region_name=REGION,
        config=Config(s3={'addressing_style': 'path'})
    )
    
    print(f"🔄 开始同步到缤纷云 S3")
    print(f"   存储桶：{BUCKET}")
    print(f"   端点：{ENDPOINT}")
    print(f"   源目录：{WORKSPACE_DIR}")
    print(f"   模式：{'干运行' if dry_run else '实际同步'}")
    print()
    
    uploaded = 0
    skipped = 0
    errors = 0
    
    # 遍历工作区文件
    for root, dirs, files in os.walk(WORKSPACE_DIR):
        # 过滤目录
        dirs[:] = [d for d in dirs if not should_exclude(os.path.join(root, d))]
        
        for filename in files:
            if should_exclude(filename):
                skipped += 1
                continue
            
            filepath = os.path.join(root, filename)
            relative_path = os.path.relpath(filepath, WORKSPACE_DIR)
            s3_key = f"workspace/{relative_path}"
            
            try:
                # 检查文件是否已存在且相同
                try:
                    remote_obj = s3_client.head_object(Bucket=BUCKET, Key=s3_key)
                    local_hash = get_file_hash(filepath)
                    remote_etag = remote_obj.get('ETag', '').strip('"')
                    
                    if local_hash == remote_etag:
                        print(f"⏭️  跳过 (未变化): {relative_path}")
                        skipped += 1
                        continue
                except:
                    pass  # 文件不存在，需要上传
                
                if dry_run:
                    print(f"📤 将上传：{relative_path}")
                    uploaded += 1
                else:
                    # 上传文件
                    s3_client.upload_file(filepath, BUCKET, s3_key)
                    print(f"✅ 已上传：{relative_path}")
                    uploaded += 1
                    
            except Exception as e:
                print(f"❌ 错误 {relative_path}: {e}")
                errors += 1
    
    print()
    print("=" * 50)
    print(f"同步完成!")
    print(f"  上传：{uploaded} 文件")
    print(f"  跳过：{skipped} 文件")
    print(f"  错误：{errors} 文件")
    print(f"  时间：{datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    
    return uploaded, skipped, errors

if __name__ == '__main__':
    dry_run = '--dry-run' in sys.argv
    sync_to_s3(dry_run=dry_run)
