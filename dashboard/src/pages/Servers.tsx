import { useCallback } from 'react'
import { useApi } from '../hooks/useApi'
import { getServers, triggerScan } from '../api/client'
import { ServerRow } from '../components/ServerRow'
import { useReveal } from '../hooks/useReveal'

export function Servers() {
  const { data: servers, refetch } = useApi(getServers)
  const revealHeader = useReveal()
  const revealList = useReveal()

  const handleScan = useCallback(async (_name: string) => {
    await triggerScan()
    refetch()
  }, [refetch])

  return (
    <div>
      <div ref={revealHeader} className="reveal" style={{ marginBottom: 48 }}>
        <h1 className="page-title">Servers</h1>
        <p className="page-subtitle">
          {servers?.length ?? 0} MCP servers discovered across {new Set(servers?.map(s => s.config_source)).size} configurations
        </p>
      </div>

      <div ref={revealList} className="reveal" style={{ display: 'flex', flexDirection: 'column', gap: 12 }}>
        {servers?.map((server) => (
          <ServerRow key={server.name} server={server} onScan={handleScan} />
        ))}

        {!servers && (
          <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', height: 128 }}>
            <div style={{
              width: 20, height: 20,
              border: '2px solid rgba(139,92,246,0.4)',
              borderTopColor: 'transparent',
              borderRadius: '50%',
              animation: 'spin 1s linear infinite',
            }} />
            <style>{`@keyframes spin { to { transform: rotate(360deg); } }`}</style>
          </div>
        )}
      </div>
    </div>
  )
}
