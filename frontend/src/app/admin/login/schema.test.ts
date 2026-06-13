import { describe, it, expect } from 'vitest';
import { LoginSchema } from './schema';

describe('LoginSchema', () => {
  it('accepts a valid email and password of 8+ characters', () => {
    const result = LoginSchema.safeParse({
      email: 'admin@example.com',
      password: 'password123',
    });
    expect(result.success).toBe(true);
  });

  it('rejects an invalid email with a path error on `email`', () => {
    const result = LoginSchema.safeParse({
      email: 'not-an-email',
      password: 'password123',
    });
    expect(result.success).toBe(false);
    if (!result.success) {
      const emailIssue = result.error.issues.find((i) => i.path[0] === 'email');
      expect(emailIssue).toBeDefined();
    }
  });

  it('rejects a password shorter than 8 characters with a path error on `password`', () => {
    const result = LoginSchema.safeParse({
      email: 'admin@example.com',
      password: 'short',
    });
    expect(result.success).toBe(false);
    if (!result.success) {
      const passwordIssue = result.error.issues.find(
        (i) => i.path[0] === 'password',
      );
      expect(passwordIssue).toBeDefined();
    }
  });

  it('rejects an empty email', () => {
    const result = LoginSchema.safeParse({ email: '', password: 'password123' });
    expect(result.success).toBe(false);
    if (!result.success) {
      const emailIssue = result.error.issues.find((i) => i.path[0] === 'email');
      expect(emailIssue).toBeDefined();
    }
  });

  it('rejects an empty password', () => {
    const result = LoginSchema.safeParse({
      email: 'admin@example.com',
      password: '',
    });
    expect(result.success).toBe(false);
    if (!result.success) {
      const passwordIssue = result.error.issues.find(
        (i) => i.path[0] === 'password',
      );
      expect(passwordIssue).toBeDefined();
    }
  });
});
