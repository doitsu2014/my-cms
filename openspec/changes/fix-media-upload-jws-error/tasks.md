## 1. Fix SupabaseStorage — add apikey header to all methods

- [x] 1.1 Add `.header("apikey", self.auth_key())` to `upload()` method (after `.bearer_auth()` on line 129)
- [x] 1.2 Add `.header("apikey", self.auth_key())` to `download()` method (after `.bearer_auth()` on line 154)
- [x] 1.3 Add `.header("apikey", self.auth_key())` to `get_info()` method (after `.bearer_auth()` on line 193)
- [x] 1.4 Add `.header("apikey", self.auth_key())` to `list_objects()` method (after `.bearer_auth()` on line 258)
- [x] 1.5 Add `.header("apikey", self.auth_key())` to `delete()` method (after `.bearer_auth()` on line 325)
- [x] 1.6 Add `.header("apikey", self.auth_key())` to `delete_batch()` method (after `.bearer_auth()` on line 352)

## 2. Update tests to assert apikey header

- [x] 2.1 Update `upload_issues_multipart_post_with_bearer_and_upsert` test to also expect `header("apikey", "service-role-test-key")`
- [x] 2.2 Update `download_returns_bytes_and_content_type_on_200` test to also expect `header("apikey", "anon-test-key")`
- [x] 2.3 Update `delete_issues_delete_to_correct_path` test to also expect `header("apikey", "service-role-test-key")`
- [x] 2.4 Update `delete_batch_issues_delete_with_body` test to also expect `header("apikey", "service-role-test-key")`

## 3. Fix SERVICE_ROLE_KEY in apps env

- [x] 3.1 Replace `SERVICE_ROLE_KEY=devkey` in `deployments/docker-swarm/apps/.env.example` with the valid JWT from `supabase/.env.example` line 19

## 4. Verification

- [x] 4.1 Run `cargo test` in `apps/api/` — all tests must pass
- [x] 4.2 Run `cargo check && cargo fmt -- --check && cargo clippy` in `apps/api/` — clean
