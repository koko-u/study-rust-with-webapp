use assert2::check;
use todo_app::todos::models::{CreateTodo, Todo, UpdateTodo};
use todo_app::todos::repositories::in_memory::TodoRepositoryInMemory;
use todo_app::todos::repositories::TodoRepository;

#[tokio::test]
async fn success_create_todo() {
    let text = "Todo Text 1";
    let id = 1_u64;

    let repository = TodoRepositoryInMemory::new();
    let todo = repository.create(CreateTodo {
        text: text.to_string(),
    });

    check!(todo == Todo::new(id, text));
}

#[tokio::test]
async fn success_find_todo() {
    let text = "Todo Text 1";
    let id = 1_u64;

    let repository = TodoRepositoryInMemory::new();
    repository.create(CreateTodo {
        text: text.to_string(),
    });

    let todo = repository.find(id);
    check!(todo == Some(Todo::new(id, text)));
}

#[tokio::test]
async fn success_all_todo() {
    let text1 = "Todo Text 1";
    let text2 = "Todo Text 2";

    let repository = TodoRepositoryInMemory::new();
    repository.create(CreateTodo {
        text: text1.to_string(),
    });
    repository.create(CreateTodo {
        text: text2.to_string(),
    });

    let mut todos = repository.all();
    todos.sort_by_key(Todo::id);
    check!(todos == vec![Todo::new(1, text1), Todo::new(2, text2)]);
}

#[tokio::test]
async fn success_update_todo() {
    let text1 = "Todo Text 1";
    let text2 = "Todo Text 2";

    let repository = TodoRepositoryInMemory::new();
    let created = repository.create(CreateTodo {
        text: text1.to_string(),
    });
    let updated = repository.update(
        created.id(),
        UpdateTodo {
            text: Some(text2.to_string()),
            completed: Some(true),
        },
    );

    let mut expected = Todo::new(created.id(), text2);
    expected.completed = true;

    check!(updated.is_ok());
    check!(updated.unwrap() == expected);
}

#[tokio::test]
async fn success_delete_todo() {
    let text = "Todo Text 1";

    let repository = TodoRepositoryInMemory::new();
    let created = repository.create(CreateTodo {
        text: text.to_string(),
    });

    let deleted = repository.delete(created.id());
    let found = repository.find(created.id());

    check!(deleted.is_ok());
    check!(found.is_none());
}
