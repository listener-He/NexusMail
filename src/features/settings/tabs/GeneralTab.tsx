import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useTranslation } from 'react-i18next';

interface AppConfig {
    language: string;
    theme: string;
    llm: any;
    search: any;
    first_run: boolean;
}

export const GeneralTab = () => {
    const { i18n } = useTranslation();
    const [config, setConfig] = useState<AppConfig | null>(null);

    useEffect(() => {
        // Load config from backend
        if (typeof window !== 'undefined' && '__TAURI__' in window) {
            invoke<AppConfig>('get_config').then(cfg => {
                setConfig(cfg);
                // Apply loaded settings
                i18n.changeLanguage(cfg.language);
                applyTheme(cfg.theme);
            }).catch(console.error);
        } else {
             // Mock config for browser
             setConfig({
                 language: 'en',
                 theme: 'system',
                 llm: {},
                 search: {},
                 first_run: false
             });
        }
    }, [i18n]);

    const applyTheme = (theme: string) => {
        if (theme === 'dark') {
            document.documentElement.classList.add('dark');
        } else if (theme === 'light') {
            document.documentElement.classList.remove('dark');
        } else {
            // System preference logic could go here
             document.documentElement.classList.add('dark'); // Default to dark for Lumina
        }
    };

    const updateConfig = async (newConfig: Partial<AppConfig>) => {
        if (!config) return;
        const updated = { ...config, ...newConfig };
        setConfig(updated);
        
        // Apply changes immediately
        if (newConfig.language) i18n.changeLanguage(newConfig.language);
        if (newConfig.theme) applyTheme(newConfig.theme);

        if (typeof window !== 'undefined' && '__TAURI__' in window) {
            try {
                await invoke('save_config', { config: updated });
            } catch (err) {
                console.error("Failed to save config:", err);
            }
        }
    };

    if (!config) return <div className="text-gray-400">Loading settings...</div>;

    return (
        <div className="space-y-6">
            <div className="space-y-2">
                <h3 className="text-lg font-medium text-white">Appearance</h3>
                <div className="grid grid-cols-3 gap-4 max-w-md">
                    {['light', 'dark', 'system'].map(theme => (
                        <button 
                            key={theme} 
                            onClick={() => updateConfig({ theme })}
                            className={`p-4 rounded-lg border text-center capitalize transition-colors ${
                                config.theme === theme 
                                ? 'bg-blue-500/20 border-blue-500 text-white' 
                                : 'bg-white/5 border-white/10 hover:bg-white/10 text-gray-400'
                            }`}
                        >
                            {theme}
                        </button>
                    ))}
                </div>
            </div>
            <div className="space-y-2">
                <h3 className="text-lg font-medium text-white">Language</h3>
                <select 
                    value={config.language}
                    onChange={(e) => updateConfig({ language: e.target.value })}
                    className="w-full max-w-md bg-white/5 border border-white/10 rounded-lg p-2 text-white focus:border-blue-500 focus:outline-none"
                >
                    <option value="en">English</option>
                    <option value="zh-CN">简体中文</option>
                    <option value="zh-TW">繁體中文</option>
                    <option value="ja">日本語</option>
                    <option value="ko">한국어</option>
                </select>
            </div>
        </div>
    );
};
