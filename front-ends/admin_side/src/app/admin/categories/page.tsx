import { useEffect, useState, useMemo } from 'react';
import { toast } from 'sonner';
import {
  ChevronDown,
  ChevronUp,
  Home,
  Pencil,
  Trash2,
  Plus,
  Search,
  X,
  FolderOpen,
} from 'lucide-react';
import { Link } from 'react-router-dom';
import Breadcrumbs from '../components/my-breadcrumbs';
import TableSkeleton from '../components/skeleton/table-skeleton';
import type { TagModel } from '@/domains/tag';
import type { CategoryModel } from '@/domains/category';
import { getApiUrl, authenticatedFetch } from '@/config/api.config';
import { useAuth } from '@/auth/AuthContext';

export default function AdminCategoriesListPage() {
  const { token, keycloak } = useAuth();
  const [categories, setCategories] = useState<CategoryModel[]>([]);
  const [pageLoading, setPageLoading] = useState(true);
  const [sortConfig, setSortConfig] = useState<{ key: string; direction: 'asc' | 'desc' } | null>(
    null
  );
  const [textFilter, setTextFilter] = useState('');
  const [typeFilter, setTypeFilter] = useState('');
  const [categoryToDelete, setCategoryToDelete] = useState<CategoryModel | null>(null);
  const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false);
  const [isDeleting, setIsDeleting] = useState(false);
  const [currentPage, setCurrentPage] = useState(1);
  const itemsPerPage = 10;

  useEffect(() => {
    const loadCategories = async () => {
      try {
        setPageLoading(true);
        const response = await authenticatedFetch(
          getApiUrl('/categories'),
          token,
          { cache: 'no-store' },
          keycloak || undefined,
        );
        const res = await response.json();
        // Ensure we always set an array
        setCategories(Array.isArray(res.data) ? res.data : []);
      } catch (error) {
        console.error('Failed to load categories:', error);
        setCategories([]);
      } finally {
        setPageLoading(false);
      }
    };

    loadCategories();
  }, [token, keycloak]);

  const handleDeleteClick = (category: CategoryModel) => {
    setCategoryToDelete(category);
    setIsDeleteModalOpen(true);
  };

  const confirmDelete = async () => {
    if (!categoryToDelete) return;

    try {
      setIsDeleting(true);
      const response = await authenticatedFetch(
        getApiUrl(`/admin/categories/${categoryToDelete.id}`),
        token,
        { method: 'DELETE' },
        keycloak || undefined,
      );

      if (response.ok) {
        setCategories(categories.filter(c => c.id !== categoryToDelete.id));
        setIsDeleteModalOpen(false);
        setCategoryToDelete(null);
        toast.success('Category deleted successfully');
      } else {
        console.error('Failed to delete category');
        toast.error('Failed to delete category');
      }
    } catch (error) {
      console.error('Error deleting category:', error);
      toast.error('An error occurred while deleting the category');
    } finally {
      setIsDeleting(false);
    }
  };

  const cancelDelete = () => {
    setIsDeleteModalOpen(false);
    setCategoryToDelete(null);
  };

  const clearFilters = () => {
    setTextFilter('');
    setTypeFilter('');
    setCurrentPage(1);
  };

  const hasActiveFilters = textFilter || typeFilter;

  // Filtering and sorting with useMemo for performance
  const filteredAndSortedCategories = useMemo(() => {
    if (!Array.isArray(categories)) return [];
    let result = [...categories];

    // Apply text filter
    if (textFilter) {
      result = result.filter(
        (c) =>
          c.slug.toLowerCase().includes(textFilter.toLowerCase()) ||
          c.displayName.toLowerCase().includes(textFilter.toLowerCase())
      );
    }

    // Apply type filter
    if (typeFilter) {
      result = result.filter((c) => c.categoryType === typeFilter);
    }

    // Apply sorting
    if (sortConfig) {
      result.sort((a: any, b: any) => {
        if (a[sortConfig.key] < b[sortConfig.key]) return sortConfig.direction === 'asc' ? -1 : 1;
        if (a[sortConfig.key] > b[sortConfig.key]) return sortConfig.direction === 'asc' ? 1 : -1;
        return 0;
      });
    }

    return result;
  }, [categories, textFilter, typeFilter, sortConfig]);

  // Pagination
  const totalPages = Math.ceil(filteredAndSortedCategories.length / itemsPerPage);
  const paginatedCategories = useMemo(() => {
    const startIndex = (currentPage - 1) * itemsPerPage;
    return filteredAndSortedCategories.slice(startIndex, startIndex + itemsPerPage);
  }, [filteredAndSortedCategories, currentPage, itemsPerPage]);

  // Reset to page 1 when filters change
  useEffect(() => {
    setCurrentPage(1);
  }, [textFilter, typeFilter]);

  // Sorting
  const sortBy = (key: string) => {
    let direction: 'asc' | 'desc' = 'asc';
    if (sortConfig?.key === key && sortConfig.direction === 'asc') {
      direction = 'desc';
    }
    setSortConfig({ key, direction });
  };

  const SortIcon = ({ columnKey }: { columnKey: string }) => {
    if (sortConfig?.key !== columnKey) return null;
    return sortConfig.direction === 'asc' ? (
      <ChevronUp className="inline w-4 h-4 ml-1" />
    ) : (
      <ChevronDown className="inline w-4 h-4 ml-1" />
    );
  };

  return (
    <div className="space-y-6">
      <Breadcrumbs
        items={[
          { label: 'Admin', href: '/admin', icon: <Home className="w-4 h-4" /> },
          { label: 'Categories' },
        ]}
      />

      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold">Categories</h1>
          <p className="text-base-content/60 text-sm">
            Manage your blog categories and organization
          </p>
        </div>
        <Link to="/admin/categories/create" className="btn btn-primary gap-2">
          <Plus className="w-4 h-4" />
          New Category
        </Link>
      </div>

      {/* Collapsible Filter */}
      <details className="group bg-base-100 shadow-sm rounded-lg">
        <summary className="flex items-center justify-between cursor-pointer list-none p-4 font-medium">
          <div className="flex items-center gap-2">
            <span>Filters</span>
            {hasActiveFilters && (
              <span className="badge badge-primary badge-sm">
                {filteredAndSortedCategories.length} / {categories.length}
              </span>
            )}
          </div>
          <ChevronDown className="w-4 h-4 transition-transform group-open:rotate-180" />
        </summary>
        <div className="px-4 pb-4">
          <div className="flex flex-col sm:flex-row gap-3">
            <label className="input input-bordered flex items-center gap-2 flex-1">
              <Search className="w-4 h-4 opacity-50" />
              <input
                type="text"
                placeholder="Search by name or slug..."
                className="grow"
                value={textFilter}
                onChange={(e) => setTextFilter(e.target.value)}
              />
              {textFilter && (
                <button
                  type="button"
                  className="btn btn-ghost btn-xs btn-circle"
                  onClick={() => setTextFilter('')}
                >
                  <X className="w-3 h-3" />
                </button>
              )}
            </label>

            <div className="flex gap-2">
              <select
                className="select select-bordered min-w-[140px]"
                value={typeFilter}
                onChange={(e) => setTypeFilter(e.target.value)}
              >
                <option value="">All Types</option>
                <option value="Blog">Blog</option>
                <option value="Other">Other</option>
              </select>

              {hasActiveFilters && (
                <button
                  className="btn btn-outline btn-sm gap-1"
                  onClick={clearFilters}
                  title="Clear all filters"
                >
                  <X className="w-4 h-4" />
                  Clear
                </button>
              )}
            </div>
          </div>
        </div>
      </details>

      {/* Categories Table */}
      <div className="card bg-base-100 shadow-sm">
        <div className="overflow-x-auto">
          {pageLoading ? (
            <div className="p-4">
              <TableSkeleton rows={5} columns={6} showHeader={true} className="w-full" />
            </div>
          ) : paginatedCategories.length === 0 ? (
            /* Empty State */
            <div className="flex flex-col items-center justify-center py-16 px-4">
              <div className="bg-base-200 rounded-full p-4 mb-4">
                <FolderOpen className="w-8 h-8 text-base-content/40" />
              </div>
              <h3 className="text-lg font-semibold mb-1">
                {hasActiveFilters ? 'No categories found' : 'No categories yet'}
              </h3>
              <p className="text-base-content/60 text-center mb-4">
                {hasActiveFilters
                  ? 'Try adjusting your search or filter criteria'
                  : 'Get started by creating your first category'}
              </p>
              {hasActiveFilters ? (
                <button className="btn btn-ghost btn-sm" onClick={clearFilters}>
                  Clear filters
                </button>
              ) : (
                <Link to="/admin/categories/create" className="btn btn-primary btn-sm gap-2">
                  <Plus className="w-4 h-4" />
                  Create Category
                </Link>
              )}
            </div>
          ) : (
            <table className="table">
              <thead>
                <tr className="bg-base-200/50">
                  <th
                    className="cursor-pointer hover:bg-base-200 transition-colors"
                    onClick={() => sortBy('displayName')}
                  >
                    Name
                    <SortIcon columnKey="displayName" />
                  </th>
                  <th
                    className="cursor-pointer hover:bg-base-200 transition-colors"
                    onClick={() => sortBy('slug')}
                  >
                    Slug
                    <SortIcon columnKey="slug" />
                  </th>
                  <th
                    className="cursor-pointer hover:bg-base-200 transition-colors"
                    onClick={() => sortBy('categoryType')}
                  >
                    Type
                    <SortIcon columnKey="categoryType" />
                  </th>
                  <th>Created By</th>
                  <th>Tags</th>
                  <th className="text-center w-28">Actions</th>
                </tr>
              </thead>
              <tbody>
                {paginatedCategories.map((category) => (
                  <tr key={category.id} className="hover:bg-base-200/30 transition-colors">
                    <td className="font-medium">{category.displayName}</td>
                    <td>
                      <code className="text-xs bg-base-200 px-2 py-1 rounded">{category.slug}</code>
                    </td>
                    <td>
                      <span
                        className={`badge badge-sm ${
                          category.categoryType === 'Blog' ? 'badge-primary' : 'badge-secondary'
                        }`}
                      >
                        {category.categoryType}
                      </span>
                    </td>
                    <td className="text-base-content/70 text-sm">{category.createdBy || '-'}</td>
                    <td>
                      {category.tags && category.tags.length > 0 ? (
                        <div className="flex flex-wrap gap-1">
                          {category.tags.slice(0, 3).map((tag: TagModel) => (
                            <span key={tag.id} className="badge badge-ghost badge-sm">
                              {tag.name}
                            </span>
                          ))}
                          {category.tags.length > 3 && (
                            <span className="badge badge-ghost badge-sm">
                              +{category.tags.length - 3}
                            </span>
                          )}
                        </div>
                      ) : (
                        <span className="text-base-content/40 text-sm">No tags</span>
                      )}
                    </td>
                    <td>
                      <div className="flex justify-center gap-1">
                        <Link
                          to={`/admin/categories/edit/${category.id}`}
                          className="btn btn-ghost btn-sm btn-square"
                          title="Edit category"
                        >
                          <Pencil className="w-4 h-4" />
                        </Link>
                        <button
                          className="btn btn-ghost btn-sm btn-square text-error hover:bg-error/10"
                          onClick={() => handleDeleteClick(category)}
                          title="Delete category"
                        >
                          <Trash2 className="w-4 h-4" />
                        </button>
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>

        {/* Pagination */}
        {!pageLoading && totalPages > 1 && (
          <div className="card-body border-t border-base-200 py-3">
            <div className="flex flex-col sm:flex-row items-center justify-between gap-3">
              <span className="text-sm text-base-content/60">
                Showing {(currentPage - 1) * itemsPerPage + 1} to{' '}
                {Math.min(currentPage * itemsPerPage, filteredAndSortedCategories.length)} of{' '}
                {filteredAndSortedCategories.length} categories
              </span>
              <div className="join">
                <button
                  className="join-item btn btn-sm"
                  disabled={currentPage === 1}
                  onClick={() => setCurrentPage((p) => p - 1)}
                >
                  Previous
                </button>
                {Array.from({ length: totalPages }, (_, i) => i + 1)
                  .filter((page) => {
                    if (totalPages <= 5) return true;
                    if (page === 1 || page === totalPages) return true;
                    if (Math.abs(page - currentPage) <= 1) return true;
                    return false;
                  })
                  .map((page, index, arr) => (
                    <span key={page}>
                      {index > 0 && arr[index - 1] !== page - 1 && (
                        <button className="join-item btn btn-sm btn-disabled">...</button>
                      )}
                      <button
                        className={`join-item btn btn-sm ${currentPage === page ? 'btn-active' : ''}`}
                        onClick={() => setCurrentPage(page)}
                      >
                        {page}
                      </button>
                    </span>
                  ))}
                <button
                  className="join-item btn btn-sm"
                  disabled={currentPage === totalPages}
                  onClick={() => setCurrentPage((p) => p + 1)}
                >
                  Next
                </button>
              </div>
            </div>
          </div>
        )}
      </div>

      {/* Delete Confirmation Modal - DaisyUI Dialog */}
      <dialog id="delete_category_modal" className={`modal ${isDeleteModalOpen ? 'modal-open' : ''}`}>
        <div className="modal-box">
          <h3 className="font-bold text-lg">Confirm Deletion</h3>
          <p className="py-4">
            Are you sure you want to delete the category <span className="font-semibold">"{categoryToDelete?.displayName}"</span>?
            This action cannot be undone.
          </p>
          <div className="modal-action">
            <button className="btn btn-ghost" onClick={cancelDelete} disabled={isDeleting}>
              Cancel
            </button>
            <button className="btn btn-error" onClick={confirmDelete} disabled={isDeleting}>
              {isDeleting ? (
                <>
                  <span className="loading loading-spinner loading-sm"></span>
                  Deleting...
                </>
              ) : (
                'Delete'
              )}
            </button>
          </div>
        </div>
        <form method="dialog" className="modal-backdrop" onClick={cancelDelete}>
          <button>close</button>
        </form>
      </dialog>
    </div>
  );
}
