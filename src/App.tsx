import React from 'react';
import { Sidebar } from './components/Sidebar';
import { ThreadList } from './components/ThreadList';
import { ChatView } from './components/ChatView';

function App() {
  return (
    <div className="h-screen w-screen flex bg-lumina-focus text-white overflow-hidden font-sans selection:bg-blue-500/30">
      <Sidebar />
      <ThreadList />
      <ChatView />
    </div>
  );
}

export default App;
