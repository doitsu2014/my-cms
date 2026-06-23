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
  Users as UsersIcon,
} from 'lucide-react';
import { Link } from 'react-router-dom';
import Breadcrumbs from '../components/my-breadcrumbs';
import TableSkeleton from '../components/skeleton/table-skeleton';
import type { AppUserModel, UserRole } from '@/domains/user';
import { UserRoleEnum } from '@/domains/user';
import { getApiUrl, authenticatedFetch } from '@/config/api.config';
import { useAuth } from '@/auth/AuthContext';

type SortKey = keyof Pick<AppUserModel, 'email' | 'role' | 'banned' | 'createdAt'>;
type SortConfig = { key: SortKey; direction: 'asc' | 'desc' } | null;

const ROLE_OPTIONS: { value: UserRole | ''; label: string }[] = [
  { value: '', label: 'All Roles' },
  { value: UserRoleEnum.Administrator, label: 'Administrator' },
  { value: UserRoleEnum.Writer, label: 'Writer' },
];

function getRoleLabel(role: UserRole | null): string {
  if (role === UserRoleEnum.Administrator) return 'Administrator';
  if (role === UserRoleEnum.Writer) return 'Writer';
  return 'Unknown';
}

function getRoleBadgeClass(role: UserRole | null): string {
  if (role === UserRoleEnum.Administrator) return 'badge-primary';
  if (role === UserRoleEnum.Writer) return 'badge-secondary';
  return 'badge-ghost';
}

