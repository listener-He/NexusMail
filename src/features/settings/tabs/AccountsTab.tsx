import { useState } from 'react';
import { Plus, Trash2, Edit } from 'lucide-react';

export const AccountsTab = () => {
    const [accounts] = useState([
        { id: '1', email: 'test@example.com', provider: 'IMAP' },
    ]);

    return (
        <div className="space-y-6">
            <div className="flex justify-between items-center">
                <h3 className="text-lg font-medium text-white">Connected Accounts</h3>
                <button className="flex items-center gap-2 px-4 py-2 bg-lumina-active rounded-lg text-white text-sm">
                    <Plus size={16} /> Add Account
                </button>
            </div>
            
            <div className="space-y-4">
                {accounts.map(account => (
                    <div key={account.id} className="p-4 rounded-xl bg-white/5 border border-white/10 flex justify-between items-center">
                        <div>
                            <p className="font-medium text-white">{account.email}</p>
                            <p className="text-sm text-gray-500">{account.provider}</p>
                        </div>
                        <div className="flex gap-2">
                            <button className="p-2 hover:bg-white/10 rounded-lg text-gray-400">
                                <Edit size={16} />
                            </button>
                            <button className="p-2 hover:bg-red-500/10 hover:text-red-400 rounded-lg text-gray-400">
                                <Trash2 size={16} />
                            </button>
                        </div>
                    </div>
                ))}
            </div>
        </div>
    );
};
