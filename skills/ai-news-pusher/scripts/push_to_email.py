#!/usr/bin/env python3
"""
AI 新闻推送到邮箱脚本
"""

import os
import sys
import json
import argparse
import smtplib
import subprocess
from email.mime.text import MIMEText
from email.mime.multipart import MIMEMultipart
from datetime import datetime
from pathlib import Path


def get_script_dir():
    """获取脚本所在目录"""
    return Path(__file__).parent.absolute()


def fetch_news(limit=8):
    """获取 AI 新闻"""
    try:
        script_dir = get_script_dir()
        result = subprocess.run(
            ['python3', str(script_dir / 'fetch_ai_news.py'), '--source', 'rss', '--limit', str(limit), '--days', '1', '--format', 'json'],
            capture_output=True,
            text=True,
            cwd=str(script_dir)
        )
        if result.returncode == 0:
            return json.loads(result.stdout)
        else:
            print(f"获取新闻失败：{result.stderr}", file=sys.stderr)
            return []
    except Exception as e:
        print(f"获取新闻失败：{e}", file=sys.stderr)
        return []


def format_email_html(news_list):
    """格式化 HTML 邮件内容"""
    if not news_list:
        return "<h2>📰 AI 新闻早报</h2><p>暂无新闻</p>"
    
    html = f"""
    <html>
    <head>
        <style>
            body {{ font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; }}
            h1 {{ color: #2c3e50; border-bottom: 2px solid #3498db; padding-bottom: 10px; }}
            .news-item {{ margin: 20px 0; padding: 15px; background: #f8f9fa; border-left: 4px solid #3498db; }}
            .news-title {{ font-size: 18px; font-weight: bold; color: #2c3e50; margin-bottom: 8px; }}
            .news-title a {{ color: #3498db; text-decoration: none; }}
            .news-title a:hover {{ text-decoration: underline; }}
            .news-meta {{ font-size: 12px; color: #7f8c8d; margin-top: 8px; }}
            .news-content {{ font-size: 14px; color: #34495e; line-height: 1.6; }}
        </style>
    </head>
    <body>
        <h1>📰 AI 新闻早报</h1>
        <p style="color: #7f8c8d;">更新时间：{datetime.now().strftime('%Y-%m-%d %H:%M')} | 共 {len(news_list)} 条新闻</p>
        <hr style="border: none; border-top: 1px solid #ddd;">
    """
    
    for i, news in enumerate(news_list, 1):
        title = news.get('title', '无标题')
        url = news.get('url', '#')
        content = news.get('content', '')[:300]
        source = news.get('source', '未知来源')
        date = news.get('published_date', '')
        
        html += f"""
        <div class="news-item">
            <div class="news-title">{i}. <a href="{url}" target="_blank">{title}</a></div>
            <div class="news-content">{content}...</div>
            <div class="news-meta">📌 来源：{source} | 📅 {date}</div>
        </div>
        """
    
    html += """
        <hr style="border: none; border-top: 1px solid #ddd;">
        <p style="color: #7f8c8d; font-size: 12px; text-align: center;">
            此邮件由 AI News Pusher 自动发送
        </p>
    </body>
    </html>
    """
    
    return html


def send_email(news_list, to_email, smtp_server, smtp_port, smtp_user, smtp_password, from_email=None, use_ssl=True, use_tls=False):
    """发送邮件"""
    if not from_email:
        from_email = smtp_user
    
    # 创建邮件
    msg = MIMEMultipart('alternative')
    msg['Subject'] = f"📰 AI 新闻早报 - {datetime.now().strftime('%Y-%m-%d')}"
    msg['From'] = from_email
    msg['To'] = to_email
    
    # HTML 内容
    html_content = format_email_html(news_list)
    msg.attach(MIMEText(html_content, 'html', 'utf-8'))
    
    try:
        # 连接 SMTP 服务器
        if use_ssl:
            server = smtplib.SMTP_SSL(smtp_server, int(smtp_port))
        else:
            server = smtplib.SMTP(smtp_server, int(smtp_port))
        
        if use_tls and not use_ssl:
            server.starttls()
        
        # 登录并发送
        server.login(smtp_user, smtp_password)
        server.sendmail(from_email, [to_email], msg.as_string())
        server.quit()
        
        print(f"✅ 邮件已发送至：{to_email}")
        return True
    
    except Exception as e:
        print(f"❌ 邮件发送失败：{e}", file=sys.stderr)
        return False


def main():
    parser = argparse.ArgumentParser(description='AI 新闻推送到邮箱')
    parser.add_argument('--limit', type=int, default=8, help='新闻数量')
    parser.add_argument('--to', type=str, required=True, help='收件人邮箱')
    parser.add_argument('--smtp-server', type=str, help='SMTP 服务器地址')
    parser.add_argument('--smtp-port', type=str, default='465', help='SMTP 端口')
    parser.add_argument('--smtp-user', type=str, help='SMTP 用户名')
    parser.add_argument('--smtp-password', type=str, help='SMTP 密码/授权码')
    parser.add_argument('--from-email', type=str, help='发件人邮箱（默认同 smtp-user）')
    parser.add_argument('--no-ssl', action='store_true', help='不使用 SSL')
    parser.add_argument('--tls', action='store_true', help='使用 TLS')
    
    args = parser.parse_args()
    
    # 从环境变量读取配置（如果命令行未提供）
    smtp_server = args.smtp_server or os.getenv('SMTP_SERVER')
    smtp_port = args.smtp_port or os.getenv('SMTP_PORT', '465')
    smtp_user = args.smtp_user or os.getenv('SMTP_USER')
    smtp_password = args.smtp_password or os.getenv('SMTP_PASSWORD')
    from_email = args.from_email or os.getenv('SMTP_FROM_EMAIL')
    
    if not all([smtp_server, smtp_user, smtp_password]):
        print("❌ 缺少 SMTP 配置，请通过命令行参数或环境变量提供：", file=sys.stderr)
        print("  --smtp-server 或 SMTP_SERVER", file=sys.stderr)
        print("  --smtp-user 或 SMTP_USER", file=sys.stderr)
        print("  --smtp-password 或 SMTP_PASSWORD", file=sys.stderr)
        sys.exit(1)
    
    # 获取新闻
    print(f"📰 正在获取 AI 新闻...")
    news_list = fetch_news(args.limit)
    
    if not news_list:
        print("⚠️ 未获取到新闻，跳过发送")
        sys.exit(0)
    
    # 发送邮件
    print(f"📧 正在发送邮件到 {args.to}...")
    success = send_email(
        news_list,
        args.to,
        smtp_server,
        smtp_port,
        smtp_user,
        smtp_password,
        from_email,
        use_ssl=not args.no_ssl,
        use_tls=args.tls
    )
    
    sys.exit(0 if success else 1)


if __name__ == '__main__':
    main()
