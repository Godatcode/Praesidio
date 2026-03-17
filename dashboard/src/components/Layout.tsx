import { Outlet } from 'react-router-dom'
import { Sidebar } from './Sidebar'

export function Layout() {
  return (
    <>
      <div className="bg-depth" />
      <Sidebar />
      <main className="relative z-[1] min-h-screen" style={{ marginLeft: 240 }}>
        <div className="max-w-[1200px] mx-auto" style={{ padding: 48 }}>
          <Outlet />
        </div>
      </main>
    </>
  )
}
