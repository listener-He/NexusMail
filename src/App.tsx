import React, { useState } from 'react';
import { Sidebar } from './components/Sidebar';
import { ThreadList } from './components/ThreadList';
import { ChatView } from './components/ChatView';
import { WorkflowEditor } from './features/workflow/WorkflowEditor';

function App() {
  const [currentView, setCurrentView] = useState('inbox');

  return (
    <div className="h-screen w-screen flex bg-lumina-focus text-white overflow-hidden font-sans selection:bg-blue-500/30">
      <Sidebar currentView={currentView} onChangeView={setCurrentView} />
      
      {currentView === 'inbox' && (
        <>
          <ThreadList />
          <ChatView />
        </>
      )}

      {currentView === 'workflow' && (
        <div className="flex-1 h-full">
            <WorkflowEditor />
        </div>
      )}
    </div>
  );
}

export default App;
