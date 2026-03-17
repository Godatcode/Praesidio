import { BrowserRouter, Routes, Route } from 'react-router-dom'
import { Layout } from './components/Layout'
import { Overview } from './pages/Overview'
import { Servers } from './pages/Servers'
import { Threats } from './pages/Threats'
import { Compliance } from './pages/Compliance'
import { Honeypot } from './pages/Honeypot'
import { AuditLog } from './pages/AuditLog'
import { Settings } from './pages/Settings'
import LandingPage from './pages/LandingPage'

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<LandingPage />} />
        <Route path="/app" element={<Layout />}>
          <Route index element={<Overview />} />
          <Route path="servers" element={<Servers />} />
          <Route path="threats" element={<Threats />} />
          <Route path="compliance" element={<Compliance />} />
          <Route path="honeypot" element={<Honeypot />} />
          <Route path="audit" element={<AuditLog />} />
          <Route path="settings" element={<Settings />} />
        </Route>
      </Routes>
    </BrowserRouter>
  )
}

export default App
