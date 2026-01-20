export const SecurityTab = () => {
    return (
        <div className="space-y-6">
            <div className="space-y-2">
                <h3 className="text-lg font-medium text-white">Database Encryption</h3>
                <p className="text-sm text-gray-400 mb-4">
                    Your local database is encrypted with a master key. You can rotate this key periodically.
                </p>
                <button className="px-4 py-2 bg-white/5 border border-white/10 rounded-lg text-white text-sm hover:bg-white/10">
                    Change Master Password
                </button>
            </div>
        </div>
    );
};
