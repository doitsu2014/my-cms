import MenuItem from './menu-item';
import { UserCircle } from 'lucide-react';
import { useAuth } from '@/auth/AuthContext';

export default function LeftMenu() {
  const { userInfo } = useAuth();

  const userName = userInfo?.name || userInfo?.username || 'Admin User';
  const userEmail = userInfo?.email || 'admin@example.com';
  const userAvatar = userInfo?.picture;

  return (
    <aside className="bg-base-200 min-h-full w-64 p-4 flex flex-col">
      {/* User Profile Card */}
      <div className="card bg-base-100 shadow-md mb-4">
        <div className="card-body p-4">
          <div className="flex items-center gap-3">
            <div className="avatar">
              <div className="w-12 rounded-full">
                {userAvatar ? (
                  <img src={userAvatar} alt={userName} />
                ) : (
                  <div className="bg-neutral text-neutral-content w-full h-full flex items-center justify-center">
                    <UserCircle className="w-8 h-8" />
                  </div>
                )}
              </div>
            </div>
            <div className="flex-1 min-w-0">
              <h2 className="font-bold text-sm truncate">{userName}</h2>
              <p className="text-xs opacity-70 truncate">{userEmail}</p>
            </div>
          </div>
        </div>
      </div>

      {/* Navigation Menu */}
      <ul className="menu bg-base-200 rounded-box flex-1 w-full">
        <li>
          <MenuItem displayName="Dashboard" slug="/admin" />
        </li>
        <li>
          <details open>
            <summary className="font-semibold">Resources</summary>
            <ul>
              <li>
                <MenuItem displayName="Categories" slug="/admin/categories" />
              </li>
              <li>
                <MenuItem displayName="Blogs" slug="/admin/blogs" />
              </li>
              <li>
                <MenuItem displayName="Media" slug="/admin/media" />
              </li>
            </ul>
          </details>
        </li>
      </ul>
    </aside>
  );
}
