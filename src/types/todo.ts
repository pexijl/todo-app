export interface Todo {
  id: string;
  title: string;
  done: boolean;
  created_at: string; // ISO 8601
}

export interface NewTodo {
  title: string;
}

export interface UpdateTodo {
  title?: string;
  done?: boolean;
}
