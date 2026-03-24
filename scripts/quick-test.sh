#!/bin/bash
# agently Edge 快速测试脚本

set -e

echo "🚀 agently Edge 快速测试"
echo "========================"
echo

# 1. 检查编译
echo "1️⃣  检查编译..."
if [ ! -f "./target/release/agently" ]; then
    echo "   ❌ 未找到编译产物，正在编译..."
    cargo build --release
else
    echo "   ✅ 编译产物存在"
fi
echo

# 2. 测试 CLI 帮助
echo "2️⃣  测试 CLI 帮助..."
./target/release/agently --help > /dev/null
echo "   ✅ CLI 帮助正常"
echo

# 3. Mock 模式测试
echo "3️⃣  测试 Mock 模式..."
echo "exit" | ./target/release/agently run examples/hello-world/agent.yaml --mock 2>&1 | grep -q "Hello" && echo "   ✅ Mock 模式正常" || echo "   ❌ Mock 模式失败"
echo

# 4. 运行测试套件
echo "4️⃣  运行测试套件..."
cargo test --quiet 2>&1 | tail -3
echo

# 5. 检查代码质量
echo "5️⃣  代码质量检查..."
cargo clippy --quiet 2>&1 && echo "   ✅ Clippy 检查通过" || echo "   ⚠️  Clippy 有警告"
cargo fmt --check 2>&1 && echo "   ✅ 格式检查通过" || echo "   ⚠️  格式需要调整"
echo

# 6. 总结
echo "========================"
echo "✅ 快速测试完成！"
echo
echo "下一步:"
echo "  - 运行示例：./target/release/agently run examples/hello-world/agent.yaml --mock"
echo "  - 查看文档：cat docs/DEPLOY.md"
echo "  - 完整测试：cargo test --all"
