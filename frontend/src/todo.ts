const API_ENDPOINT = "http://localhost:5036/graphql";

export function getAllTodo() {
  const query = `
  query getAllTodo {
    allTodos {
      id
      title
      description
    }
  }`;
  const payload = {"query": query};

  return fetch(API_ENDPOINT, {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify(payload)
  })
  .then(response => response.json())
  .then(data => data.data.allTodos)
  .catch(error => {console.error(error); return [];});
}

export async function addTodo(title: string, description: string) {
  const query = `
  mutation AddTodo {
    postTodo(title: "${title}", description: "${description}")
  }`;

  const payload = {"query": query};
  const response = await fetch(API_ENDPOINT, {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify(payload)
  });
  return response.json();
}

