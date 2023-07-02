# global css @root ff:Arial c:white/87 bg:black/85
# global css a c:indigo5 c@hover:indigo6
# global css body m:0 d:flex ja:center h:100vh

tag todo
	def load(url, data)
		let res = await window.fetch(url, {method: "POST", headers: {"Content-Type": "application/json"}, body: JSON.stringify(data)})
		await res.json!

	<self>
		css w:100% p:5 g:5 d:vbox

		let graphql_query = {
			operationName: "getAllTodoQuery"
			query: "query getAllTodoQuery \{ allTodos \{ id title description \} \}"
		}
		let resp = await load("http://localhost:5036/graphql", graphql_query)

		for todo in resp.data.allTodos
			<div>
				css min-width:30 p:2 bg:teal3 c:gray7 bxs:sm rd:md ta:center
				"{todo.title}: {todo.description}"

tag app
	<self>
		<h1> "Imba"
		<todo>

imba.mount <app>