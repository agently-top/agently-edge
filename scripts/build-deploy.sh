#!/bin/bash
# 构建单文件部署版本
# 自动下载 llama.cpp 并静态链接

set -e

echo "🚀 Building agently Edge Runtime (single binary)"
echo "================================================"
echo

cd "$(dirname "$0")"

# 1. 检查依赖
echo "1️⃣  Checking dependencies..."

if ! command -v cmake &> /dev/null; then
    echo "❌ cmake not found. Please install:"
    echo "   apt-get install cmake  # Ubuntu/Debian"
    echo "   yum install cmake      # CentOS/RHEL"
    exit 1
fi

if ! command -v git &> /dev/null; then
    echo "❌ git not found. Please install git."
    exit 1
fi

echo "   ✅ cmake: $(cmake --version | head -1)"
echo "   ✅ git: $(git --version)"
echo

# 2. 编译 llama.cpp 和 runtime
echo "2️⃣  Building with static llama.cpp..."
echo "   This may take a few minutes..."
echo

# 设置编译选项
export RUSTFLAGS="-C target-feature=+crt-static"

# 编译 release 版本
cargo build --release --features full-llama

# 3. 检查结果
echo
echo "3️⃣  Build complete!"
echo

BINARY="./target/release/agently"
if [ -f "$BINARY" ]; then
    SIZE=$(du -h "$BINARY" | cut -f1)
    echo "   ✅ Binary: $BINARY"
    echo "   📦 Size: $SIZE"
    echo
    
    # 测试运行
    echo "4️⃣  Testing binary..."
    if $BINARY --help > /dev/null 2>&1; then
        echo "   ✅ Binary works correctly"
    else
        echo "   ⚠️  Binary may have issues"
    fi
    echo
else
    echo "   ❌ Binary not found!"
    exit 1
fi

# 5. 部署说明
echo "================================================"
echo "📦 Deployment Instructions"
echo "================================================"
echo
echo "Binary location: $BINARY"
echo
echo "To deploy to a device:"
echo "  1. Copy the binary:"
echo "     scp $BINARY user@device:/usr/local/bin/agently"
echo
echo "  2. Copy example config:"
echo "     scp -r examples user@device:~/"
echo
echo "  3. Run on device:"
echo "     ssh user@device"
echo "     agently run examples/hello-world/agent.yaml --mock"
echo
echo "To use with real model:"
echo "  1. Download GGUF model to device"
echo "  2. Run: agently run agent.yaml --verbose"
echo
echo "================================================"
echo "✅ Build successful!"
echo "================================================"
