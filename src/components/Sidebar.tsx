import React from 'react';
import { Inbox, Send, Archive, Settings, User } from 'lucide-react';
import { cn } from '../lib/utils';

interface NavItemProps {
  icon: React.ReactNode;
  active?: boolean;
}

const NavItem = ({ icon, active }: NavItemProps) => (
  <div className={cn(
    "p-3 rounded-xl transition-all duration-300 cursor-pointer mb-4",
    active 
      ? "bg-lumina-active text-white shadow-lg shadow-blue-500/30" 
      : "text-gray-400 hover:bg-white/10 hover:text-white"
  )}>
    {icon}
  </div>
);

export const Sidebar = () => {
  return (
    <div className="w-20 h-full bg-slate-900/50 backdrop-blur-xl border-r border-white/5 flex flex-col items-center py-8 z-50">
      <div className="mb-10">
        <div className="w-10 h-10 rounded-full bg-lumina-primary flex items-center justify-center font-bold text-white shadow-lg shadow-cyan-500/20">
          N
        </div>
      </div>
      
      <div className="flex-1 flex flex-col items-center w-full">
        <NavItem icon={<Inbox size={24} />} active />
        <NavItem icon={<Send size={24} />} />
        <NavItem icon={<Archive size={24} />} />
      </div>

      <div className="flex flex-col items-center w-full">
        <NavItem icon={<Settings size={24} />} />
        <div className="w-10 h-10 rounded-full bg-gradient-to-br from-gray-700 to-gray-900 border border-white/10 mt-4 flex items-center justify-center">
            <User size={20} className="text-gray-400" />
        </div>
      </div>
    </div>
  );
};
