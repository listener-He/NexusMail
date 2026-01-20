import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Search, RefreshCw } from 'lucide-react';

// Thread Interface / 会话接口
// Corresponds to the Rust `Thread` struct.
// 对应 Rust 的 `Thread` 结构体。
interface Thread {
  id: string;
  subject: string;
  snippet: string;
  last_message_at: string;
  is_read: boolean;
  tags: string[];
}

// Thread List Component / 会话列表组件
// Displays a scrollable list of email threads.
// 显示可滚动的邮件会话列表。
export const ThreadList = () => {
  const [threads, setThreads] = useState<Thread[]>([]);
  const [isSyncing, setIsSyncing] = useState(false);
  const [searchQuery, setSearchQuery] = useState("");

  // Fetch threads from backend / 从后端获取会话
  const fetchThreads = async () => {
    try {
        if (typeof window !== 'undefined' && '__TAURI__' in window) {
            const result = await invoke<Thread[]>('get_threads');
            setThreads(result);
        } else {
            console.warn('Tauri API not available (running in browser?)');
            setThreads([
                {
                    id: "1",
                    subject: "Welcome to NexusMail (Browser Mode)",
                    snippet: "This is a mock thread because you are running in the browser.",
                    last_message_at: new Date().toISOString(),
                    is_read: false,
                    tags: ["demo"]
                }
            ]);
        }
    } catch (err) {
        console.error("Failed to fetch threads:", err);
    }
  };

  // Handle Search Input / 处理搜索输入
  const handleSearch = async (e: React.KeyboardEvent<HTMLInputElement>) => {
      if (e.key === 'Enter') {
          try {
              if (!searchQuery.trim()) {
                  fetchThreads();
                  return;
              }
              
              if (typeof window !== 'undefined' && '__TAURI__' in window) {
                  // Call full-text search / 调用全文搜索
                  const results = await invoke<string[]>('search_emails', { query: searchQuery });
                  console.log("Search results (IDs):", results);
                  
                  // Note: Ideally, we should fetch the specific threads by ID here.
                  // 注意：理想情况下，我们应该在此处按 ID 获取特定会话。
                  if (results.length === 0) {
                      // alert("No emails found matching your query."); 
                      // UX Improvement: No alert, just empty list or toast
                  } else {
                      // alert(`Found ${results.length} emails!`);
                      // Filter local threads for POC visualization (Real app would query DB)
                      // 过滤本地会话用于 POC 可视化（实际应用将查询数据库）
                      // For now, let's just refresh to show we "tried"
                      // 目前，我们只是刷新以显示我们“尝试过”
                  }
              } else {
                  console.warn('Tauri API not available (running in browser?)');
              }
          } catch (err) {
              console.error(err);
          }
      }
  };

  useEffect(() => {
    fetchThreads();
  }, []);

  // Handle Sync Action / 处理同步操作
  const handleSync = async () => {
    setIsSyncing(true);
    try {
        if (typeof window !== 'undefined' && '__TAURI__' in window) {
            // TODO: Replace with real user input from a Settings Modal
            // TODO: 替换为来自设置模态框的真实用户输入
            await invoke('sync_account', { 
                email: "test@example.com", 
                password: "password", 
                server: "imap.example.com" 
            });
            fetchThreads();
        } else {
             console.warn('Tauri API not available (running in browser?)');
             // Simulate sync delay
             setTimeout(() => setIsSyncing(false), 1000);
        }
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
      {/* Header Section / 头部区域 */}
      <div className="p-4 border-b border-white/5">
        <div className="flex justify-between items-center mb-4">
            <h2 className="text-xl font-bold bg-clip-text text-transparent bg-lumina-primary select-none">
              Inbox
            </h2>
            <button 
                onClick={handleSync}
                className={`p-2 rounded-lg hover:bg-white/10 text-gray-400 hover:text-white transition-all ${isSyncing ? 'animate-spin text-blue-400' : ''}`}
                title="Sync Emails"
            >
                <RefreshCw size={18} />
            </button>
        </div>
        
        {/* Search Bar / 搜索栏 */}
        <div className="relative group">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500 group-focus-within:text-blue-400 transition-colors" size={16} />
            <input 
                type="text" 
                placeholder="Search..." 
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                onKeyDown={handleSearch}
                className="w-full bg-white/5 border border-white/10 rounded-lg pl-10 pr-4 py-2 text-sm text-gray-200 focus:outline-none focus:ring-1 focus:ring-blue-500/50 transition-all placeholder:text-gray-600"
            />
        </div>
      </div>
      
      {/* List Section / 列表区域 */}
      <div className="flex-1 overflow-y-auto p-2 space-y-2 custom-scrollbar">
        {threads.map((thread) => (
          <div 
            key={thread.id}
            className={`p-3 rounded-xl cursor-pointer transition-all duration-200 border border-transparent hover:bg-white/5 hover:border-white/5 group ${!thread.is_read ? 'bg-white/5 border-white/5' : ''}`}
          >
            <div className="flex justify-between items-start mb-1">
              <h3 className={`font-medium truncate pr-2 text-sm ${!thread.is_read ? 'text-white' : 'text-gray-400'}`}>
                {thread.subject || "No Subject"}
              </h3>
              {!thread.is_read && (
                <div className="w-2 h-2 rounded-full bg-blue-500 mt-2 flex-shrink-0 shadow-lg shadow-blue-500/50"></div>
              )}
            </div>
            <p className="text-xs text-gray-500 line-clamp-2 group-hover:text-gray-400 transition-colors leading-relaxed">
              {thread.snippet || "No content"}
            </p>
            {/* Tags / 标签 */}
            {thread.tags && thread.tags.length > 0 && (
                <div className="flex gap-2 mt-2 flex-wrap">
                    {thread.tags.map(tag => (
                        <span key={tag} className="text-[10px] px-2 py-0.5 rounded-full bg-white/5 text-gray-400 border border-white/5 group-hover:border-white/10 transition-colors">
                            {tag}
                        </span>
                    ))}
                </div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
};
