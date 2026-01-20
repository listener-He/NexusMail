import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { GeneralTab } from './tabs/GeneralTab';
import { AccountsTab } from './tabs/AccountsTab';
import { SyncTab } from './tabs/SyncTab';
import { IntelligenceTab } from './tabs/IntelligenceTab';
import { SecurityTab } from './tabs/SecurityTab';

export const SettingsView = () => {
    const { t } = useTranslation();
    const [activeTab, setActiveTab] = useState('general');

    const tabs = [
        { id: 'general', label: 'General' },
        { id: 'accounts', label: 'Accounts' },
        { id: 'sync', label: 'Backup & Sync' },
        { id: 'intelligence', label: 'Intelligence' },
        { id: 'security', label: 'Security' },
    ];

    return (
        <div className="h-full flex flex-col bg-slate-950 text-white">
            <div className="p-8 pb-0">
                <h1 className="text-3xl font-bold bg-clip-text text-transparent bg-lumina-primary mb-6">
                    {t('settings.title')}
                </h1>
                <div className="flex gap-4 border-b border-white/10 overflow-x-auto">
                    {tabs.map(tab => (
                        <button
                            key={tab.id}
                            onClick={() => setActiveTab(tab.id)}
                            className={`pb-3 px-2 text-sm font-medium transition-all whitespace-nowrap ${
                                activeTab === tab.id 
                                    ? 'text-blue-400 border-b-2 border-blue-400' 
                                    : 'text-gray-400 hover:text-white'
                            }`}
                        >
                            {tab.label}
                        </button>
                    ))}
                </div>
            </div>
            
            <div className="flex-1 p-8 overflow-y-auto">
                {activeTab === 'general' && <GeneralTab />}
                {activeTab === 'accounts' && <AccountsTab />}
                {activeTab === 'sync' && <SyncTab />}
                {activeTab === 'intelligence' && <IntelligenceTab />}
                {activeTab === 'security' && <SecurityTab />}
            </div>
        </div>
    );
};
