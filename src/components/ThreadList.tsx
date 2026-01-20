import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Search, RefreshCw } from 'lucide-react';

interface Thread {
  id: string;
  subject: string;
  snippet: string;
  last_message_at: string;
  is_read: boolean;
  tags: string[];
}

export const ThreadList = () => {
  const [threads, setThreads] = useState<Thread[]>([]);
  const [isSyncing, setIsSyncing] = useState(false);

  const [searchQuery, setSearchQuery] = useState("");

  const fetchThreads = () => {
    invoke<Thread[]>('get_threads')
      .then(setThreads)
      .catch(console.error);
  };

  const handleSearch = async (e: React.KeyboardEvent<HTMLInputElement>) => {
      if (e.key === 'Enter') {
          try {
              if (!searchQuery.trim()) {
                  fetchThreads();
                  return;
              }
              const results = await invoke<string[]>('search_emails', { query: searchQuery });
              console.log("Search results (IDs):", results);
              // For now, since we only get IDs back from Sonic, we would need to fetch the actual threads by ID.
              // For this POC, we'll just log them. In a real app, we'd have a `get_threads_by_ids` command.
              // To demonstrate UI feedback:
              if (results.length === 0) {
                  alert("No emails found matching your query.");
              } else {
                  alert(`Found ${results.length} emails! (IDs logged to console)`);
              }
          } catch (err) {
              console.error(err);
          }
      }
  };

  useEffect(() => {
    fetchThreads();
  }, []);

  const handleSync = async () => {
    setIsSyncing(true);
    try {
        // Hardcoded credentials for POC
        await invoke('sync_account', { 
            email: "test@example.com", 
            password: "password", 
            server: "imap.example.com" 
        });
        fetchThreads();
    } catch (error) {
        console.error("Sync failed (expected in POC without real creds):", error);
        // Refresh anyway to show if any data changed
        fetchThreads();
    } finally {
        setIsSyncing(false);
    }
  };

  return (
    <div className="w-80 h-full bg-slate-900/30 backdrop-blur-md border-r border-white/5 flex flex-col">
      <div className="p-4 border-b border-white/5">
        <div className="flex justify-between items-center mb-4">
            <h2 className="text-xl font-bold bg-clip-text text-transparent bg-lumina-primary">
              Inbox
            </h2>
            <button 
                onClick={handleSync}
                className={`p-2 rounded-lg hover:bg-white/10 text-gray-400 hover:text-white transition-all ${isSyncing ? 'animate-spin text-blue-400' : ''}`}
            >
                <RefreshCw size={18} />
            </button>
        </div>
        <div className="relative">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500" size={16} />
            <input 
                type="text" 
                placeholder="Search..." 
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                onKeyDown={handleSearch}
                className="w-full bg-white/5 border border-white/10 rounded-lg pl-10 pr-4 py-2 text-sm text-gray-200 focus:outline-none focus:ring-1 focus:ring-blue-500/50 transition-all"
            />
        </div>
      </div>
      
      <div className="flex-1 overflow-y-auto p-2 space-y-2">
        {threads.map((thread) => (
          <div 
            key={thread.id}
            className={`p-3 rounded-xl cursor-pointer transition-all duration-200 border border-transparent hover:bg-white/5 hover:border-white/5 group ${!thread.is_read ? 'bg-white/5' : ''}`}
          >
            <div className="flex justify-between items-start mb-1">
              <h3 className={`font-medium truncate pr-2 ${!thread.is_read ? 'text-white' : 'text-gray-400'}`}>
                {thread.subject}
              </h3>
              {!thread.is_read && (
                <div className="w-2 h-2 rounded-full bg-blue-500 mt-2"></div>
              )}
            </div>
            <p className="text-sm text-gray-500 line-clamp-2 group-hover:text-gray-400 transition-colors">
              {thread.snippet}
            </p>
            <div className="flex gap-2 mt-2">
                {thread.tags.map(tag => (
                    <span key={tag} className="text-xs px-2 py-0.5 rounded-full bg-white/5 text-gray-400 border border-white/5">
                        {tag}
                    </span>
                ))}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
