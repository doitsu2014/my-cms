import { LayoutGrid, FileText, CheckCircle2, Info } from 'lucide-react';
import { Link } from 'react-router-dom';

export default function AdminDashboard() {
  return (
    <div className="container max-w-7xl mx-auto">
      <h1 className="text-3xl font-bold mb-6">Admin Dashboard</h1>

      {/* Stats Section - Enhanced with DaisyUI 5.x */}
      <div className="stats stats-vertical lg:stats-horizontal shadow w-full mb-8">
        <div className="stat">
          <div className="stat-figure text-primary">
            <LayoutGrid className="w-8 h-8" />
          </div>
          <div className="stat-title">Total Categories</div>
          <div className="stat-value text-primary">0</div>
          <div className="stat-desc">All blog categories</div>
        </div>

        <div className="stat">
          <div className="stat-figure text-secondary">
            <FileText className="w-8 h-8" />
          </div>
          <div className="stat-title">Total Blogs</div>
          <div className="stat-value text-secondary">0</div>
          <div className="stat-desc">All blog posts</div>
        </div>

        <div className="stat">
          <div className="stat-figure text-success">
            <CheckCircle2 className="w-8 h-8" />
          </div>
          <div className="stat-title">Published</div>
          <div className="stat-value text-success">0</div>
          <div className="stat-desc">Published posts</div>
        </div>
      </div>

      {/* Quick Actions - Enhanced Card Layout */}
      <div className="card bg-base-200 shadow-xl mb-8">
        <div className="card-body">
          <h2 className="card-title text-2xl mb-4">Quick Actions</h2>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <Link to="/admin/categories" className="btn btn-primary btn-lg">
              <LayoutGrid className="w-5 h-5" />
              Manage Categories
            </Link>
            <Link to="/admin/blogs" className="btn btn-secondary btn-lg">
              <FileText className="w-5 h-5" />
              Manage Blogs
            </Link>
          </div>
        </div>
      </div>

      {/* Welcome Alert - DaisyUI Alert Component */}
      <div role="alert" className="alert alert-info">
        <Info className="w-6 h-6" />
        <span>Welcome to the admin panel. Use the left menu to navigate to different sections.</span>
      </div>
    </div>
  );
}
