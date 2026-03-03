import { useState } from "react";

import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function loadTree() {
    const tree = await invoke("list_tree");
    console.log(tree);
  }

  return (
    <main className="container">
      
    </main>
  );
}

export default App;
