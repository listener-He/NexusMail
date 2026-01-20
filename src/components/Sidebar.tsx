import React from 'react';
import { Inbox, Send, Archive, Settings, User, GitFork } from 'lucide-react';
import { cn } from '../lib/utils';
import { useTranslation } from 'react-i18next';

interface NavItemProps {
  icon: React.ReactNode;
  label?: string;
  active?: boolean;
  onClick?: () => void;
}

const NavItem = ({ icon, label, active, onClick }: NavItemProps) => (
  <div 
    onClick={onClick}
    className={cn(
    "p-3 rounded-xl transition-all duration-300 cursor-pointer mb-4 flex items-center gap-2 group relative",
    active 
      ? "bg-lumina-active text-white shadow-lg shadow-blue-500/30" 
      : "text-gray-400 hover:bg-white/10 hover:text-white"
  )}
  title={label}
  >
    {icon}
    {/* Tooltip for collapsed sidebar, or expanded text if we decide to expand sidebar later */}
  </div>
);

interface SidebarProps {
    currentView: string;
    onChangeView: (view: string) => void;
}

export const Sidebar = ({ currentView, onChangeView }: SidebarProps) => {
  const { t } = useTranslation();

  return (
    <div className="w-20 h-full bg-slate-900/50 backdrop-blur-xl border-r border-white/5 flex flex-col items-center py-8 z-50">
      <div className="mb-10">
        <div className="w-10 h-10 rounded-full bg-lumina-primary flex items-center justify-center font-bold text-white shadow-lg shadow-cyan-500/20 select-none">
          N
        </div>
      </div>
      
      <div className="flex-1 flex flex-col items-center w-full">
        <NavItem 
            icon={<Inbox size={24} />} 
            label={t('sidebar.inbox')}
            active={currentView === 'inbox'} 
            onClick={() => onChangeView('inbox')}
        />
        <NavItem 
            icon={<Send size={24} />} 
            label={t('sidebar.sent')}
            active={currentView === 'sent'}
            onClick={() => onChangeView('sent')}
        />
        <NavItem 
            icon={<Archive size={24} />} 
            label={t('sidebar.archive')}
            active={currentView === 'archive'}
            onClick={() => onChangeView('archive')}
        />
        <NavItem 
            icon={<GitFork size={24} />} 
            label={t('sidebar.workflows')}
            active={currentView === 'workflow'} 
            onClick={() => onChangeView('workflow')}
        />
      </div>

      <div className="flex flex-col items-center w-full">
        <NavItem 
            icon={<Settings size={24} />} 
            label={t('sidebar.settings')}
            active={currentView === 'settings'}
            onClick={() => onChangeView('settings')}
        />
        <div 
            className="w-10 h-10 rounded-full bg-gradient-to-br from-gray-700 to-gray-900 border border-white/10 mt-4 flex items-center justify-center cursor-pointer hover:border-white/30 transition-all"
            onClick={() => onChangeView('profile')}
            title={t('sidebar.profile')}
        >
            <User size={20} className="text-gray-400" />
        </div>
      </div>
    </div>
  );
};
