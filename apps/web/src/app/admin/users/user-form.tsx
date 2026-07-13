import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useForm, type Resolver } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { toast } from 'sonner';
import {
  createUserSchema,
  modifyUserSchema,
  resetPasswordSchema,
  type CreateUserFormData,
  type ModifyUserFormData,
  type ResetPasswordFormData,
} from '@/schemas/user.schema';
import {
  UserRoleEnum,
  type AppUserModel,
  type UserRole,
} from '@/domains/user';
import { getApiUrl, authenticatedFetch } from '@/config/api.config';
import { useAuth } from '@/auth/AuthContext';
import {
  Save,
  X,
  User as UserIcon,
  Mail,
  KeyRound,
  Shield,
  ShieldOff,
  ShieldCheck,
  Phone,
  IdCard,
} from 'lucide-react';

type UserFormValues = {
  email: string;
  password: string;
  role: UserRole;
  banned: boolean;
  fullName: string;
  phone: string;
};

export default function UserForm({ id }: { id?: string }) {
  const navigate = useNavigate();
  const { token } = useAuth();
  const [fetchingData, setFetchingData] = useState(false);
  const [fabOpen, setFabOpen] = useState(false);
  const [resetOpen, setResetOpen] = useState(false);
  const [resetPassword, setResetPassword] = useState('');
  const [resetting, setResetting] = useState(false);
  const isEdit = Boolean(id);

  const {
    register,
    handleSubmit,
    reset,
    watch,
    formState: { errors, isSubmitting },
  } = useForm<UserFormValues>({
    resolver: zodResolver(
      isEdit ? modifyUserSchema : createUserSchema,
    ) as unknown as Resolver<UserFormValues>,
    defaultValues: {
      email: '',
      password: '',
      role: UserRoleEnum.Writer,
      banned: false,
      fullName: '',
      phone: '',
    },
  });

  const bannedValue = watch('banned');

  const isLoading = isSubmitting || fetchingData;

  useEffect(() => {
    if (id) {
      const fetchUser = async () => {
        setFetchingData(true);
        try {
          const response = await authenticatedFetch(
            getApiUrl(`/users/${id}`),
            token,
            { cache: 'no-store' },
          );
          if (response && response.ok) {
            const res: { data: AppUserModel } = await response.json();
            reset({
              email: res.data.email,
              password: '',
              role: (res.data.role ?? UserRoleEnum.Writer) as UserRole,
              banned: res.data.banned,
              fullName: res.data.fullName ?? '',
              phone: res.data.phone ?? '',
            });
          } else {
            toast.error('Failed to load user');
          }
        } catch (error) {
          console.error('Failed to load user:', error);
          toast.error('Error loading user');
        } finally {
          setFetchingData(false);
        }
      };

      fetchUser();
    } else {
      reset({
        email: '',
        password: '',
        role: UserRoleEnum.Writer,
        banned: false,
        fullName: '',
        phone: '',
      });
    }
  }, [id, reset, token]);

  const onSubmit = async (data: UserFormValues) => {
    try {
      if (id) {
        const updatePayload: ModifyUserFormData = {
          email: data.email,
          role: data.role,
          banned: data.banned,
          fullName: data.fullName || undefined,
          phone: data.phone || undefined,
        };

        const updateResponse = await authenticatedFetch(
          getApiUrl(`/users/${id}`),
          token,
          {
            method: 'PUT',
            headers: {
              'Content-Type': 'application/json',
            },
            body: JSON.stringify(updatePayload),
          },
        );

        if (updateResponse.ok) {
          toast.success('User updated');
          navigate('/admin/users');
        } else {
          const errorData = await updateResponse.json();
          console.error(errorData, updateResponse.status);
          toast.error(errorData?.message ?? 'Failed to update user');
        }
      } else {
        const createPayload: CreateUserFormData = {
          email: data.email,
          password: data.password,
          role: data.role,
          fullName: data.fullName || undefined,
          phone: data.phone || undefined,
        };

        const createResponse = await authenticatedFetch(
          getApiUrl('/users'),
          token,
          {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
            },
            body: JSON.stringify(createPayload),
          },
        );

        if (createResponse.ok) {
          toast.success(
            `User created. Share this password securely: ${data.password}`,
            { duration: 30000 },
          );
          navigate('/admin/users');
        } else {
          const errorData = await createResponse.json();
          console.error(errorData, createResponse.status);
          toast.error(errorData?.message ?? 'Failed to create user');
        }
      }
    } catch (error) {
      console.error('Error submitting form:', error);
      toast.error('Network error. Please try again.');
    }
  };

  const onResetPassword = async () => {
    const parsed = resetPasswordSchema.safeParse({ password: resetPassword });
    if (!parsed.success) {
      toast.error(parsed.error.issues[0]?.message ?? 'Invalid password');
      return;
    }
    if (!id) return;
    try {
      setResetting(true);
      const response = await authenticatedFetch(
        getApiUrl(`/users/${id}/reset-password`),
        token,
        {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ password: parsed.data.password }),
        },
      );
      if (response.ok) {
        const res: { data: { temporaryPassword: string } } = await response.json();
        toast.success(
          `Password reset. Share this securely: ${res.data.temporaryPassword}`,
          { duration: 30000 },
        );
        setResetOpen(false);
        setResetPassword('');
        navigate('/admin/users');
      } else {
        const errorData = await response.json();
        toast.error(errorData?.message ?? 'Failed to reset password');
      }
    } catch (error) {
      console.error('Error resetting password:', error);
      toast.error('Network error. Please try again.');
    } finally {
      setResetting(false);
    }
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-6 w-full">
      {/* Identity */}
      <div className="card bg-base-100 shadow-lg border-t-4 border-t-primary hover:shadow-xl transition-shadow duration-300">
        <div className="card-body">
          <div className="flex items-start gap-4">
            <div className="bg-primary/10 p-3 rounded-xl">
              <UserIcon className="w-6 h-6 text-primary" />
            </div>
            <div className="flex-1">
              <h2 className="card-title text-lg">Identity</h2>
              <p className="text-sm text-base-content/60">
                The user's contact details and how they'll be identified in the system
              </p>
            </div>
          </div>

          <div className="divider my-2"></div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <label className="form-control w-full">
              <div className="label py-1">
                <span className="label-text font-medium">Email</span>
              </div>
              <label className={`input input-bordered flex items-center gap-2 w-full focus-within:input-primary ${errors.email ? 'input-error' : ''}`}>
                <Mail className="w-4 h-4 opacity-50 shrink-0" />
                <input
                  type="email"
                  {...register('email')}
                  className="grow bg-transparent border-0 outline-none focus:outline-none"
                  placeholder="user@example.com"
                  disabled={isLoading}
                />
              </label>
              <div className="label py-1">
                <span className={`label-text-alt ${errors.email ? 'text-error' : 'text-base-content/50'}`}>
                  {errors.email?.message ?? 'Used for sign-in and notifications'}
                </span>
              </div>
            </label>

            <label className="form-control w-full">
              <div className="label py-1">
                <span className="label-text font-medium">Full name</span>
              </div>
              <label className={`input input-bordered flex items-center gap-2 w-full focus-within:input-primary ${errors.fullName ? 'input-error' : ''}`}>
                <IdCard className="w-4 h-4 opacity-50 shrink-0" />
                <input
                  type="text"
                  {...register('fullName')}
                  className="grow bg-transparent border-0 outline-none focus:outline-none"
                  placeholder="Alice Example"
                  disabled={isLoading}
                  maxLength={120}
                />
              </label>
              <div className="label py-1">
                <span className={`label-text-alt ${errors.fullName ? 'text-error' : 'text-base-content/50'}`}>
                  {errors.fullName?.message ?? 'Up to 120 characters'}
                </span>
              </div>
            </label>

            <label className="form-control w-full md:col-span-2">
              <div className="label py-1">
                <span className="label-text font-medium">Phone</span>
              </div>
              <label className={`input input-bordered flex items-center gap-2 w-full focus-within:input-primary ${errors.phone ? 'input-error' : ''}`}>
                <Phone className="w-4 h-4 opacity-50 shrink-0" />
                <input
                  type="tel"
                  {...register('phone')}
                  className="grow bg-transparent border-0 outline-none focus:outline-none"
                  placeholder="+14155550100"
                  disabled={isLoading}
                />
              </label>
              <div className="label py-1">
                <span className={`label-text-alt ${errors.phone ? 'text-error' : 'text-base-content/50'}`}>
                  {errors.phone?.message ?? 'Optional · E.164 format, e.g. +14155550100'}
                </span>
              </div>
            </label>
          </div>
        </div>
      </div>

      {/* Credentials (create mode: password) / Security actions (edit mode) */}
      <div className="card bg-base-100 shadow-lg border-t-4 border-t-warning hover:shadow-xl transition-shadow duration-300">
        <div className="card-body">
          <div className="flex items-start gap-4">
            <div className="bg-warning/10 p-3 rounded-xl">
              {isEdit ? (
                <Shield className="w-6 h-6 text-warning" />
              ) : (
                <KeyRound className="w-6 h-6 text-warning" />
              )}
            </div>
            <div className="flex-1">
              <h2 className="card-title text-lg">
                {isEdit ? 'Security actions' : 'Credentials'}
              </h2>
              <p className="text-sm text-base-content/60">
                {isEdit
                  ? 'Ban or reset password for this user. Changes take effect on their next sign-in.'
                  : 'Set the initial password the user will use to sign in.'}
              </p>
            </div>
          </div>

          <div className="divider my-2"></div>

          {!isEdit ? (
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <label className="form-control w-full">
                <div className="label py-1">
                  <span className="label-text font-medium">Password</span>
                </div>
                <label className={`input input-bordered flex items-center gap-2 w-full focus-within:input-primary ${errors.password ? 'input-error' : ''}`}>
                  <KeyRound className="w-4 h-4 opacity-50 shrink-0" />
                  <input
                    type="password"
                    {...register('password')}
                    className="grow bg-transparent border-0 outline-none focus:outline-none"
                    placeholder="At least 8 characters"
                    disabled={isLoading}
                  />
                </label>
                <div className="label py-1">
                  <span className={`label-text-alt ${errors.password ? 'text-error' : 'text-base-content/50'}`}>
                    {errors.password?.message ?? 'At least 8 characters · share securely after creation'}
                  </span>
                </div>
              </label>
            </div>
          ) : (
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {/* Banned toggle */}
              <div className="form-control w-full">
                <div className="label py-1">
                  <span className="label-text font-medium">Account status</span>
                </div>
                <div className="flex items-center justify-between gap-3 px-4 py-3 rounded-xl border border-base-300 bg-base-100">
                  <span className="flex items-center gap-2 text-sm">
                    {bannedValue ? (
                      <ShieldOff className="w-4 h-4 text-error" />
                    ) : (
                      <Shield className="w-4 h-4 text-success" />
                    )}
                    <span className="font-medium">{bannedValue ? 'Banned' : 'Active'}</span>
                  </span>
                  <input
                    type="checkbox"
                    {...register('banned')}
                    className="toggle toggle-error"
                    disabled={isLoading}
                    aria-label="Ban this user"
                  />
                </div>
                <div className="label py-1">
                  <span className={`label-text-alt ${errors.banned ? 'text-error' : 'text-base-content/50'}`}>
                    {errors.banned?.message ?? 'Banned users cannot sign in until reinstated'}
                  </span>
                </div>
              </div>

              {/* Reset password */}
              <div className="form-control w-full">
                <div className="label py-1">
                  <span className="label-text font-medium">Reset password</span>
                </div>
                <div className="flex items-center justify-between gap-3 px-4 py-3 rounded-xl border border-base-300 bg-base-100">
                  <span className="flex items-center gap-2 text-sm text-base-content/70">
                    <KeyRound className="w-4 h-4 opacity-60" />
                    <span>Force a new password</span>
                  </span>
                  <button
                    type="button"
                    className="btn btn-sm btn-outline btn-warning gap-2"
                    onClick={() => setResetOpen(true)}
                    disabled={isLoading}
                  >
                    <KeyRound className="w-3.5 h-3.5" />
                    Reset
                  </button>
                </div>
                <div className="label py-1">
                  <span className="label-text-alt text-base-content/50">
                    You'll see the new temporary password once after submission
                  </span>
                </div>
              </div>
            </div>
          )}
        </div>
      </div>

      {/* Authorisation */}
      <div className="card bg-base-100 shadow-lg border-t-4 border-t-secondary hover:shadow-xl transition-shadow duration-300">
        <div className="card-body">
          <div className="flex items-start gap-4">
            <div className="bg-secondary/10 p-3 rounded-xl">
              <ShieldCheck className="w-6 h-6 text-secondary" />
            </div>
            <div className="flex-1">
              <h2 className="card-title text-lg">Authorisation</h2>
              <p className="text-sm text-base-content/60">
                What this user is allowed to do inside the CMS
              </p>
            </div>
          </div>

          <div className="divider my-2"></div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <label className="form-control w-full">
              <div className="label py-1">
                <span className="label-text font-medium">Role</span>
              </div>
              <select
                {...register('role')}
                className={`select select-bordered w-full focus:select-primary ${errors.role ? 'select-error' : ''}`}
                disabled={isLoading}
              >
                <option value={UserRoleEnum.Administrator}>Administrator</option>
                <option value={UserRoleEnum.Writer}>Writer</option>
              </select>
              <div className="label py-1">
                <span className={`label-text-alt ${errors.role ? 'text-error' : 'text-base-content/50'}`}>
                  {errors.role?.message ?? 'Administrators can manage users and all content'}
                </span>
              </div>
            </label>
          </div>
        </div>
      </div>

      <dialog className={`modal ${resetOpen ? 'modal-open' : ''}`}>
        <div className="modal-box">
          <h3 className="font-bold text-lg">Reset password</h3>
          <p className="py-2 text-sm text-base-content/60">
            Set a new password for this user. You will see it once after submission; share it
            with the user out-of-band.
          </p>
          <label className="form-control w-full">
            <div className="label">
              <span className="label-text font-medium">New password</span>
            </div>
            <input
              type="password"
              className="input input-bordered w-full"
              placeholder="At least 8 characters"
              value={resetPassword}
              onChange={(e) => setResetPassword(e.target.value)}
              disabled={resetting}
            />
          </label>
          <div className="modal-action">
            <button
              type="button"
              className="btn btn-ghost"
              onClick={() => {
                setResetOpen(false);
                setResetPassword('');
              }}
              disabled={resetting}
            >
              Cancel
            </button>
            <button
              type="button"
              className="btn btn-warning"
              onClick={onResetPassword}
              disabled={resetting}
            >
              {resetting ? (
                <>
                  <span className="loading loading-spinner loading-sm"></span>
                  Resetting...
                </>
              ) : (
                'Reset password'
              )}
            </button>
          </div>
        </div>
        <form
          method="dialog"
          className="modal-backdrop"
          onClick={() => {
            setResetOpen(false);
            setResetPassword('');
          }}
        >
          <button>close</button>
        </form>
      </dialog>

      <div className="fixed bottom-8 right-8 z-50 flex flex-col items-center gap-3">
        <div className={`flex flex-col items-center gap-3 transition-all duration-300 ${fabOpen ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-4 pointer-events-none'}`}>
          <button
            type="button"
            className="btn btn-lg btn-circle shadow-md bg-base-100 hover:bg-base-200"
            onClick={() => {
              navigate('/admin/users');
              setFabOpen(false);
            }}
            disabled={isLoading}
          >
            <X className="w-5 h-5" />
          </button>

          <button
            type="submit"
            className="btn btn-lg btn-circle btn-success shadow-md"
            disabled={isLoading}
          >
            {isSubmitting ? (
              <span className="loading loading-spinner loading-sm"></span>
            ) : (
              <Save className="w-5 h-5" />
            )}
          </button>
        </div>

        <button
          type="button"
          className={`btn btn-lg btn-circle btn-primary shadow-lg transition-transform duration-300 ${fabOpen ? 'rotate-45' : ''}`}
          onClick={() => setFabOpen(!fabOpen)}
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="20"
            height="20"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
          >
            <line x1="12" y1="5" x2="12" y2="19"></line>
            <line x1="5" y1="12" x2="19" y2="12"></line>
          </svg>
        </button>
      </div>
    </form>
  );
}
