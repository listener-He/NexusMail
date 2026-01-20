import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Plus, Play, Pause, Edit, Trash2 } from 'lucide-react';
import { WorkflowEditor } from './WorkflowEditor';
import { useTranslation } from 'react-i18next';

interface Workflow {
  id: string;
  name: string;
  enabled: boolean;
  triggers: any[];
  filters: any[];
  actions: any[];
}

export const WorkflowManager = () => {
  const { t } = useTranslation();
  const [workflows, setWorkflows] = useState<Workflow[]>([]);
  const [isEditing, setIsEditing] = useState(false);
  const [currentWorkflowId, setCurrentWorkflowId] = useState<string | null>(null);

  const fetchWorkflows = async () => {
      try {
          if (typeof window !== 'undefined' && '__TAURI__' in window) {
              const res = await invoke<Workflow[]>('get_workflows');
              setWorkflows(res);
          } else {
              // Mock data for browser
              setWorkflows([
                  { id: '1', name: 'Invoice Automation', enabled: true, triggers: [], filters: [], actions: [] },
                  { id: '2', name: 'Newsletter Archive', enabled: false, triggers: [], filters: [], actions: [] }
              ]);
          }
      } catch (err) {
          console.error(err);
      }
  };

  useEffect(() => {
    fetchWorkflows();
  }, [isEditing]);

  const handleCreate = () => {
      setCurrentWorkflowId(null);
      setIsEditing(true);
  };

  const handleEdit = (id: string) => {
      setCurrentWorkflowId(id);
      setIsEditing(true);
  };

  if (isEditing) {
      return (
          <div className="h-full flex flex-col">
              <button 
                onClick={() => setIsEditing(false)}
                className="self-start m-4 text-gray-400 hover:text-white"
              >
                  &larr; {t('workflow.back', "Back to Workflows")}
              </button>
              <div className="flex-1">
                 <WorkflowEditor workflowId={currentWorkflowId} onSave={() => setIsEditing(false)} />
              </div>
          </div>
      );
  }

  return (
    <div className="p-8 h-full bg-slate-950 text-white overflow-y-auto">
      <div className="flex justify-between items-center mb-8">
        <div>
            <h1 className="text-3xl font-bold bg-clip-text text-transparent bg-lumina-primary">
                {t('workflow.title')}
            </h1>
            <p className="text-gray-400 mt-2">{t('workflow.subtitle', "Manage your automation rules")}</p>
        </div>
        <button 
            onClick={handleCreate}
            className="flex items-center gap-2 px-4 py-2 bg-lumina-active rounded-lg hover:brightness-110 transition-all shadow-lg shadow-blue-500/20"
        >
            <Plus size={18} />
            {t('workflow.create')}
        </button>
      </div>

      <div className="grid gap-4">
          {workflows.map(wf => (
              <div key={wf.id} className="p-4 rounded-xl bg-white/5 border border-white/5 hover:border-white/10 transition-all flex justify-between items-center group">
                  <div className="flex items-center gap-4">
                      <div className={`w-10 h-10 rounded-lg flex items-center justify-center ${wf.enabled ? 'bg-green-500/10 text-green-400' : 'bg-gray-700/30 text-gray-500'}`}>
                          {wf.enabled ? <Play size={20} /> : <Pause size={20} />}
                      </div>
                      <div>
                          <h3 className="font-semibold text-lg">{wf.name}</h3>
                          <p className="text-sm text-gray-500">
                              {t('workflow.stats', { triggers: wf.triggers.length, actions: wf.actions.length, defaultValue: "{{triggers}} triggers · {{actions}} actions" })}
                          </p>
                      </div>
                  </div>
                  
                  <div className="flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
                      <button 
                        onClick={() => handleEdit(wf.id)}
                        className="p-2 hover:bg-white/10 rounded-lg text-gray-400 hover:text-white"
                      >
                          <Edit size={18} />
                      </button>
                      <button className="p-2 hover:bg-red-500/10 rounded-lg text-gray-400 hover:text-red-400">
                          <Trash2 size={18} />
                      </button>
                  </div>
              </div>
          ))}
      </div>
    </div>
  );
};
