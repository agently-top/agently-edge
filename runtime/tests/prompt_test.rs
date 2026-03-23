use agently_edge::llm::{PromptManager, Role};

#[test]
fn test_prompt_manager_add_message() {
    let mut manager = PromptManager::new();

    manager.add_system("You are a helpful assistant.");
    manager.add_user("Hello!");

    let messages = manager.build_messages();
    assert_eq!(messages.len(), 2);
    assert_eq!(messages[0].role, Role::System);
    assert_eq!(messages[1].role, Role::User);
}

#[test]
fn test_prompt_manager_with_history() {
    let mut manager = PromptManager::new();

    manager.add_system("You are a helpful assistant.");
    manager.add_user("Question 1");
    manager.add_assistant("Answer 1");
    manager.add_user("Question 2");

    let messages = manager.build_messages();
    assert_eq!(messages.len(), 4);
}

#[test]
fn test_prompt_manager_template() {
    let mut manager = PromptManager::new();

    manager.set_system_template("You are {role}.");
    manager.add_variable("role", "a helpful shopping assistant");
    manager.add_user("Hello!");

    let messages = manager.build_messages();
    assert!(messages[0].content.contains("shopping assistant"));
}
