import { useState } from 'react';
import { Sidebar } from './components/Sidebar';
import { ThreadList } from './components/ThreadList';
import { ChatView } from './components/ChatView';
import { WorkflowManager } from './features/workflow/WorkflowManager';
import { SettingsView } from './features/settings/SettingsView';
import { Inbox, GitFork, Settings, Send, Archive } from 'lucide-react';

const BottomNav = ({ currentView, onChangeView }: { currentView: string, onChangeView: (view: string) => void }) => (
    <div className="md:hidden fixed bottom-0 left-0 right-0 bg-slate-900/90 backdrop-blur-xl border-t border-white/5 flex justify-around p-4 z-50 pb-8">
        <button onClick={() => onChangeView('inbox')} className={currentView === 'inbox' ? 'text-blue-400' : 'text-gray-400'}>
            <Inbox size={24} />
        </button>
        <button onClick={() => onChangeView('sent')} className={currentView === 'sent' ? 'text-blue-400' : 'text-gray-400'}>
            <Send size={24} />
        </button>
        <button onClick={() => onChangeView('workflow')} className={currentView === 'workflow' ? 'text-blue-400' : 'text-gray-400'}>
            <GitFork size={24} />
        </button>
        <button onClick={() => onChangeView('archive')} className={currentView === 'archive' ? 'text-blue-400' : 'text-gray-400'}>
            <Archive size={24} />
        </button>
        <button onClick={() => onChangeView('settings')} className={currentView === 'settings' ? 'text-blue-400' : 'text-gray-400'}>
            <Settings size={24} />
        </button>
    </div>
);

function App() {
  const [currentView, setCurrentView] = useState('inbox');

  return (
    <div className="h-screen w-screen flex bg-lumina-focus text-white overflow-hidden font-sans selection:bg-blue-500/30">
      <div className="hidden md:flex h-full">
        <Sidebar currentView={currentView} onChangeView={setCurrentView} />
      </div>
      
      <div className="flex-1 flex flex-col h-full overflow-hidden relative">
          <div className="flex-1 flex overflow-hidden pb-20 md:pb-0">
            {currentView === 'inbox' && (
                <>
                <ThreadList />
                <div className="hidden lg:block flex-1">
                    <ChatView />
                </div>
                </>
            )}

            {currentView === 'workflow' && (
                <div className="flex-1 h-full">
                    <WorkflowManager />
                </div>
            )}

            {currentView === 'settings' && (
                <div className="flex-1 h-full">
                    <SettingsView />
                </div>
            )}

            {(currentView === 'sent' || currentView === 'archive') && (
                <div className="flex-1 flex items-center justify-center text-gray-500">
                    <p>Folder: {currentView} (Coming Soon)</p>
                </div>
            )}

            {currentView === 'profile' && (
                <div className="flex-1 flex items-center justify-center text-gray-500">
                    <p>Profile (Coming Soon)</p>
                </div>
            )}
          </div>
          <BottomNav currentView={currentView} onChangeView={setCurrentView} />
      </div>
    </div>
  );
}

export default App;
