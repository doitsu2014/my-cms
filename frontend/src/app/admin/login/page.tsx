import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { useNavigate, useSearchParams } from 'react-router-dom';
import { LogIn, Mail, Lock } from 'lucide-react';
import { toast } from 'sonner';
import { LoginSchema, type LoginInput } from './schema';
import { getSupabaseClient } from '@/auth/supabase';

export default function AdminLoginPage() {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const from = searchParams.get('from') ?? '/admin';

  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<LoginInput>({
    resolver: zodResolver(LoginSchema),
  });

  const onSubmit = async (data: LoginInput) => {
    const { error } = await getSupabaseClient().auth.signInWithPassword({
      email: data.email,
      password: data.password,
    });

    if (error) {
      toast.error(error.message);
      return;
    }

    navigate(from, { replace: true });
  };

  return (
    <div className="flex items-center justify-center min-h-screen bg-base-200 p-4">
      <div className="card bg-base-100 shadow-xl w-full max-w-md">
        <div className="card-body">
          <h1 className="card-title text-2xl justify-center">
            <LogIn className="w-6 h-6" />
            Sign in
          </h1>
          <p className="text-center text-gray-500 mb-4">
            Enter your email and password to access the admin panel.
          </p>

          <form onSubmit={handleSubmit(onSubmit)} className="space-y-4" noValidate>
            <div className="form-control">
              <label className="label" htmlFor="email">
                <span className="label-text">Email</span>
              </label>
              <div className="join w-full">
                <span className="join-item btn btn-square btn-ghost no-animation pointer-events-none">
                  <Mail className="w-4 h-4" />
                </span>
                <input
                  id="email"
                  type="email"
                  autoComplete="email"
                  className={`input input-bordered join-item w-full ${errors.email ? 'input-error' : ''}`}
                  placeholder="you@example.com"
                  {...register('email')}
                />
              </div>
              {errors.email && (
                <span className="label-text-alt text-error mt-1">
                  {errors.email.message}
                </span>
              )}
            </div>

            <div className="form-control">
              <label className="label" htmlFor="password">
                <span className="label-text">Password</span>
              </label>
              <div className="join w-full">
                <span className="join-item btn btn-square btn-ghost no-animation pointer-events-none">
                  <Lock className="w-4 h-4" />
                </span>
                <input
                  id="password"
                  type="password"
                  autoComplete="current-password"
                  className={`input input-bordered join-item w-full ${errors.password ? 'input-error' : ''}`}
                  placeholder="••••••••"
                  {...register('password')}
                />
              </div>
              {errors.password && (
                <span className="label-text-alt text-error mt-1">
                  {errors.password.message}
                </span>
              )}
            </div>

            <button
              type="submit"
              className="btn btn-primary w-full"
              disabled={isSubmitting}
            >
              {isSubmitting ? (
                <span className="loading loading-spinner" />
              ) : (
                <>
                  <LogIn className="w-4 h-4" />
                  Sign in
                </>
              )}
            </button>
          </form>
        </div>
      </div>
    </div>
  );
}
