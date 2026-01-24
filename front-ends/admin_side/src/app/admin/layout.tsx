import LeftMenu from './components/left-menu';
import { LayoutProvider } from './layoutContext';
import { MainLayout } from './layoutMain';
import './admin.css';

export default function AdminLayout({ children }: { children: React.ReactNode }) {
  return (
    <LayoutProvider>
      <div className="drawer lg:drawer-open">
        <input id="admin-drawer" type="checkbox" className="drawer-toggle" />
        <div className="drawer-content flex flex-col">
          {/* Main Content */}
          <MainLayout>{children}</MainLayout>
        </div>
        <div className="drawer-side">
          <label htmlFor="admin-drawer" aria-label="close sidebar" className="drawer-overlay"></label>
          {/* Left Sidebar - visible on desktop, drawer on mobile */}
          <LeftMenu />
        </div>
      </div>
    </LayoutProvider>
  );
}
