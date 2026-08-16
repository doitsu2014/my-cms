import { Route, Routes, Navigate } from 'react-router-dom';
import SiteLayout from './components/layout/SiteLayout';
import AboutPage from './pages/AboutPage';
import CategoriesPage from './pages/CategoriesPage';
import CategoryDetailPage from './pages/CategoryDetailPage';
import HomePage from './pages/HomePage';
import PostDetailPage from './pages/PostDetailPage';
import PublicMetadataBoundary from './metadata/PublicMetadataBoundary';

const AppContent = () => (
  <>
    <PublicMetadataBoundary />
    <SiteLayout>
      <Routes>
        <Route path="/" element={<Navigate to="/en" replace />} />
        <Route path="/:lang" element={<HomePage />} />
        <Route path="/:lang/categories" element={<CategoriesPage />} />
        <Route path="/:lang/categories/:slug" element={<CategoryDetailPage />} />
        <Route path="/:lang/posts/:slug" element={<PostDetailPage />} />
        <Route path="/:lang/about" element={<AboutPage />} />
        <Route path="*" element={<Navigate to="/en" replace />} />
      </Routes>
    </SiteLayout>
  </>
);

export default AppContent;
