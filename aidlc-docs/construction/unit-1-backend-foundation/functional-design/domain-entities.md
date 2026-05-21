# Domain Entities

## 1. BackendRuntimeState
- **Purpose**: Represents the backend process lifecycle as observed by the client and internal supervisor.
- **Fields**:
  - `status`: `stopped | starting | ready | unhealthy | failed_to_start`
  - `startedAt`
  - `lastHealthCheckAt`
  - `launchMode`: `bundled_auto_start | managed_service`

## 2. RememberedContext
- **Purpose**: Canonical persisted restore context owned by the backend.
- **Fields**:
  - `deviceRef`
  - `channelRef`
  - `autoResume`
  - `updatedAt`

## 3. BootstrapResult
- **Purpose**: Represents the backend response the client uses to initialize the app.
- **Fields**:
  - `mode`: `selection_required | restored_context | resume_requested`
  - `rememberedContext`
  - `availableContractEndpoints`
  - `warnings`

## 4. ApiError
- **Purpose**: Stable structured loopback API error object.
- **Fields**:
  - `code`
  - `message`
  - `retryHint`
  - `details` (sanitized, optional, non-sensitive)

## 5. HealthStatus
- **Purpose**: Represents loopback backend readiness for the client and runtime supervisor.
- **Fields**:
  - `ready`
  - `status`
  - `serviceVersion`
  - `apiVersion`

## 6. ContractEndpointDescriptor
- **Purpose**: Represents stable API surface discovery for the client while some endpoints remain provisional.
- **Fields**:
  - `name`
  - `path`
  - `status`: `available | provisional`

## Entity Relationships
- `BackendRuntimeState` informs `HealthStatus`.
- `RememberedContext` is loaded to produce `BootstrapResult`.
- `BootstrapResult` may include multiple `ContractEndpointDescriptor` values.
- `ApiError` can be returned by any loopback endpoint when validation or processing fails.

## Entity Constraints
- `RememberedContext.deviceRef` and `RememberedContext.channelRef` may both be absent on first launch.
- `BootstrapResult.mode` must align with the presence or absence of valid remembered state.
- `ApiError.message` must always be safe for end-user display.