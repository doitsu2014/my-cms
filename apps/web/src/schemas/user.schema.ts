import { z } from 'zod';
import { UserRoleEnum } from '@/domains/user';

const optionalFullName = z
  .string()
  .max(120, 'Full name must be 120 characters or fewer')
  .optional()
  .or(z.literal('').transform(() => undefined));

const optionalPhone = z
  .string()
  .regex(/^\+[1-9]\d{6,14}$/, 'Phone must be in E.164 format (e.g. +14155550100)')
  .optional()
  .or(z.literal('').transform(() => undefined));

export const createUserSchema = z.object({
  email: z.string().email('Invalid email'),
  password: z.string().min(8, 'Password must be at least 8 characters'),
  role: z.enum([UserRoleEnum.Administrator, UserRoleEnum.Writer]),
  fullName: optionalFullName,
  phone: optionalPhone,
});
export type CreateUserFormData = z.infer<typeof createUserSchema>;

export const modifyUserSchema = z.object({
  email: z.string().email('Invalid email'),
  role: z.enum([UserRoleEnum.Administrator, UserRoleEnum.Writer]),
  banned: z.boolean().default(false),
  fullName: optionalFullName,
  phone: optionalPhone,
});
export type ModifyUserFormData = z.infer<typeof modifyUserSchema>;

export const resetPasswordSchema = z.object({
  password: z.string().min(8, 'Password must be at least 8 characters'),
});
export type ResetPasswordFormData = z.infer<typeof resetPasswordSchema>;
