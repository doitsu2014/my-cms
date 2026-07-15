import './App.css';
import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import { AuthProvider } from './auth/AuthContext';
import { ProtectedRoute } from './auth/ProtectedRoute';
import { AdminOnlyRoute } from './app/admin/components/admin-only-route';
import { ToastProvider } from './components/toast-provider';
import AdminLayout from './app/admin/layout';
import AdminDashboard from './app/admin/page';
import AdminCategoriesListPage from './app/admin/categories/page';
import AdminCreateCategoryPage from './app/admin/categories/create/page';
import AdminEditCategoryPage from './app/admin/categories/edit/page';
import AdminBlogsPage from './app/admin/blogs/page';
import AdminCreateBlogPage from './app/admin/blogs/create/page';
import AdminEditBlogPage from './app/admin/blogs/edit/page';
import AdminMediaPage from './app/admin/media/page';
import AdminBucketsPage from './app/admin/media/buckets/page';
import AdminUsersListPage from './app/admin/users/page';
import AdminCreateUserPage from './app/admin/users/create/page';
import AdminEditUserPage from './app/admin/users/edit/page';
import AdminLoginPage from './app/admin/login/page';

const App = () => {
  return (
    <ToastProvider>
      <BrowserRouter>
        <AuthProvider>
          <Routes>
            {/* All admin routes are protected */}
            <Route path="/admin" element={<ProtectedRoute><AdminLayout><AdminDashboard /></AdminLayout></ProtectedRoute>} />
            <Route path="/admin/categories" element={<ProtectedRoute><AdminLayout><AdminCategoriesListPage /></AdminLayout></ProtectedRoute>} />
            <Route path="/admin/categories/create" element={<ProtectedRoute><AdminLayout><AdminCreateCategoryPage /></AdminLayout></ProtectedRoute>} />
            <Route path="/admin/categories/edit/:id" element={<ProtectedRoute><AdminLayout><AdminEditCategoryPage /></AdminLayout></ProtectedRoute>} />
            <Route path="/admin/blogs" element={<ProtectedRoute><AdminLayout><AdminBlogsPage /></AdminLayout></ProtectedRoute>} />
            <Route path="/admin/blogs/create" element={<ProtectedRoute><AdminLayout><AdminCreateBlogPage /></AdminLayout></ProtectedRoute>} />
            <Route path="/admin/blogs/edit/:id" element={<ProtectedRoute><AdminLayout><AdminEditBlogPage /></AdminLayout></ProtectedRoute>} />
            <Route path="/admin/media" element={<ProtectedRoute><AdminLayout><AdminMediaPage /></AdminLayout></ProtectedRoute>} />
            <Route path="/admin/media/buckets" element={<ProtectedRoute><AdminLayout><AdminOnlyRoute><AdminBucketsPage /></AdminOnlyRoute></AdminLayout></ProtectedRoute>} />
            <Route path="/admin/users" element={<ProtectedRoute><AdminLayout><AdminOnlyRoute><AdminUsersListPage /></AdminOnlyRoute></AdminLayout></ProtectedRoute>} />
            <Route path="/admin/users/create" element={<ProtectedRoute><AdminLayout><AdminOnlyRoute><AdminCreateUserPage /></AdminOnlyRoute></AdminLayout></ProtectedRoute>} />
            <Route path="/admin/users/edit/:id" element={<ProtectedRoute><AdminLayout><AdminOnlyRoute><AdminEditUserPage /></AdminOnlyRoute></AdminLayout></ProtectedRoute>} />
            <Route path="/admin/login" element={<AdminLoginPage />} />
            <Route path="/" element={<Navigate to="/admin" replace />} />
          </Routes>
        </AuthProvider>
      </BrowserRouter>
    </ToastProvider>
  );
};

export default App;


