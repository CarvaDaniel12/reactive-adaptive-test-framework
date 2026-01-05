# Story 15.11: Rate Limiting

Status: ready-for-dev

## Story

As a Security Engineer,
I want to implement intelligent rate limiting across all endpoints,
So that I can protect against brute force attacks, DDoS, and API abuse while maintaining legitimate usage.

## Acceptance Criteria

1. **Given** rate limiting is configured globally
   **When** a request comes in
   **Then** system checks the request against rate limits
   **And** allows the request if within limits
   **And** returns 429 Too Many Requests if limits exceeded
   **And** includes Retry-After header

2. **Given** rate limiting is configured per endpoint
   **When** an endpoint has specific limits
   **Then** system applies endpoint-specific limits instead of global limits
   **And** endpoints can have different time windows (e.g., login: 5 attempts per 15min, API: 1000 requests per hour)
   **And** configuration is flexible and can be updated without restart

3. **Given** rate limiting is configured per user/IP
   **When** rate limiting evaluates a request
   **Then** system checks limits based on user ID
   **And** system checks limits based on IP address
   **And** system checks limits based on API key (if provided)
   **And** system uses the most restrictive limit among all three

4. **Given** an API key is used
   **When** rate limiting evaluates a request
   **Then** system applies API key-specific rate limits
   **And** different API keys can have different limits
   **And** limits are configured per permission level (api.read: 1000/hour, api.write: 500/hour)

5. **Given** an endpoint allows burst traffic
   **When** legitimate traffic needs to handle occasional bursts
   **Then** system implements burst allowance mechanism
   **And** system allows short-term burst (e.g., 10 requests in 1 second)
   **And** system maintains long-term average rate (e.g., 100 requests per minute)
   **And** system refills burst allowance over time

6. **Given** rate limiting is configured with token bucket algorithm
   **When** requests arrive
   **Then** system tracks token consumption
   **And** system deducts tokens for each request
   **And** system rejects requests when tokens are exhausted
   **And** system replenishes tokens at configured rate

7. **Given** rate limiting is configured with sliding window log
   **When** tracking requests
   **Then** system maintains a rolling window of recent requests
   **And** system allows request if count within window < threshold
   **And** system updates sliding window with each request
   **And** system rejects request if count exceeds threshold

8. **Given** a user is authenticated
   **When** they make requests
   **Then** system tracks their usage per-minute
   **And** system provides clear error messages with time until limit reset
   **And** system shows remaining requests in rate limit headers

9. **Given** rate limiting is configured for protected routes
   **When** a protected route is accessed
   **Then** system applies rate limiting middleware
   **And** system can exempt certain users/roles from rate limits
   **And** system can bypass rate limits for trusted IP ranges

10. **Given** rate limit violations occur
   **When** threshold is exceeded
   **Then** system logs the violation
   **And** system sends alert if configured (Slack, email)
   **And** system tracks rate limit metrics (violations per user, per IP, per endpoint)
   **And** system provides dashboard for monitoring

## Tasks / Subtasks

