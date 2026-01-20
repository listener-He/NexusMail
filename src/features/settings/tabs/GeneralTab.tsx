export const GeneralTab = () => {
    return (
        <div className="space-y-6">
            <div className="space-y-2">
                <h3 className="text-lg font-medium text-white">Appearance</h3>
                <div className="grid grid-cols-3 gap-4 max-w-md">
                    {['Light', 'Dark', 'System'].map(theme => (
                        <button key={theme} className="p-4 rounded-lg border border-white/10 bg-white/5 hover:bg-white/10 text-center">
                            {theme}
                        </button>
                    ))}
                </div>
            </div>
            <div className="space-y-2">
                <h3 className="text-lg font-medium text-white">Language</h3>
                <select className="w-full max-w-md bg-white/5 border border-white/10 rounded-lg p-2 text-white">
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
