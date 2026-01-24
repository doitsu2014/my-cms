import './App.css';
import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import { AuthProvider } from './auth/AuthContext';
import { ProtectedRoute } from './auth/ProtectedRoute';
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

const App = () => {
  return (
    <ToastProvider>
      <AuthProvider>
        <BrowserRouter>
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
          <Route path="/" element={<Navigate to="/admin" replace />} />
        </Routes>
        </BrowserRouter>
      </AuthProvider>
    </ToastProvider>
  );
};

export default App;


