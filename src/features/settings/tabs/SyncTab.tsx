import { RefreshCw, Pause, Play } from 'lucide-react';

export const SyncTab = () => {
    return (
        <div className="space-y-6">
            <div className="flex justify-between items-center">
                <h3 className="text-lg font-medium text-white">Sync Status</h3>
                <div className="flex gap-2">
                    <button className="flex items-center gap-2 px-4 py-2 bg-white/5 border border-white/10 rounded-lg text-white text-sm hover:bg-white/10">
                        <Pause size={16} /> Pause
                    </button>
                    <button className="flex items-center gap-2 px-4 py-2 bg-lumina-active rounded-lg text-white text-sm">
                        <RefreshCw size={16} /> Sync Now
                    </button>
                </div>
            </div>

            <div className="p-6 rounded-xl bg-white/5 border border-white/10 space-y-4">
                <div className="flex justify-between text-sm text-gray-400">
                    <span>Syncing...</span>
                    <span>45%</span>
                </div>
                <div className="w-full h-2 bg-white/10 rounded-full overflow-hidden">
                    <div className="h-full bg-blue-500 w-[45%]"></div>
                </div>
                <p className="text-xs text-gray-500">Last synced: Just now</p>
            </div>

            <div className="space-y-2">
                <h3 className="text-lg font-medium text-white">Channels</h3>
                <div className="grid gap-4">
                    {['S3 Backup', 'WebDAV'].map(channel => (
                        <div key={channel} className="p-4 rounded-xl bg-white/5 border border-white/10 flex justify-between items-center">
                            <span className="text-white">{channel}</span>
                            <div className="flex items-center gap-2">
                                <span className="w-2 h-2 rounded-full bg-green-500"></span>
                                <span className="text-sm text-gray-400">Active</span>
                            </div>
                        </div>
                    ))}
                </div>
            </div>
        </div>
    );
};