export default function AdminUsersListPage() {
  const { token, userInfo } = useAuth();
  const [users, setUsers] = useState<AppUserModel[]>([]);
  const [pageLoading, setPageLoading] = useState(true);
  const [sortConfig, setSortConfig] = useState<SortConfig>(null);
  const [emailFilter, setEmailFilter] = useState('');
  const [roleFilter, setRoleFilter] = useState<UserRole | ''>('');
  const [userToDelete, setUserToDelete] = useState<AppUserModel | null>(null);
  const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false);
  const [isDeleting, setIsDeleting] = useState(false);
  const [currentPage, setCurrentPage] = useState(1);
  const itemsPerPage = 10;

  const currentUserId = userInfo?.id;

  useEffect(() => {
    const loadUsers = async () => {
      try {
        setPageLoading(true);
        const params = new URLSearchParams();
        params.set('page', '1');
        params.set('perPage', '200');
        if (roleFilter) params.set('role', roleFilter);
        if (emailFilter) params.set('email', emailFilter);
        const url = `${getApiUrl('/users')}?${params.toString()}`;
        const response = await authenticatedFetch(url, token, { cache: 'no-store' });
        const res = await response.json();
        setUsers(Array.isArray(res.data) ? res.data : []);
      } catch (error) {
        console.error('Failed to load users:', error);
        setUsers([]);
      } finally {
        setPageLoading(false);
      }
    };

    loadUsers();
  }, [token, roleFilter, emailFilter]);

  const handleDeleteClick = (user: AppUserModel) => {
    setUserToDelete(user);
    setIsDeleteModalOpen(true);
  };

  const confirmDelete = async () => {
    if (!userToDelete) return;

    try {
      setIsDeleting(true);
      const response = await authenticatedFetch(
        getApiUrl(`/users/${userToDelete.id}`),
        token,
        { method: 'DELETE' },
      );

      if (response.ok) {
        setUsers(users.filter((u) => u.id !== userToDelete.id));
        setIsDeleteModalOpen(false);
        setUserToDelete(null);
        toast.success('User deleted successfully');
      } else {
        let errorMessage = 'Failed to delete user';
        try {
          const errorData = await response.json();
          errorMessage = errorData?.errors?.[0] ?? errorMessage;
        } catch {
          // ignore JSON parse failure and fall back to default message
        }
        toast.error(errorMessage);
      }
    } catch (error) {
      console.error('Error deleting user:', error);
      toast.error('An error occurred while deleting the user');
    } finally {
      setIsDeleting(false);
    }
  };

  const cancelDelete = () => {
    setIsDeleteModalOpen(false);
    setUserToDelete(null);
  };

  const clearFilters = () => {
    setEmailFilter('');
    setRoleFilter('');
    setCurrentPage(1);
  };

  const hasActiveFilters = Boolean(emailFilter || roleFilter);

  const filteredAndSortedUsers = useMemo(() => {
    if (!Array.isArray(users)) return [];
    let result = [...users];

    if (emailFilter) {
      const needle = emailFilter.toLowerCase();
      result = result.filter((u) => u.email.toLowerCase().includes(needle));
    }

    if (roleFilter) {
      result = result.filter((u) => u.role === roleFilter);
    }

    if (sortConfig) {
      const { key, direction } = sortConfig;
      result.sort((a, b) => {
        const av = a[key];
        const bv = b[key];
        if (av == null && bv == null) return 0;
        if (av == null) return direction === 'asc' ? -1 : 1;
        if (bv == null) return direction === 'asc' ? 1 : -1;
        if (av < bv) return direction === 'asc' ? -1 : 1;
        if (av > bv) return direction === 'asc' ? 1 : -1;
        return 0;
      });
    }

    return result;
  }, [users, emailFilter, roleFilter, sortConfig]);

  const totalPages = Math.ceil(filteredAndSortedUsers.length / itemsPerPage);
  const paginatedUsers = useMemo(() => {
    const startIndex = (currentPage - 1) * itemsPerPage;
    return filteredAndSortedUsers.slice(startIndex, startIndex + itemsPerPage);
  }, [filteredAndSortedUsers, currentPage, itemsPerPage]);

  useEffect(() => {
    setCurrentPage(1);
  }, [emailFilter, roleFilter]);

  const sortBy = (key: SortKey) => {
    let direction: 'asc' | 'desc' = 'asc';
    if (sortConfig?.key === key && sortConfig.direction === 'asc') {
      direction = 'desc';
    }
    setSortConfig({ key, direction });
  };

  const SortIcon = ({ columnKey }: { columnKey: SortKey }) => {
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
          { label: 'Users' },
        ]}
      />

      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold">Users</h1>
          <p className="text-base-content/60 text-sm">Manage administrator and writer accounts</p>
        </div>
        <Link to="/admin/users/create" className="btn btn-primary gap-2">
          <Plus className="w-4 h-4" />
          New User
        </Link>
      </div>

      <details className="group bg-base-100 shadow-sm rounded-lg">
        <summary className="flex items-center justify-between cursor-pointer list-none p-4 font-medium">
          <div className="flex items-center gap-2">
            <span>Filters</span>
            {hasActiveFilters && (
              <span className="badge badge-primary badge-sm">
                {filteredAndSortedUsers.length} / {users.length}
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
                placeholder="Search by email..."
                className="grow"
                value={emailFilter}
                onChange={(e) => setEmailFilter(e.target.value)}
              />
              {emailFilter && (
                <button
                  type="button"
                  className="btn btn-ghost btn-xs btn-circle"
                  onClick={() => setEmailFilter('')}
                >
                  <X className="w-3 h-3" />
                </button>
              )}
            </label>

            <div className="flex gap-2">
              <select
                className="select select-bordered min-w-[160px]"
                value={roleFilter}
                onChange={(e) => setRoleFilter(e.target.value as UserRole | '')}
              >
                {ROLE_OPTIONS.map((opt) => (
                  <option key={opt.value || 'all'} value={opt.value}>
                    {opt.label}
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

      <div className="card bg-base-100 shadow-sm">
        <div className="overflow-x-auto">
          {pageLoading ? (
            <div className="p-4">
              <TableSkeleton rows={5} columns={5} showHeader={true} className="w-full" />
            </div>
          ) : paginatedUsers.length === 0 ? (
            <div className="flex flex-col items-center justify-center py-16 px-4">
              <div className="bg-base-200 rounded-full p-4 mb-4">
                <UsersIcon className="w-8 h-8 text-base-content/40" />
              </div>
              <h3 className="text-lg font-semibold mb-1">
                {hasActiveFilters ? 'No users found' : 'No users yet'}
              </h3>
              <p className="text-base-content/60 text-center mb-4">
                {hasActiveFilters
                  ? 'Try adjusting your search or filter criteria'
                  : 'Get started by creating your first user'}
              </p>
              {hasActiveFilters ? (
                <button className="btn btn-ghost btn-sm" onClick={clearFilters}>
                  Clear filters
                </button>
              ) : (
                <Link to="/admin/users/create" className="btn btn-primary btn-sm gap-2">
                  <Plus className="w-4 h-4" />
                  Create User
                </Link>
              )}
            </div>
          ) : (
            <table className="table">
              <thead>
                <tr className="bg-base-200/50">
                  <th
                    className="cursor-pointer hover:bg-base-200 transition-colors"
                    onClick={() => sortBy('email')}
                  >
                    Email
                    <SortIcon columnKey="email" />
                  </th>
                  <th
                    className="cursor-pointer hover:bg-base-200 transition-colors"
                    onClick={() => sortBy('role')}
                  >
                    Role
                    <SortIcon columnKey="role" />
                  </th>
                  <th
                    className="cursor-pointer hover:bg-base-200 transition-colors"
                    onClick={() => sortBy('banned')}
                  >
                    Status
                    <SortIcon columnKey="banned" />
                  </th>
                  <th
                    className="cursor-pointer hover:bg-base-200 transition-colors"
                    onClick={() => sortBy('createdAt')}
                  >
                    Created
                    <SortIcon columnKey="createdAt" />
                  </th>
                  <th className="text-center w-28">Actions</th>
                </tr>
              </thead>
              <tbody>
                {paginatedUsers.map((user) => {
                  const isSelf = Boolean(currentUserId && user.id === currentUserId);
                  return (
                    <tr key={user.id} className="hover:bg-base-200/30 transition-colors">
                      <td className="font-medium">
                        <div className="flex items-center gap-2">
                          <span>{user.email}</span>
                          {isSelf && (
                            <span className="badge badge-outline badge-sm">You</span>
                          )}
                        </div>
                      </td>
                      <td>
                        <span className={`badge badge-sm ${getRoleBadgeClass(user.role)}`}>
                          {getRoleLabel(user.role)}
                        </span>
                      </td>
                      <td>
                        {user.banned ? (
                          <span className="badge badge-sm badge-error">Banned</span>
                        ) : (
                          <span className="badge badge-sm badge-success">Active</span>
                        )}
                      </td>
                      <td className="text-base-content/70 text-sm">
                        {new Date(user.createdAt).toLocaleDateString()}
                      </td>
                      <td>
                        <div className="flex justify-center gap-1">
                          <Link
                            to={`/admin/users/edit/${user.id}`}
                            className="btn btn-ghost btn-sm btn-square"
                            title="Edit user"
                          >
                            <Pencil className="w-4 h-4" />
                          </Link>
                          <button
                            className="btn btn-ghost btn-sm btn-square text-error hover:bg-error/10 disabled:opacity-40 disabled:cursor-not-allowed"
                            onClick={() => handleDeleteClick(user)}
                            disabled={isSelf}
                            title={
                              isSelf ? 'You cannot delete your own account' : 'Delete user'
                            }
                          >
                            <Trash2 className="w-4 h-4" />
                          </button>
                        </div>
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          )}
        </div>

        {!pageLoading && totalPages > 1 && (
          <div className="card-body border-t border-base-200 py-3">
            <div className="flex flex-col sm:flex-row items-center justify-between gap-3">
              <span className="text-sm text-base-content/60">
                Showing {(currentPage - 1) * itemsPerPage + 1} to{' '}
                {Math.min(currentPage * itemsPerPage, filteredAndSortedUsers.length)} of{' '}
                {filteredAndSortedUsers.length} users
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

      <dialog id="delete_user_modal" className={`modal ${isDeleteModalOpen ? 'modal-open' : ''}`}>
        <div className="modal-box">
          <h3 className="font-bold text-lg">Confirm Deletion</h3>
          <p className="py-4">
            Are you sure you want to delete the user{' '}
            <span className="font-semibold">"{userToDelete?.email}"</span>? This action cannot be
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
