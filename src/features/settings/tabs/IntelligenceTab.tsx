export const IntelligenceTab = () => {
    return (
        <div className="space-y-6">
            <div className="space-y-2">
                <h3 className="text-lg font-medium text-white">LLM Provider</h3>
                <select className="w-full max-w-md bg-white/5 border border-white/10 rounded-lg p-2 text-white">
                    <option value="ollama">Ollama (Local)</option>
                    <option value="openai">OpenAI</option>
                    <option value="anthropic">Anthropic</option>
                </select>
            </div>
            
            <div className="space-y-2">
                <h3 className="text-lg font-medium text-white">Model Name</h3>
                <input 
                    type="text" 
                    placeholder="llama3" 
                    className="w-full max-w-md bg-white/5 border border-white/10 rounded-lg p-2 text-white"
                />
            </div>

            <div className="space-y-2">
                <h3 className="text-lg font-medium text-white">API Key</h3>
                <input 
                    type="password" 
                    placeholder="sk-..." 
                    className="w-full max-w-md bg-white/5 border border-white/10 rounded-lg p-2 text-white"
                />
                <p className="text-xs text-gray-500">Stored securely in system keychain.</p>
            </div>
        </div>
    );
};
