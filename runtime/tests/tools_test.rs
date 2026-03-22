use agently_edge::tools::{http::HttpTool, shell::ShellTool, Tool, ToolRegistry};

#[test]
fn test_tool_registry_register() {
    let mut registry = ToolRegistry::new();

    // 注册工具
    let http_tool = Box::new(HttpTool::new());
    registry
        .register(http_tool)
        .expect("Failed to register tool");

    // 验证工具已注册
    assert!(registry.has_tool("http"));
}

#[test]
fn test_tool_registry_execute() {
    let mut registry = ToolRegistry::new();

    // 注册 Shell 工具
    let shell_tool = Box::new(ShellTool::new());
    registry
        .register(shell_tool)
        .expect("Failed to register tool");

    // 执行工具
    let args = serde_json::json!({
        "command": "echo hello"
    });
    let result = registry
        .execute("shell", &args)
        .expect("Failed to execute tool");

    assert!(result.is_string());
}

#[test]
fn test_http_tool() {
    let tool = HttpTool::new();

    assert_eq!(tool.name(), "http");
    assert!(!tool.description().is_empty());
}

#[test]
fn test_shell_tool() {
    let tool = ShellTool::new();

    assert_eq!(tool.name(), "shell");
    assert!(!tool.description().is_empty());
}
