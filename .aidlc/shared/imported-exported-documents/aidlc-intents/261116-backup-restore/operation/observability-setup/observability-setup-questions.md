# Observability Setup Questions

## Q1. [Operations engineer] Deployed backup service target
Is there an already deployed environment for the backup/restore CLI to monitor? This intent skipped deployment because no target environment or deployment configuration exists.

A. No deployment exists yet; document required signals but do not create environment-specific dashboards or alarms
B. A staging deployment exists and can be named for configuration
C. A production deployment exists and can be named for configuration
D. Other (please specify)
X. Other (please specify)

[Answer]:

## Q2. [Operations engineer] Backup/restore operational SLOs
What measured service objectives should govern a future functioning backup/restore service? The current CLI does not perform backup or restore, so a live SLO cannot be measured yet.

A. Do not set numeric SLOs until working backup/restore capability and a deployment exist; list candidate SLIs and mark targets pending
B. Set a provisional objective of 99% successful backup operations over a rolling 30-day window and 99% successful restores in quarterly recovery drills
C. Set a provisional restore-time objective of 4 hours and data-loss objective of 24 hours, explicitly unverified until end-to-end drills are possible
D. Other (please specify)
X. Other (please specify)

[Answer]:
