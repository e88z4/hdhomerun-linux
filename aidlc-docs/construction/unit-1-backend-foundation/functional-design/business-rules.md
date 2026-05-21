# Business Rules

## Runtime and Startup Rules
- The desktop client is allowed to auto-start the bundled backend when it is not already running.
- The backend must expose a readiness state distinct from simple process existence.
- Client startup logic must wait for backend readiness before treating the backend as usable.

## Restore-State Rules
- Canonical remembered state is backend-owned.
- Canonical remembered state in v1 includes:
  - last used device
  - last watched channel
  - last known playback preference or auto-resume state
- If the remembered device is unavailable at launch, the backend clears device-specific remembered state before returning a bootstrap result.
- When remembered device state is cleared, the backend must return a selection-needed outcome rather than guessing a replacement device automatically.

## API Contract Rules
- Unit 1 must expose stable response contracts early, even before later units provide full implementations.
- Health and state endpoints must be real and meaningful in Unit 1.
- Device and playback endpoints may be provisional in Unit 1, but their response shapes must remain stable enough for the client to integrate against.
- Loopback API inputs must be validated before internal processing.

## Error-Handling Rules
- All client-visible API errors must use a structured shape.
- Structured errors must include:
  - `code`
  - `message`
  - `retryHint` or equivalent retry guidance field
- Messages must be safe for user-facing rendering and must not reveal internal runtime details.
- Retry hints should encourage recoverable actions where reasonable rather than generic failure output.

## Logging Rules
- Logs must be structured.
- Logs must avoid sensitive data.
- Logs should support request tracing and backend startup troubleshooting.

## Validation Rules
- Loopback API requests must be validated for shape, required fields, and allowed values.
- Invalid requests must fail fast with structured validation errors rather than ambiguous generic failures.
- Placeholder endpoints must still validate inputs consistently even if their downstream implementation is incomplete.

## Non-Guessing Rule
- The backend must not silently choose a different device when the remembered device disappears.
- The backend must not invent playback or device state that it cannot confirm.