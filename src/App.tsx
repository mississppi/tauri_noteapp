import {invoke} from '@tauri-apps/api/tauri';
import {useState, useEffect, useCallback} from 'react';
import List from './components/List';
import './App.css';

interface Post {
  id: number;
  title: string;
  content: string;
}

function App() {
  console.log("make App");
  return (
    <div className="App">
      <List></List>
    </div>
  );
}

export default App;
