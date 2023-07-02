def load(url, data)
	let res = await window.fetch(url, {method: "POST", headers: {"Content-Type": "application/json"}, body: JSON.stringify(data)})
	await res.json!

tag Box
	css pos:relative d:flex ja:center
		rd:md bg:hue3 c:gray9/70
		w:40 h:40 m:1 px:4 fs:sm cursor:grab
		ff:sans
	x = 0
	y = 0
	<self[x:{x}px y:{y}px] @touch.moved.sync(self)> <slot> "box"

tag app
	<self[d:hflex ja:center]>
		let graphql_query = {
			operationName: "getAllTodoQuery"
			query: "query getAllTodoQuery \{ allTodos \{ id title description \} \}"
		}
		let resp = await load("http://localhost:5036/graphql", graphql_query)

		for todo in resp.data.allTodos
			<div>
				<Box[hue:blue]> "{todo.title}: {todo.description}"	

imba.mount do
	<app>