import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useTranslation } from 'react-i18next';

export const SecurityTab = () => {
    const { t } = useTranslation();
    const [newKey, setNewKey] = useState('');
    const [status, setStatus] = useState('');

    const handleChangePassword = async () => {
        if (!newKey) return;
        setStatus(t('settings.security.processing', 'Processing... This may take a while.'));
        try {
            if (typeof window !== 'undefined' && '__TAURI__' in window) {
                await invoke('change_master_password', { newKey });
                setStatus(t('settings.security.success', 'Success! Database re-encrypted.'));
                setNewKey('');
            } else {
                setStatus(t('settings.security.simulated', 'Simulated success (Browser mode)'));
            }
        } catch (err) {
            console.error(err);
            setStatus(t('settings.security.error', 'Failed to change password.'));
        }
    };

    return (
        <div className="space-y-6">
            <div className="space-y-2">
                <h3 className="text-lg font-medium text-white">{t('settings.security.db_encryption', 'Database Encryption')}</h3>
                <p className="text-sm text-gray-400 mb-4">
                    {t('settings.security.desc', 'Your local database is encrypted with a master key. You can rotate this key periodically.')}
                </p>
                <div className="flex gap-2 max-w-md">
                    <input 
                        type="password" 
                        placeholder={t('settings.security.new_pw_placeholder', 'New Master Password')} 
                        value={newKey}
                        onChange={e => setNewKey(e.target.value)}
                        className="flex-1 bg-white/5 border border-white/10 rounded-lg p-2 text-white"
                    />
                    <button 
                        onClick={handleChangePassword}
                        disabled={!newKey}
                        className="px-4 py-2 bg-lumina-active rounded-lg text-white text-sm disabled:opacity-50"
                    >
                        {t('settings.security.change_key', 'Change Key')}
                    </button>
                </div>
                {status && <p className="text-sm text-blue-400">{status}</p>}
            </div>
        </div>
    );
};
