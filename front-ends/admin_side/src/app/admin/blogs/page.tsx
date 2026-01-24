import { useEffect, useState, useMemo, useCallback } from 'react';
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
  FileText,
  Copy,
} from 'lucide-react';
import { Link } from 'react-router-dom';
import type { PostModel } from '@/domains/post';
import type { CategoryModel } from '@/domains/category';
import Breadcrumbs from '../components/my-breadcrumbs';
import TableSkeleton from '../components/skeleton/table-skeleton';
import { getApiUrl, authenticatedFetch } from '@/config/api.config';
import { useAuth } from '@/auth/AuthContext';

export default function AdminBlogsPage() {
  const { token, keycloak } = useAuth();
  const [blogs, setBlogs] = useState<PostModel[]>([]);
  const [categories, setCategories] = useState<CategoryModel[]>([]);
  const [pageLoading, setPageLoading] = useState(true);
  const [sortConfig, setSortConfig] = useState<{ key: string; direction: 'asc' | 'desc' } | null>(
    null
  );
  const [textFilter, setTextFilter] = useState('');
  const [statusFilter, setStatusFilter] = useState('');
  const [categoryFilter, setCategoryFilter] = useState('');
  const [blogToDelete, setBlogToDelete] = useState<PostModel | null>(null);
  const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false);
  const [isDeleting, setIsDeleting] = useState(false);
  const [currentPage, setCurrentPage] = useState(1);
  const itemsPerPage = 10;

  useEffect(() => {
    const loadData = async () => {
      try {
        setPageLoading(true);
        const [blogsResponse, categoriesResponse] = await Promise.all([
          authenticatedFetch(
            getApiUrl('/posts?categoryType=Blog'),
            token,
            { cache: 'no-store' },
            keycloak || undefined
          ),
          authenticatedFetch(
            getApiUrl('/categories'),
            token,
            { cache: 'no-store' },
            keycloak || undefined
          ),
        ]);

        const blogsRes = await blogsResponse.json();
        const categoriesRes = await categoriesResponse.json();

        setBlogs(Array.isArray(blogsRes.data) ? blogsRes.data : []);
        setCategories(Array.isArray(categoriesRes.data) ? categoriesRes.data : []);
      } catch (error) {
        console.error('Error fetching data:', error);
        setBlogs([]);
        setCategories([]);
      } finally {
        setPageLoading(false);
      }
    };

    loadData();
  }, [token, keycloak]);

  const handleDeleteClick = (blog: PostModel) => {
    setBlogToDelete(blog);
    setIsDeleteModalOpen(true);
  };

  const confirmDelete = async () => {
    if (!blogToDelete) return;

    try {
      setIsDeleting(true);
      const response = await authenticatedFetch(
        getApiUrl(`/admin/posts/${blogToDelete.id}`),
        token,
        { method: 'DELETE' },
        keycloak || undefined
      );

      if (response.ok) {
        setBlogs((prevBlogs) => prevBlogs.filter((blog) => blog.id !== blogToDelete.id));
        setIsDeleteModalOpen(false);
        setBlogToDelete(null);
        toast.success('Blog deleted successfully');
      } else {
        console.error('Failed to delete blog:', response.statusText);
        toast.error('Failed to delete blog');
      }
    } catch (error) {
      console.error('Error deleting blog:', error);
      toast.error('An error occurred while deleting the blog');
    } finally {
      setIsDeleting(false);
    }
  };

  const cancelDelete = () => {
    setIsDeleteModalOpen(false);
    setBlogToDelete(null);
  };

  const clearFilters = () => {
    setTextFilter('');
    setStatusFilter('');
    setCategoryFilter('');
    setCurrentPage(1);
  };

  const hasActiveFilters = textFilter || statusFilter || categoryFilter;

  // Copy slug to clipboard
  const copySlug = useCallback((slug: string) => {
    navigator.clipboard.writeText(slug).then(() => {
      toast.success('Slug copied to clipboard');
    }).catch(() => {
      toast.error('Failed to copy slug');
    });
  }, []);

  // Create a map of categoryId -> categoryDisplayName for quick lookup
  const categoryMap = useMemo(() => {
    const map = new Map<string, string>();
    categories.forEach((cat) => {
      map.set(cat.id, cat.displayName);
    });
    return map;
  }, [categories]);

  // Helper function to get category display name
  const getCategoryDisplayName = (blog: PostModel): string => {
    // First try to use categoryDisplayName from the post
    if (blog.categoryDisplayName) {
      return blog.categoryDisplayName;
    }
    // Fall back to looking up from categories map
    if (blog.categoryId && categoryMap.has(blog.categoryId)) {
      return categoryMap.get(blog.categoryId) || 'Uncategorized';
    }
    return 'Uncategorized';
  };

  // Filtering and sorting with useMemo for performance
  const filteredAndSortedBlogs = useMemo(() => {
    if (!Array.isArray(blogs)) return [];
    let result = [...blogs];

    // Apply text filter
    if (textFilter) {
      result = result.filter(
        (b) =>
          b.title.toLowerCase().includes(textFilter.toLowerCase()) ||
          b.slug.toLowerCase().includes(textFilter.toLowerCase())
      );
    }

    // Apply status filter
    if (statusFilter) {
      const isPublished = statusFilter === 'published';
      result = result.filter((b) => b.published === isPublished);
    }

    // Apply category filter
    if (categoryFilter) {
      result = result.filter((b) => b.categoryId === categoryFilter);
    }

    // Apply sorting
    if (sortConfig) {
      result.sort((a: any, b: any) => {
        let aVal = a[sortConfig.key];
        let bVal = b[sortConfig.key];

        // Handle date sorting
        if (sortConfig.key === 'lastModifiedAt' || sortConfig.key === 'createdAt') {
          aVal = new Date(aVal).getTime();
          bVal = new Date(bVal).getTime();
        }

        if (aVal < bVal) return sortConfig.direction === 'asc' ? -1 : 1;
        if (aVal > bVal) return sortConfig.direction === 'asc' ? 1 : -1;
        return 0;
      });
    } else {
      // Default sort by createdAt descending
      result.sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime());
    }

    return result;
  }, [blogs, textFilter, statusFilter, categoryFilter, sortConfig]);

  // Pagination
  const totalPages = Math.ceil(filteredAndSortedBlogs.length / itemsPerPage);
  const paginatedBlogs = useMemo(() => {
    const startIndex = (currentPage - 1) * itemsPerPage;
    return filteredAndSortedBlogs.slice(startIndex, startIndex + itemsPerPage);
  }, [filteredAndSortedBlogs, currentPage, itemsPerPage]);

  // Reset to page 1 when filters change
  useEffect(() => {
    setCurrentPage(1);
  }, [textFilter, statusFilter, categoryFilter]);

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
          { label: 'Blogs' },
        ]}
      />

      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold">Blogs</h1>
          <p className="text-base-content/60 text-sm">Manage your blog posts and articles</p>
        </div>
        <Link to="/admin/blogs/create" className="btn btn-primary gap-2">
          <Plus className="w-4 h-4" />
          New Blog
        </Link>
      </div>

      {/* Collapsible Filter */}
      <details className="group bg-base-100 shadow-sm rounded-lg">
        <summary className="flex items-center justify-between cursor-pointer list-none p-4 font-medium">
          <div className="flex items-center gap-2">
            <span>Filters</span>
            {hasActiveFilters && (
              <span className="badge badge-primary badge-sm">
                {filteredAndSortedBlogs.length} / {blogs.length}
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
                placeholder="Search by title or slug..."
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

            <div className="flex gap-2 flex-wrap">
              <select
                className="select select-bordered min-w-[140px]"
                value={statusFilter}
                onChange={(e) => setStatusFilter(e.target.value)}
              >
                <option value="">All Status</option>
                <option value="published">Published</option>
                <option value="draft">Draft</option>
              </select>

              <select
                className="select select-bordered min-w-[160px]"
                value={categoryFilter}
                onChange={(e) => setCategoryFilter(e.target.value)}
              >
                <option value="">All Categories</option>
                {categories.map((cat) => (
                  <option key={cat.id} value={cat.id}>
                    {cat.displayName}
                  </option>
                ))}
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

      {/* Blogs Table */}
      <div className="card bg-base-100 shadow-sm">
        <div className="overflow-x-auto">
          {pageLoading ? (
            <div className="p-4">
              <TableSkeleton rows={5} columns={8} showHeader={true} className="w-full" />
            </div>
          ) : paginatedBlogs.length === 0 ? (
            /* Empty State */
            <div className="flex flex-col items-center justify-center py-16 px-4">
              <div className="bg-base-200 rounded-full p-4 mb-4">
                <FileText className="w-8 h-8 text-base-content/40" />
              </div>
              <h3 className="text-lg font-semibold mb-1">
                {hasActiveFilters ? 'No blogs found' : 'No blogs yet'}
              </h3>
              <p className="text-base-content/60 text-center mb-4">
                {hasActiveFilters
                  ? 'Try adjusting your search or filter criteria'
                  : 'Get started by creating your first blog post'}
              </p>
              {hasActiveFilters ? (
                <button className="btn btn-ghost btn-sm" onClick={clearFilters}>
                  Clear filters
                </button>
              ) : (
                <Link to="/admin/blogs/create" className="btn btn-primary btn-sm gap-2">
                  <Plus className="w-4 h-4" />
                  Create Blog
                </Link>
              )}
            </div>
          ) : (
            <table className="table">
              <thead>
                <tr className="bg-base-200/50">
                  <th
                    className="cursor-pointer hover:bg-base-200 transition-colors"
                    onClick={() => sortBy('title')}
                  >
                    Title
                    <SortIcon columnKey="title" />
                  </th>
                  <th>Slug</th>
                  <th
                    className="cursor-pointer hover:bg-base-200 transition-colors"
                    onClick={() => sortBy('categoryDisplayName')}
                  >
                    Category
                    <SortIcon columnKey="categoryDisplayName" />
                  </th>
                  <th
                    className="cursor-pointer hover:bg-base-200 transition-colors"
                    onClick={() => sortBy('published')}
                  >
                    Status
                    <SortIcon columnKey="published" />
                  </th>
                  <th>Tags</th>
                  <th
                    className="cursor-pointer hover:bg-base-200 transition-colors"
                    onClick={() => sortBy('lastModifiedAt')}
                  >
                    Last Modified
                    <SortIcon columnKey="lastModifiedAt" />
                  </th>
                  <th className="text-center w-28">Actions</th>
                </tr>
              </thead>
              <tbody>
                {paginatedBlogs.map((blog) => (
                  <tr key={blog.id} className="hover:bg-base-200/30 transition-colors">
                    <td className="font-medium max-w-xs">
                      <div className="truncate" title={blog.title}>
                        {blog.title}
                      </div>
                    </td>
                    <td>
                      <div className="tooltip tooltip-top before:max-w-xs before:whitespace-normal" data-tip={blog.slug}>
                        <button
                          type="button"
                          onClick={() => copySlug(blog.slug)}
                          className="flex items-center gap-1 text-xs bg-base-200 px-2 py-1 rounded hover:bg-base-300 transition-colors cursor-pointer max-w-[120px] group"
                        >
                          <code className="truncate">{blog.slug}</code>
                          <Copy className="w-3 h-3 opacity-0 group-hover:opacity-60 flex-shrink-0" />
                        </button>
                      </div>
                    </td>
                    <td className="text-base-content/70 text-sm">
                      {getCategoryDisplayName(blog)}
                    </td>
                    <td>
                      <span
                        className={`badge badge-sm ${
                          blog.published ? 'badge-success' : 'badge-ghost'
                        }`}
                      >
                        {blog.published ? 'Published' : 'Draft'}
                      </span>
                    </td>
                    <td>
                      {blog.tags && blog.tags.length > 0 ? (
                        <div className="flex flex-wrap gap-1">
                          {blog.tags.slice(0, 3).map((tag) => (
                            <span key={tag.id} className="badge badge-ghost badge-sm">
                              {tag.name}
                            </span>
                          ))}
                          {blog.tags.length > 3 && (
                            <span className="badge badge-ghost badge-sm">
                              +{blog.tags.length - 3}
                            </span>
                          )}
                        </div>
                      ) : (
                        <span className="text-base-content/40 text-sm">No tags</span>
                      )}
                    </td>
                    <td className="text-base-content/70 text-sm">
                      <div>{new Date(blog.lastModifiedAt).toLocaleDateString()}</div>
                      <div className="text-xs opacity-60">{blog.lastModifiedBy}</div>
                    </td>
                    <td>
                      <div className="flex justify-center gap-1">
                        <Link
                          to={`/admin/blogs/edit/${blog.id}`}
                          className="btn btn-ghost btn-sm btn-square"
                          title="Edit blog"
                        >
                          <Pencil className="w-4 h-4" />
                        </Link>
                        <button
                          className="btn btn-ghost btn-sm btn-square text-error hover:bg-error/10"
                          onClick={() => handleDeleteClick(blog)}
                          title="Delete blog"
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
                {Math.min(currentPage * itemsPerPage, filteredAndSortedBlogs.length)} of{' '}
                {filteredAndSortedBlogs.length} posts
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
      <dialog id="delete_blog_modal" className={`modal ${isDeleteModalOpen ? 'modal-open' : ''}`}>
        <div className="modal-box">
          <h3 className="font-bold text-lg">Confirm Deletion</h3>
          <p className="py-4">
            Are you sure you want to delete the blog{' '}
            <span className="font-semibold">"{blogToDelete?.title}"</span>? This action cannot be
            undone.
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
