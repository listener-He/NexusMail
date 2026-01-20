import { useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import {
  ReactFlow,
  MiniMap,
  Controls,
  Background,
  useNodesState,
  useEdgesState,
  addEdge,
  type Connection,
} from '@xyflow/react';
import '@xyflow/react/dist/style.css';
import { useTranslation } from 'react-i18next';

const initialNodes = [
  { id: '1', position: { x: 250, y: 100 }, data: { label: 'Trigger: New Email' }, style: { background: '#1F2937', color: 'white', border: '1px solid #3B82F6' } },
  { id: '2', position: { x: 250, y: 250 }, data: { label: 'Filter: Contains "Invoice"' }, style: { background: '#1F2937', color: 'white', border: '1px solid #6EE7B7' } },
  { id: '3', position: { x: 250, y: 400 }, data: { label: 'Action: Upload to S3' }, style: { background: '#1F2937', color: 'white', border: '1px solid #F59E0B' } },
];

const initialEdges = [
    { id: 'e1-2', source: '1', target: '2', animated: true, style: { stroke: '#3B82F6' } },
    { id: 'e2-3', source: '2', target: '3', animated: true, style: { stroke: '#6EE7B7' } },
];

export const WorkflowEditor = ({ workflowId, onSave }: { workflowId?: string | null, onSave?: () => void }) => {
  const { t } = useTranslation();
  const [nodes, , onNodesChange] = useNodesState(initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

  const onConnect = useCallback(
    (params: Connection) => setEdges((eds) => addEdge(params, eds)),
    [setEdges],
  );

  const handleSave = async () => {
      // Convert React Flow nodes to Nexus Engine YAML format
      // Simplified: We assume a linear flow Trigger -> Filter -> Action for this POC
      const yaml = `
id: ${workflowId || crypto.randomUUID()}
name: "Visual Workflow"
enabled: true
triggers:
  - id: "t1"
    type_: OnNewEmail
filters:
  - id: "f1"
    field: "subject"
    operator: "contains"
    value: "Invoice"
actions:
  - id: "a1"
    type_: ArchiveEmail
    config: "{}"
`;
      try {
          if (typeof window !== 'undefined' && '__TAURI__' in window) {
              await invoke('save_workflow', { yaml });
              alert(t('workflow.save_success', "Workflow saved successfully!"));
          } else {
              console.warn('Tauri API not available (running in browser?)');
              alert(t('workflow.save_simulated', "Workflow saved (simulated)!"));
          }
          if (onSave) onSave();
      } catch (err) {
          console.error(err);
          alert(t('workflow.save_error', "Failed to save workflow."));
      }
  };

  return (
    <div className="w-full h-full bg-slate-950">
        <div className="p-4 border-b border-white/5 bg-slate-900/50 backdrop-blur-xl flex justify-between items-center z-10 relative">
            <div>
                <h2 className="text-xl font-bold text-white">{t('workflow.editor.title', "Invoice Automation")}</h2>
                <p className="text-sm text-gray-400">{t('workflow.editor.desc', 'Runs when new email contains "Invoice"')}</p>
            </div>
            <div className="flex gap-2">
                <button 
                    onClick={handleSave}
                    className="px-4 py-2 bg-white/5 hover:bg-white/10 text-white rounded-lg transition-colors border border-white/10"
                >
                    {t('workflow.save', "Save")}
                </button>
                <button className="px-4 py-2 bg-lumina-active text-white rounded-lg shadow-lg shadow-blue-500/20">
                    {t('workflow.activate', "Activate")}
                </button>
            </div>
        </div>
        <div className="h-[calc(100%-80px)]">
            <ReactFlow
                nodes={nodes}
                edges={edges}
                onNodesChange={onNodesChange}
                onEdgesChange={onEdgesChange}
                onConnect={onConnect}
                fitView
                className="bg-slate-950"
            >
                <Controls className="bg-white/10 border-white/10 fill-white" />
                <MiniMap className="bg-slate-900 border-white/10" nodeColor="#3B82F6" />
                <Background color="#333" gap={16} />
            </ReactFlow>
        </div>
    </div>
  );
};
