import { Paperclip, Send as SendIcon, MoreHorizontal } from 'lucide-react';

export const ChatView = () => {
  return (
    <div className="flex-1 h-full flex flex-col bg-slate-950 relative overflow-hidden">
        {/* Background Gradient Blob */}
        <div className="absolute top-0 right-0 w-[500px] h-[500px] bg-blue-500/10 rounded-full blur-[100px] pointer-events-none" />
        
        {/* Header */}
        <div className="h-16 border-b border-white/5 flex items-center justify-between px-6 backdrop-blur-sm z-10">
            <div>
                <h2 className="text-lg font-semibold text-white">Invoice #1023</h2>
                <p className="text-sm text-gray-500">Finance Dept • Oct 24, 10:30 AM</p>
            </div>
            <button className="p-2 hover:bg-white/5 rounded-lg text-gray-400 hover:text-white transition-colors">
                <MoreHorizontal size={20} />
            </button>
        </div>

        {/* Chat Area (Bubble Stream) */}
        <div className="flex-1 overflow-y-auto p-6 space-y-6 z-10">
            {/* Incoming Message */}
            <div className="flex gap-4 max-w-3xl">
                <div className="w-10 h-10 rounded-full bg-gradient-to-br from-purple-500 to-indigo-500 flex-shrink-0" />
                <div className="space-y-2">
                    <div className="bg-white/10 p-4 rounded-2xl rounded-tl-none backdrop-blur-sm border border-white/5">
                        <p className="text-gray-200 leading-relaxed">
                            Hi there, please find attached the invoice for the last month services. Let me know if you have any questions.
                        </p>
                    </div>
                    {/* Attachment Card */}
                    <div className="flex items-center gap-3 p-3 rounded-xl bg-white/5 border border-white/5 w-64 hover:bg-white/10 transition-colors cursor-pointer group">
                        <div className="p-2 rounded-lg bg-red-500/20 text-red-400">
                            <Paperclip size={18} />
                        </div>
                        <div className="flex-1 min-w-0">
                            <p className="text-sm font-medium text-gray-300 truncate group-hover:text-white">invoice_oct.pdf</p>
                            <p className="text-xs text-gray-500">2.4 MB</p>
                        </div>
                    </div>
                </div>
            </div>

            {/* Outgoing Message */}
            <div className="flex gap-4 max-w-3xl ml-auto flex-row-reverse">
                <div className="w-10 h-10 rounded-full bg-lumina-primary flex-shrink-0" />
                <div className="bg-lumina-active p-4 rounded-2xl rounded-tr-none shadow-lg shadow-blue-500/10">
                    <p className="text-white leading-relaxed">
                        Received, thanks! I'll process this right away.
                    </p>
                </div>
            </div>
        </div>

        {/* Input Area */}
        <div className="p-6 pt-0 z-10">
            <div className="bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-2 flex items-end gap-2 shadow-2xl">
                <button className="p-3 text-gray-400 hover:text-white hover:bg-white/10 rounded-xl transition-colors">
                    <Paperclip size={20} />
                </button>
                <textarea 
                    placeholder="Reply..." 
                    className="flex-1 bg-transparent border-none focus:ring-0 text-white placeholder-gray-500 resize-none py-3 max-h-32"
                    rows={1}
                />
                <button className="p-3 bg-lumina-active text-white rounded-xl shadow-lg shadow-blue-500/20 hover:opacity-90 transition-opacity">
                    <SendIcon size={20} />
                </button>
            </div>
        </div>
    </div>
  );
};
