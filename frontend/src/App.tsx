import React, { useState } from 'react';
import { getAllTodo, addTodo } from './todo';

interface ItemProps {
  item: string;
}

const Item: React.FC<ItemProps> = ({ item }) => {
  return (
    <li>{item}</li>
  );
};

interface TodosProps {
  items: string[];
}

const Todos: React.FC<TodosProps> = ({ items }) => {
  return (
    <ul>
      {items.map((item: string) => (
        <Item item={item} />
      ))}
    </ul>
  );
};

const App: React.FC = () => {
  const [listItems, setListItems] = useState<string[]>([]);
  const [inputValue, setInputValue] = useState<string>('');

  getAllTodo().then(allTodos => {
    allTodos.map((item: Object) => {
      console.log(item.title);
      listItems.push(item.title);
    })
  });

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setInputValue(e.target.value);
  };

  const handleFormSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (inputValue.trim() !== '') {
      addTodo(inputValue, inputValue);
      setListItems([...listItems, inputValue]);
      setInputValue('');
    }
  };

  return (
    <div>
      <h1>AWESOME TITLE HERE!</h1>
      <form onSubmit={handleFormSubmit}>
        <input
          type="text"
          value={inputValue}
          onChange={handleInputChange}
        />
        <button type="submit">Add Item</button>
      </form>
      <Todos items={listItems} />
    </div>
  );
};

export default App;

