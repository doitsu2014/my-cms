import { useState } from 'react';
import { Search, Sun, Moon, Menu, LogOut, LogIn } from 'lucide-react';
import { useAuth } from '@/auth/AuthContext';
import KeycloakDebugMenu from './keycloak-debug-menu';

export default function TopBar() {
  const [searchExpanded, setSearchExpanded] = useState(false);
  const { authenticated, logout, login } = useAuth();

  return (
    <>
      <div className="navbar bg-base-100 min-h-16 px-0">
        {/* Mobile Menu Button & Theme Toggle */}
        <div className="navbar-start">
          <label htmlFor="admin-drawer" className="btn btn-ghost btn-circle lg:hidden">
            <Menu className="w-6 h-6" />
          </label>
          <label className="swap swap-rotate btn btn-ghost btn-circle">
            <input type="checkbox" className="theme-controller" value="dark" />
            <Sun className="swap-off w-6 h-6" />
            <Moon className="swap-on w-6 h-6" />
          </label>
        </div>

        {/* Center Space */}
        <div className="navbar-center"></div>

        {/* Search & Auth */}
        <div className="navbar-end gap-2">
          {/* Search */}
          <div className="join">
            <input
              type="text"
              placeholder="Search..."
              className={`input input-bordered join-item transition-all duration-300 ${
                searchExpanded ? 'w-64' : 'w-0 px-0 border-0'
              }`}
              onFocus={() => setSearchExpanded(true)}
              onBlur={() => setSearchExpanded(false)}
            />
            <button
              className="btn btn-ghost btn-circle join-item"
              onClick={() => setSearchExpanded(!searchExpanded)}
            >
              <Search className="w-5 h-5" />
            </button>
          </div>

          {/* Keycloak Debug Menu - visible when authenticated */}
          {authenticated && <KeycloakDebugMenu />}

          {/* Auth Button */}
          {authenticated ? (
            <button className="btn btn-error btn-sm gap-1" onClick={logout}>
              <LogOut className="w-4 h-4" />
              <span className="hidden sm:inline">Logout</span>
            </button>
          ) : (
            <button className="btn btn-primary btn-sm gap-1" onClick={login}>
              <LogIn className="w-4 h-4" />
              <span className="hidden sm:inline">Login</span>
            </button>
          )}
        </div>
      </div>
      <div className="divider my-0"></div>
    </>
  );
}
