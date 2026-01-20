import { useTranslation } from 'react-i18next';

export const IntelligenceTab = () => {
    const { t } = useTranslation();

    return (
        <div className="space-y-6">
            <div className="space-y-2">
                <h3 className="text-lg font-medium text-white">{t('settings.intelligence.provider', 'LLM Provider')}</h3>
                <select className="w-full max-w-md bg-white/5 border border-white/10 rounded-lg p-2 text-white">
                    <option value="ollama">{t('settings.intelligence.providers.ollama', 'Ollama (Local)')}</option>
                    <option value="openai">{t('settings.intelligence.providers.openai', 'OpenAI')}</option>
                    <option value="anthropic">{t('settings.intelligence.providers.anthropic', 'Anthropic')}</option>
                </select>
            </div>
            
            <div className="space-y-2">
                <h3 className="text-lg font-medium text-white">{t('settings.intelligence.model', 'Model Name')}</h3>
                <input 
                    type="text" 
                    placeholder="llama3" 
                    className="w-full max-w-md bg-white/5 border border-white/10 rounded-lg p-2 text-white"
                />
            </div>

            <div className="space-y-2">
                <h3 className="text-lg font-medium text-white">{t('settings.intelligence.api_key', 'API Key')}</h3>
                <input 
                    type="password" 
                    placeholder="sk-..." 
                    className="w-full max-w-md bg-white/5 border border-white/10 rounded-lg p-2 text-white"
                />
                <p className="text-xs text-gray-500">{t('settings.intelligence.secure_storage', 'Stored securely in system keychain.')}</p>
            </div>
        </div>
    );
};
