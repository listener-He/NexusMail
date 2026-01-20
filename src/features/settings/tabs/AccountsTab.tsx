import { useState, useEffect } from 'react';
import { Plus, Trash2, Edit } from 'lucide-react';
import { invoke } from '@tauri-apps/api/core';
import { useTranslation } from 'react-i18next';

interface Account {
    id: string;
    email: string;
    provider: string;
}

export const AccountsTab = () => {
    const { t } = useTranslation();
    const [accounts, setAccounts] = useState<Account[]>([]);
    const [isAdding, setIsAdding] = useState(false);
    
    // Form state
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [provider, setProvider] = useState('IMAP');
    const [server, setServer] = useState('');
    const [port, setPort] = useState('993');

    const fetchAccounts = async () => {
        try {
            if (typeof window !== 'undefined' && '__TAURI__' in window) {
                const res = await invoke<Account[]>('get_accounts');
                setAccounts(res);
            } else {
                setAccounts([{ id: '1', email: 'demo@example.com', provider: 'IMAP (Browser Mode)' }]);
            }
        } catch (err) {
            console.error("Failed to fetch accounts:", err);
        }
    };

    useEffect(() => {
        fetchAccounts();
    }, []);

    const handleAddAccount = async () => {
        try {
            const credentials = JSON.stringify({ server, port, password });
            if (typeof window !== 'undefined' && '__TAURI__' in window) {
                await invoke('create_account', { email, provider, credentials });
                setIsAdding(false);
                fetchAccounts();
                // Reset form
                setEmail('');
                setPassword('');
                setServer('');
            } else {
                alert(t('settings.accounts.browser_mode', "Cannot add account in browser mode."));
            }
        } catch (err) {
            console.error("Failed to add account:", err);
            alert(t('settings.accounts.add_error', "Failed to add account"));
        }
    };

    const handleDelete = async (id: string) => {
        if (!confirm(t('settings.accounts.delete_confirm', "Are you sure you want to delete this account?"))) return;
        try {
            if (typeof window !== 'undefined' && '__TAURI__' in window) {
                await invoke('delete_account', { id });
                fetchAccounts();
            }
        } catch (err) {
            console.error(err);
        }
    };

    return (
        <div className="space-y-6">
            <div className="flex justify-between items-center">
                <h3 className="text-lg font-medium text-white">{t('settings.accounts.title', "Connected Accounts")}</h3>
                <button 
                    onClick={() => setIsAdding(!isAdding)}
                    className="flex items-center gap-2 px-4 py-2 bg-lumina-active rounded-lg text-white text-sm"
                >
                    <Plus size={16} /> {isAdding ? t('common.cancel', 'Cancel') : t('settings.accounts.add', 'Add Account')}
                </button>
            </div>
            
            {isAdding && (
                <div className="p-4 rounded-xl bg-white/5 border border-white/10 space-y-4">
                    <h4 className="font-medium text-white">{t('settings.accounts.new_details', "New Account Details")}</h4>
                    <div className="grid gap-4">
                        <input 
                            type="email" placeholder={t('settings.accounts.email_placeholder', "Email Address")} 
                            className="bg-slate-900 border border-white/10 rounded p-2 text-white"
                            value={email} onChange={e => setEmail(e.target.value)}
                        />
                        <div className="grid grid-cols-2 gap-4">
                            <select 
                                className="bg-slate-900 border border-white/10 rounded p-2 text-white"
                                value={provider} onChange={e => setProvider(e.target.value)}
                            >
                                <option value="IMAP">IMAP</option>
                                <option value="Gmail">Gmail</option>
                                <option value="Outlook">Outlook</option>
                            </select>
                            <input 
                                type="password" placeholder={t('settings.accounts.password_placeholder', "Password / App Password")} 
                                className="bg-slate-900 border border-white/10 rounded p-2 text-white"
                                value={password} onChange={e => setPassword(e.target.value)}
                            />
                        </div>
                        {provider === 'IMAP' && (
                            <div className="grid grid-cols-3 gap-4">
                                <input 
                                    type="text" placeholder={t('settings.accounts.server_placeholder', "IMAP Server (e.g. imap.gmail.com)")} 
                                    className="col-span-2 bg-slate-900 border border-white/10 rounded p-2 text-white"
                                    value={server} onChange={e => setServer(e.target.value)}
                                />
                                <input 
                                    type="text" placeholder={t('settings.accounts.port_placeholder', "Port (993)")} 
                                    className="bg-slate-900 border border-white/10 rounded p-2 text-white"
                                    value={port} onChange={e => setPort(e.target.value)}
                                />
                            </div>
                        )}
                        <button 
                            onClick={handleAddAccount}
                            className="w-full py-2 bg-blue-600 hover:bg-blue-500 rounded text-white font-medium"
                        >
                            {t('settings.accounts.save', "Save Account")}
                        </button>
                    </div>
                </div>
            )}

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
                            <button 
                                onClick={() => handleDelete(account.id)}
                                className="p-2 hover:bg-red-500/10 hover:text-red-400 rounded-lg text-gray-400"
                            >
                                <Trash2 size={16} />
                            </button>
                        </div>
                    </div>
                ))}
                {accounts.length === 0 && (
                    <p className="text-gray-500 text-center py-8">{t('settings.accounts.no_accounts', "No accounts connected yet.")}</p>
                )}
            </div>
        </div>
    );
};
