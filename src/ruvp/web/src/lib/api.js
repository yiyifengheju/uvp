const BASE = '';

export async function fetchProjects() {
  const res = await fetch(`${BASE}/api/projects`);
  if (!res.ok) throw new Error('Failed to fetch projects');
  return res.json();
}

export async function fetchProjectOverview(id) {
  const res = await fetch(`${BASE}/api/projects/${id}/overview`);
  if (!res.ok) throw new Error('Failed to fetch project overview');
  return res.json();
}

export async function addTodo(projectId, content) {
  const res = await fetch(`${BASE}/api/projects/${projectId}/todos`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ content }),
  });
  if (!res.ok) throw new Error('Failed to add todo');
  return res.json();
}

export async function patchTodo(projectId, todoId, done) {
  const res = await fetch(`${BASE}/api/projects/${projectId}/todos/${todoId}`, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ done }),
  });
  if (!res.ok) throw new Error('Failed to patch todo');
  return res.json();
}

export async function deleteTodo(projectId, todoId) {
  const res = await fetch(`${BASE}/api/projects/${projectId}/todos/${todoId}`, {
    method: 'DELETE',
  });
  if (!res.ok) throw new Error('Failed to delete todo');
}
