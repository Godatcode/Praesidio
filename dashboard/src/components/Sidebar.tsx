import { NavLink } from 'react-router-dom'
import {
  LayoutDashboard,
  Server,
  AlertTriangle,
  ShieldCheck,
  Crosshair,
  FileText,
  Settings,
} from 'lucide-react'

const navItems = [
  { to: '/app', label: 'Overview', icon: LayoutDashboard },
  { to: '/app/servers', label: 'Servers', icon: Server },
  { to: '/app/threats', label: 'Threats', icon: AlertTriangle },
  { to: '/app/compliance', label: 'Compliance', icon: ShieldCheck },
  { to: '/app/honeypot', label: 'Honeypot', icon: Crosshair },
  { to: '/app/audit', label: 'Audit Log', icon: FileText },
]

const bottomItems = [
  { to: '/app/settings', label: 'Settings', icon: Settings },
]

export function Sidebar() {
  return (
    <nav className="sidebar">
      <div className="sidebar-logo">
        <div className="logo-icon">
          <ShieldCheck size={14} color="#fff" />
        </div>
        Praesidio
      </div>
      <div className="sidebar-version">v0.2.0</div>

      <div className="sidebar-nav">
        {navItems.map(({ to, label, icon: Icon }) => (
          <NavLink
            key={to}
            to={to}
            end={to === '/app'}
            className={({ isActive }) =>
              `sidebar-item${isActive ? ' active' : ''}`
            }
          >
            <Icon size={16} />
            {label}
          </NavLink>
        ))}
      </div>

      <div className="sidebar-divider" />

      {bottomItems.map(({ to, label, icon: Icon }) => (
        <NavLink
          key={to}
          to={to}
          className={({ isActive }) =>
            `sidebar-item${isActive ? ' active' : ''}`
          }
        >
          <Icon size={16} />
          {label}
        </NavLink>
      ))}

      <div className="sidebar-status">
        <span className="status-dot" />
        Proxy: Active
      </div>
    </nav>
  )
}
