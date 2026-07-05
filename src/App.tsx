import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Todo } from "./types/todo";
import "./App.css";

function App() {
  const [todos, setTodos] = useState<Todo[]>([]);

  useEffect(() => {
    invoke<Todo[]>("list_todos").then(setTodos).catch(console.error);
  }, []);

  return (
    <main className="container">
      <h1>Todo List</h1>
      <ul>
        {todos.map((todo) => (
          <li key={todo.id}>
            <input type="checkbox" checked={todo.done} onChange={() => {}} />
            <span>{todo.title}</span>
          </li>
        ))}
      </ul>
    </main>
  );
}

export default App;
