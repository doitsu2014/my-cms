export const UserRoleEnum = {
  Administrator: 'my-headless-cms-administrator',
  Writer: 'my-headless-cms-writer',
} as const;
export type UserRole = typeof UserRoleEnum[keyof typeof UserRoleEnum];

export interface AppUserModel {
  id: string;
  email: string;
  fullName: string | null;
  phone: string | null;
  role: UserRole | null;
  banned: boolean;
  createdAt: string;
  updatedAt: string;
  lastSignInAt: string | null;
}

export interface CreateUserRequest {
  email: string;
  password: string;
  role: UserRole;
  fullName?: string | null;
  phone?: string | null;
}

export interface CreateUserResponse {
  user: AppUserModel;
  temporaryPassword: string;
}

export interface ModifyUserRequest {
  email?: string;
  role?: UserRole;
  banned?: boolean;
  fullName?: string | null;
  phone?: string | null;
}

export interface ResetPasswordRequest {
  password: string;
}

export interface ResetPasswordResponse {
  temporaryPassword: string;
}
