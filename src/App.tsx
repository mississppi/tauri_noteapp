import {invoke} from '@tauri-apps/api/tauri';
import {useState, useEffect, useCallback} from 'react';
import List from './components/List';
import Main from './components/Main';
import './App.css';

interface Post {
  id: number;
  title: string;
  content: string;
}

function App() {
  console.log("make App");
  const [posts, setPosts] = useState<[]>([]);


  //rustから記事一覧を取得
  async function get_posts(){
    return invoke('list');
  }

  useEffect(() => {
    console.log("App - useEffect");
    // get_posts().then(result => {
    //   if (typeof result === 'object'){
    //     const p: Post[] = result;
    //     setPosts(p);
    //   } 
    // });
  }, [])

  return (
    <div className="App">
      <List></List>
      <Main></Main>
    </div>
  );
}

export default App;