- [ ] Task 1: Implement core rate limiting algorithms (AC: #6, #7)
  - [ ] 1.1: Create `token_bucket` algorithm implementation
  - [ ] 1.2: Implement `sliding_window_log` algorithm implementation
  - [ ] 1.3: Create `fixed_window` algorithm implementation
  - [ ] 1.4: Add unit tests for all algorithms
  - [ ] 1.5: Add performance benchmarks for algorithms
  - [ ] 1.6: Document algorithm characteristics (burst capacity, accuracy)

- [ ] Task 2: Create rate limiter service and configuration (AC: #1, #2, #4)
  - [ ] 2.1: Create `RateLimiter` trait for abstraction
  - [ ] 2.2: Create `RateLimitConfig` struct
  - [ ] 2.3: Implement `RateLimiterService` with multiple limiters
  - [ ] 2.4: Add support for global, per-endpoint, per-user, per-IP limits
  - [ ] 2.5: Implement API key-specific rate limiting
  - [ ] 2.6: Add burst allowance configuration
  - [ ] 2.7: Implement configurable time windows
  - [ ] 2.8: Add unit tests for service

- [ ] Task 3: Create rate limiting middleware (AC: #1, #3, #9)
  - [ ] 3.1: Create `rate_limiting_middleware.rs` in `qa-pms-auth`
  - [ ] 3.2: Implement middleware factory function
  - [ ] 3.3: Add support for multiple rate limiters
  - [ ] 3.4: Implement key extraction strategies (user_id, ip_address, api_key)
  - [ ] 3.5: Handle rate limit violations (return 429)
  - [ ] 3.6: Add Retry-After header calculation
  - [ ] 3.7: Add rate limit headers to response
  - [ ] 3.8: Implement IP/user/API key tracking
  - [ ] 3.9: Add unit tests for middleware

- [ ] Task 4: Implement rate limit storage and metrics (AC: #8, #10)
  - [ ] 4.1: Create rate limit metrics table
  - [ ] 4.2: Implement `MetricsService` for tracking violations
  - [ ] 4.3: Track violations per user, per IP, per endpoint
  - [ ] 4.4: Track violations over time (daily, hourly)
  - [ ] 4.5: Calculate aggregate statistics
  - [ ] 4.6: Implement violation alerting (email, Slack webhook)
  - [ ] 4.7: Store rate limit state in Redis for distributed systems
  - [ ] 4.8: Add unit tests for metrics service

- [ ] Task 5: Create admin rate limiting configuration UI (AC: #2, #10)
  - [ ] 5.1: Create `RateLimitingConfig` page
  - [ ] 5.2: Display global rate limiting settings
  - [ ] 5.3: Implement per-endpoint limit editor
  - [ ] 5.4: Add per-user limit configuration
  - [ ] 5.5: Add trusted IP range configuration
  - [ ] 5.6: Implement role-based exemption configuration
  - [ ] 5.7: Add algorithm selection (token bucket, sliding window, etc.)
  - [ ] 5.8: Show rate limit metrics dashboard
  - [ ] 5.9: Display violation alerts and trends
  - [ ] 5.10: Add real-time rate limit monitoring

- [ ] Task 6: Implement frontend rate limit notification (AC: #8)
  - [ ] 6.1: Create `useRateLimits` hook
  - [ ] 6.2: Track remaining requests per endpoint
  - [ ] 6.3: Display rate limit countdown timers
  - [ ] 6.4: Show user-friendly error messages
  - [ ] 6.5: Implement automatic retry with exponential backoff
  - [ ] 6.6: Handle 429 responses gracefully
  - [ ] 6.7: Add loading states during retry periods
  - [ ] 6.8: Add unit tests for hook

- [ ] Task 7: Implement rate limit analytics and reporting (AC: #10)
  - [ ] 7.1: Create rate limit analytics dashboard
  - [ ] 7.2: Display violation trends over time
  - [ ] 7.3: Show top violating IPs/users
  - [ ] 7.4: Display endpoint usage statistics
  - [ ] 7.5: Implement time series charts
  - [ ] 7.6: Add export functionality for analytics data
  - [ ] 7.7: Implement date range filtering
  - [ ] 7.8: Add aggregate statistics widgets
  - [ ] 7.9: Add real-time updates via WebSocket
  - [ ] 7.10: Add unit tests for analytics components

- [ ] Task 8: Add comprehensive error handling and logging (AC: All)
  - [ ] 8.1: Create `RateLimitError` enum
  - [ ] 8.2: Implement user-friendly error messages
  - [ ] 8.3: Log all rate limit decisions
  - [ ] 8.4: Log rate limit violations with context
  - [ ] 8.5: Include user ID, IP address, endpoint in logs
  - [ ] 8.6: Track token bucket state
  - [ ] 8.7: Implement structured logging with tracing
  - [ ] 8.8: Add alerting for repeated violations
  - [ ] 8.9: Add unit tests for error handling
  - [ ] 8.10: Add integration tests for complete flow

- [ ] Task 9: Implement DDoS protection and circuit breaker (AC: #1, #9)
  - [ ] 9.1: Create `CircuitBreaker` service
  - [ ] 9.2: Implement automatic circuit opening on sustained violations
  - [ ] 9.3: Implement automatic circuit recovery after cooldown
  - [ ] 9.4: Add circuit state tracking
  - [ ] 9.5: Implement graceful degradation (return 503)
  - [ ] 9.6: Add DDoS detection (sudden spike from same IP)
  - [ ] 9.7: Add automatic IP blocking on DDoS detection
  - [ ] 9.8: Add admin notification for circuit events
  - [ ] 9.9: Add unit tests for circuit breaker
  - [ ] 9.10: Add integration tests for DDoS protection

- [ ] Task 10: Integrate with authentication and authorization (AC: #3, #4)
  - [ ] 10.1: Track failed login attempts in rate limiter
  - [ ] 10.2: Increase lockout threshold after repeated failures
  - [ ] 10.3: Track password reset requests
  - [ ] 10.4: Prevent abuse of password reset endpoint
  - [ ] 10.5: Track MFA verification attempts
  - [ ] 10.6: Integrate with session management
  - [ ] 10.7: Integrate with API key authentication
  - [ ] 10.8: Use rate limit metrics in audit trail
  - [ ] 10.9: Add unit tests for auth integration
  - [ ] 10.10: Add integration tests for complete auth flow

## Dev Notes

### Architecture Alignment

This story implements **Rate Limiting** per Epic 15 requirements:

- **Backend Location**: `crates/qa-pms-auth/src/rate_limiting/`
- **Middleware**: `crates/qa-pms-auth/src/middleware/rate_limiting.rs`
- **Storage**: Redis for distributed systems (optional), in-memory for single-instance
- **Algorithms**: Token bucket, Sliding window log, Fixed window
- **Monitoring**: Comprehensive metrics, analytics dashboard, alerting

### Technical Implementation Details

#### Dependencies to Add
```toml
# crates/qa-pms-auth/Cargo.toml
[dependencies]
# Existing
tower-governor = "0.4"
tower = "0.4"
tokio = { version = "1.35", features = ["full"] }
axum = "0.7"
uuid = { version = "1.6", features = ["v4", "serde"] }
chrono = "0.4"

# New for rate limiting
governor = "0.5"  # More advanced rate limiting
# Optional: Redis for distributed rate limiting
redis = { version = "0.24", features = ["tokio-comp", "connection-manager"], optional = true }
```

#### Rate Limiting Algorithms

##### Token Bucket Algorithm
```rust
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct TokenBucket {
    capacity: u32,      // Maximum tokens
    tokens: u32,       // Current tokens
    refill_rate: u32,  // Tokens added per second
    last_refill: Instant,
}

#[derive(Debug, Clone)]
pub struct TokenBucketState {
    pub tokens: u32,
    pub last_refill: Instant,
    pub reset_at: Instant,
}

pub struct TokenBucket {
    state: Arc<Mutex<TokenBucketState>>,
}

impl TokenBucket {
    pub fn new(capacity: u32, refill_rate: u32) -> Self {
        Self {
            state: Arc::new(Mutex::new(TokenBucketState {
                capacity,
                tokens: capacity,
                last_refill: Instant::now(),
                reset_at: Instant::now(),
            })),
        }
    }
    
    /// Attempt to consume tokens for a request
    pub async fn consume(&self, tokens: u32) -> Result<bool, RateLimitError> {
        let mut state = self.state.lock().await;
        
        // Refill tokens if time passed
        let now = Instant::now();
        let elapsed = now.duration_since(state.last_refill);
        let tokens_to_add = (elapsed.as_secs_f64() * state.refill_rate as u64) as u32;
        state.tokens = (state.tokens + tokens_to_add).min(state.capacity);
        state.last_refill = now;
        
        // Check if enough tokens
        if state.tokens >= tokens {
            state.tokens -= tokens;
            Ok(true)
        } else {
            Err(RateLimitError::RateLimitExceeded {
                tokens_available: state.tokens,
                retry_after: Duration::from_secs_f64((tokens - state.tokens) as f64 / state.refill_rate as f64),
            })
        }
    }
}
```

##### Sliding Window Log Algorithm
```rust
use std::collections::VecDeque;
use tokio::time::Instant;

#[derive(Debug, Clone)]
pub struct SlidingWindow {
    window_size: usize,
    requests: VecDeque<Instant>,
}

#[derive(Debug, Clone)]
pub struct SlidingWindowState {
    pub requests: VecDeque<Instant>,
}

pub struct SlidingWindow {
    state: Arc<Mutex<SlidingWindowState>>,
}

impl SlidingWindow {
    pub fn new(window_size: usize) -> Self {
        Self {
            state: Arc::new(Mutex::new(SlidingWindowState {
                requests: VecDeque::with_capacity(window_size),
            })),
        }
    }
    
    /// Record a request and check if allowed
    pub async fn record_request(&self) -> Result<bool, RateLimitError> {
        let mut state = self.state.lock().await;
        let now = Instant::now();
        
        // Remove requests outside window
        while state.requests.len() >= state.window_size {
            state.requests.pop_front();
        }
        
        // Add new request
        state.requests.push_back(now);
        
        // Check if within threshold
        if state.requests.len() < state.window_size {
            Ok(true)
        } else {
            Err(RateLimitError::RateLimitExceeded {
                tokens_available: 0,
                retry_after: Duration::from_millis(1000), // Wait for oldest request to leave window
            })
        }
    }
}
```

#### Rate Limiting Service
```rust
use crate::algorithms::{TokenBucket, SlidingWindow};
use sqlx::PgPool;
use tower_governor::Governor;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    pub global: GlobalLimitConfig,
    pub endpoints: HashMap<String, EndpointLimit>,
    pub api_keys: HashMap<String, ApiKeyLimit>,
}

#[derive(Debug, Clone)]
pub struct GlobalLimitConfig {
    pub requests_per_second: u32,
    pub burst: Option<BurstConfig>,
}

#[derive(Debug, Clone)]
pub struct BurstConfig {
    pub max_burst: u32,
    pub refill_rate: u32,
}

#[derive(Debug, Clone)]
pub struct EndpointLimit {
    pub path: String,
    pub requests_per_second: u32,
    pub window_seconds: Option<u32>,
    pub burst: Option<BurstConfig>,
}

#[derive(Debug, Clone)]
pub struct ApiKeyLimit {
    pub requests_per_second: u32,
    pub permissions: Vec<String>,
}

pub struct RateLimiterService {
    db: Arc<PgPool>,
    token_buckets: Arc<Mutex<HashMap<String, Arc<TokenBucket>>>>,
    sliding_windows: Arc<Mutex<HashMap<String, Arc<SlidingWindow>>>>,
    metrics: Arc<MetricsService>,
}

impl RateLimiterService {
    pub async fn check_rate_limit(
        &self,
        user_id: Option<Uuid>,
        ip_address: Option<String>,
        api_key: Option<String>,
        endpoint: &str,
    ) -> Result<bool, RateLimitError> {
        // Get rate limit config for endpoint
        let config = self.get_endpoint_config(endpoint).await?;
        
        // Apply the most restrictive limit
        let global_check = self.check_global_limit(user_id, ip_address, &config.global).await?;
        if !global_check.is_allowed {
            return Ok(false);
        }
        
        // Check API key limit
        if let Some(key) = api_key {
            let api_key_check = self.check_api_key_limit(&key, &config.api_keys).await?;
            if !api_key_check.is_allowed {
                return Ok(false);
            }
        }
        
        // Check per-user limit
        let user_check = self.check_user_limit(user_id, &config).await?;
        if !user_check.is_allowed {
            return Ok(false);
        }
        
        // Check per-IP limit
        let ip_check = self.check_ip_limit(ip_address, &config).await?;
        if !ip_check.is_allowed {
            return Ok(false);
        }
        
        // Record request for algorithm
        let algorithm = self.get_algorithm_for_endpoint(endpoint).await?;
        let allowed = match algorithm {
            RateLimitAlgorithm::TokenBucket(bucket) => {
                bucket.consume(1).await?
            }
            RateLimitAlgorithm::SlidingWindow(window) => {
                window.record_request().await?
            }
        };
        
        if !allowed {
            // Log violation
            self.metrics.record_violation(user_id, ip_address, endpoint).await;
        }
        
        Ok(allowed)
    }
}
```

#### Rate Limiting Middleware
```rust
use axum::{
    extract::{Request, State, TypedHeader},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response, Json},
    Extension,
};
use tower_governor::{Governor, GovernorConfigBuilder};
use crate::rate_limiting::RateLimiterService;

pub async fn rate_limiting_middleware<B>(
    State(limiter_service): State<Arc<RateLimiterService>>,
    State(metrics): State<Arc<MetricsService>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // Extract key components for rate limiting
    let user_id = req.extensions().get::<UserContext>()
        .map(|uc| uc.user_id);
    let ip_address = extract_ip_address(&req);
    let api_key = extract_api_key(&req);
    let endpoint = req.uri().path();
    
    // Check rate limit
    let is_allowed = limiter_service
        .check_rate_limit(user_id, ip_address, api_key, endpoint)
        .await?;
    
    if !is_allowed {
        // Calculate retry-after
        let retry_after = limiter_service
            .get_retry_after(user_id, ip_address, api_key, endpoint)
            .await?;
        
        // Record violation
        metrics.record_violation(user_id, ip_address, endpoint).await;
        
        let response = Json(json!({
            "error": "Rate limit exceeded",
            "retry_after": retry_after.as_secs(),
            "message": format!("Too many requests. Please wait {} seconds.", retry_after.as_secs()),
        }));
        
        return Ok(Response::builder()
            .status(StatusCode::TOO_MANY_REQUESTS)
            .header("Retry-After", format!("{}; Max-Age={}", retry_after.as_secs()))
            .header("X-RateLimit-Limit", format!("{}", limiter_service.get_limit(user_id, ip_address, endpoint).await?))
            .header("X-RateLimit-Remaining", format!("{}", limiter_service.get_remaining(user_id, ip_address, endpoint).await?))
            .header("X-RateLimit-Reset", format!("{}", limiter_service.get_reset_time(user_id, ip_address, endpoint).await?()))
            .body(response.into_body())
            .unwrap());
    }
    
    Ok(next.run(req).await)
}

fn extract_ip_address(req: &Request) -> Option<String> {
    // Try multiple headers in order
    req.headers()
        .get("x-forwarded-for")
        .or_else(|| {
            req.headers()
                .get("x-real-ip")
        })
        .or_else(|| {
            req.connect_info()
                .map(|info| info.ipaddr().to_string())
        })
}

fn extract_api_key(req: &Request) -> Option<String> {
    // Check Bearer: qapms_sk_XXX
    req.headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer "))
        .and_then(|key| {
            if key.starts_with("qapms_sk_") {
                Some(key.to_string())
            } else {
                None
            }
        })
}
```

#### Rate Limit Metrics Dashboard
```typescript
// frontend/src/components/admin/RateLimitingDashboard.tsx
import { useState, useEffect } from 'react';
import { api } from '@/api';
import { BarChart, LineChart, AlertTriangle, Shield, TrendingUp } from 'lucide-react';

interface RateLimitMetric {
  endpoint: string;
  total_requests: number;
  limited_requests: number;
  limit_rate: number;
  top_violating_ips: string[];
  top_violating_users: string[];
}

export const RateLimitingDashboard: React.FC = () => {
  const [metrics, setMetrics] = useState<RateLimitMetric[]>([]);
  const [timeRange, setTimeRange] = useState('24h');
  const [loading, setLoading] = useState(false);
  
  useEffect(() => {
    loadMetrics();
  }, [timeRange]);
  
  const loadMetrics = async () => {
    setLoading(true);
    try {
      const response = await api.get('/admin/rate-limit/metrics', {
        time_range
      });
      setMetrics(response.data);
    } catch (error) {
      console.error('Failed to load rate limit metrics:', error);
    } finally {
      setLoading(false);
    }
  };
  
  return (
    <div className="rate-limit-dashboard">
      <div className="header">
        <h2>Rate Limiting Dashboard</h2>
        <div className="time-range">
          <select value={timeRange} onChange={(e) => setTimeRange(e.target.value)}>
            <option value="1h">Last 1 Hour</option>
            <option value="6h">Last 6 Hours</option>
            <option value="24h">Last 24 Hours</option>
            <option value="7d">Last 7 Days</option>
            <option value="30d">Last 30 Days</option>
          </select>
        </div>
      </div>
      
      <div className="overview">
        <div className="stat-card">
          <Shield className="icon" />
          <div>
            <h3>Total Requests</h3>
            <p className="value">{metrics.reduce((sum, m) => sum + m.total_requests, 0).toLocaleString()}</p>
          </div>
        </div>
        <div className="stat-card">
          <AlertTriangle className="icon warning" />
          <div>
            <h3>Limited Requests</h3>
            <p className="value">{metrics.reduce((sum, m) => sum + m.limited_requests, 0).toLocaleString()}</p>
          </div>
        </div>
        <div className="stat-card">
          <TrendingUp className="icon success" />
          <div>
            <h3>Limit Rate</h3>
            <p className="value">{((metrics.reduce((sum, m) => sum + m.limit_rate, 0) / metrics.length).toFixed(2))} / second</p>
          </div>
        </div>
      </div>
      
      <div className="charts">
        <div className="chart">
          <h3>Requests Over Time</h3>
          <LineChart data={metrics.map(m => m.total_requests)} />
        </div>
        <div className="chart">
          <h3>Violations Over Time</h3>
          <BarChart data={metrics.map(m => m.limited_requests)} />
        </div>
      </div>
      
      <div className="top-violators">
        <div className="violators-list">
          <h3>Top Violating IPs</h3>
          {metrics[0]?.top_violating_ips.map((ip, index) => (
            <div key={index} className="violator-item">
              <span>{ip}</span>
              <span>{metrics[0].total_requests} requests</span>
            </div>
          ))}
        </div>
        <div className="violators-list">
          <h3>Top Violating Users</h3>
          {metrics[0]?.top_violating_users.map((userId, index) => (
            <div key={index} className="violator-item">
              <span>{userId}</span>
              <span>{metrics[0].total_requests} requests</span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};
```

### Testing Strategy

#### Unit Tests
- Token bucket algorithm correctness
- Sliding window algorithm correctness
- Rate limit calculation logic
- Burst allowance implementation
- Multiple limit evaluation (most restrictive)

#### Integration Tests
- Complete rate limiting flow
- Token bucket consumption
- Sliding window tracking
- Rate limit violation logging
- API key-specific rate limiting
- Per-user rate limiting
- Per-IP rate limiting

#### End-to-End Tests
- Endpoint with global rate limit
- Endpoint with specific rate limit
- API key with specific rate limit
- User exceeding rate limit
- IP exceeding rate limit
- Multiple requests from same user
- Burst traffic handling
- Rate limit violation detection

#### Security Tests
- Brute force attack prevention
- DDoS attack detection
- Rate limit bypass attempts
- Spoofed API key attempts
- Excessive API key usage
- Rate limit exhaustion attack
- Circuit breaker triggering

### File List

**Files to be created:**
- `crates/qa-pms-auth/src/rate_limiting/algorithms.rs`
- `crates/qa-pms-auth/src/rate_limiting/algorithms/token_bucket.rs`
- `crates/qa-pms-auth/src/rate_limiting/algorithms/sliding_window.rs`
- `crates/qa-pms-auth/src/rate_limiting/algorithms/fixed_window.rs`
- `crates/qa-pms-auth/src/rate_limiting/service.rs`
- `crates/qa-pms-auth/src/rate_limiting/middleware.rs`
- `crates/qa-pms-auth/src/metrics/service.rs`
- `crates/qa-pms-auth/src/metrics/models.rs`
- `migrations/create_rate_limit_metrics_table.sql`
- `frontend/src/pages/admin/RateLimitingConfig.tsx`
- `frontend/src/components/admin/rate-limiting/RateLimitingDashboard.tsx`

**Files to be modified:**
- `crates/qa-pms-auth/Cargo.toml` (add governor, optional redis)
- `crates/qa-pms-api/src/main.rs` (add rate limiting middleware)
- `frontend/src/api/admin.ts` (add rate limiting methods)

```
