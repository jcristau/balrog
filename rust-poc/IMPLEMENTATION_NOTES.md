# Implementation Notes

## Architecture Decisions

### Database Layer

- **SQLx** for compile-time checked SQL queries
- **Connection pooling** with 5 max connections (adjustable)
- **Two release storage systems** supported:
  - New-style: `releases_json` + `release_assets` with deep JSON merging
  - Old-style: `releases` table with LONGTEXT JSON

### Rule Matching

Implemented as **2-phase matching** matching Python behavior:

1. **Phase 1 (SQL)**: Filter by exact-match and NULL-match columns
   - product
   - buildTarget
   - headerArchitecture
   - distVersion

2. **Phase 2 (In-memory)**: Apply complex matching logic
   - Channel (glob matching)
   - Version (PostModernMozillaVersion comparison)
   - BuildID (string comparison with operators)
   - Memory (integer comparison)
   - OS Version (simple expression with CSV OR and && AND)
   - Instruction Set (CSV substring matching)
   - Distribution (CSV exact matching)
   - Locale (CSV exact matching)
   - MIG64 (3x3 boolean truth table)
   - JAWS (3x3 boolean truth table)

### Blob System

Trait-based design for extensibility:

- `Blob` trait: Common interface for all blobs
- `XmlBlob` trait: XML generation for update responses
- Factory pattern in `create_blob()` dispatches by `schema_version`

Implemented blob types:
- **ReleaseBlobV9** (schema_version 9): Firefox application updates
- **DesupportBlob** (schema_version 50): End-of-support notifications

### Version Parsing

- **PostModernMozillaVersion**: major.minor.patch with optional prerelease (a/b)
  - Examples: `130.0`, `130.0.1`, `130a1`
- **GlobVersion**: major.* patterns
  - Examples: `70.*`, `80.*`

Comparison operators: `<`, `<=`, `>`, `>=`, `==`

### Error Handling

- All errors return empty `<updates></updates>` XML (never 404)
- Errors are logged but don't expose internal details to clients
- Database errors, parsing errors, and rule evaluation errors all handled gracefully

### Security

- **URL validation**: Allowed domains list for update URLs
- **Input validation**: Query parameters sanitized
- **XML escaping**: Ampersands properly escaped in URLs
- **Read-only**: No write operations to database

## Key Differences from Python

### Similarities

- Identical SQL queries for Phase 1
- Same rule matching logic
- Same XML output format
- Same emergency shutoff checks
- Same pinnable release lookups

### Differences

1. **Performance**:
   - Compiled vs interpreted
   - Native async/await
   - Zero-copy deserialization where possible

2. **Type Safety**:
   - Compile-time SQL validation with SQLx
   - Strong typing for all data structures
   - No runtime type errors

3. **Memory Safety**:
   - Rust's ownership system prevents memory leaks
   - No garbage collection pauses
   - Predictable memory usage

4. **Concurrency**:
   - Tokio async runtime
   - Connection pooling built-in
   - No GIL (Global Interpreter Lock)

## Testing Strategy

### Unit Tests

- Rule matching functions (24 tests)
- Version parsing and comparison
- Boolean truth table logic
- XML escaping
- System capabilities parsing

### Integration Testing (TODO)

- Full end-to-end update request
- Database integration tests
- Side-by-side comparison with Python

### Performance Testing (TODO)

- Latency benchmarks
- Throughput benchmarks
- Memory usage profiling
- Connection pool tuning

## Known Limitations

1. **Version Classes**: Only PostModernMozillaVersion and GlobVersion implemented
   - Legacy version classes (ToolkitVersion, etc.) not needed for Firefox 5+

2. **Blob Types**: Only ReleaseBlobV9 and DesupportBlob
   - SuperBlob, GMP, SystemAddons not implemented (out of scope for PoC)

3. **Update URL Versions**: Only version 6 supported
   - Versions 1-5 are deprecated

4. **Background Rate**: Uses non-deterministic RNG
   - In production, might want seeded RNG for reproducibility

5. **Platform Aliases**: Simplified resolution
   - Complex alias chains not fully tested

## Deployment Considerations

### Environment Variables

- `DBURI`: MySQL connection string (required)
- `PORT`: Server port (default 9010)
- `CACHE_CONTROL`: Cache-Control header (default "public, max-age=90")

### Resource Requirements

- **Memory**: ~10-50MB base + connection pool overhead
- **CPU**: Minimal (async I/O bound)
- **Connections**: 5 database connections per instance

### Scaling

- **Horizontal**: Add more instances behind load balancer
- **Vertical**: Increase connection pool size
- **Database**: Consider read replicas for high traffic

### Monitoring

Recommended metrics to add:
- Request latency (p50, p95, p99)
- Database query time
- Rule evaluation time
- Cache hit rate (if caching added)
- Error rate by type
- Active connections

### High Availability

- Stateless design allows easy horizontal scaling
- Health checks for load balancer integration
- Graceful shutdown on SIGTERM
- Database connection retry logic

## Future Enhancements

1. **Caching Layer**
   - In-memory cache for rules
   - Redis for distributed caching
   - CDN integration for XML responses

2. **Metrics and Observability**
   - Prometheus metrics
   - OpenTelemetry tracing
   - Structured logging

3. **Content Signatures**
   - Autograph integration
   - Signature verification

4. **Additional Blob Types**
   - SuperBlob
   - GMP (Gecko Media Plugin)
   - SystemAddons

5. **Admin API**
   - Rule management
   - Release management
   - Emergency shutoff controls

6. **Advanced Features**
   - A/B testing framework
   - Gradual rollout controls
   - Geo-targeting

## Migration Path

1. **Phase 1**: Deploy Rust PoC alongside Python (read-only)
2. **Phase 2**: Route small percentage of traffic to Rust
3. **Phase 3**: Increase traffic gradually while monitoring
4. **Phase 4**: Full cutover once validated
5. **Phase 5**: Decommission Python server

## Code Organization

```
src/
├── main.rs              # Entry point, server setup
├── config.rs            # Configuration
├── error.rs             # Error handling
├── health.rs            # Health checks
├── db/                  # Database layer
│   ├── models.rs        # SQLx models
│   ├── rules.rs         # Rule queries
│   ├── releases.rs      # Release queries
│   ├── emergency_shutoffs.rs
│   └── pinnable_releases.rs
├── rule_matching/       # Rule matching engine
│   ├── mod.rs           # Orchestration
│   ├── comparison.rs    # Comparison operators
│   ├── version.rs       # Version parsing
│   ├── channel.rs       # Channel matching
│   ├── buildid.rs       # Build ID matching
│   ├── memory.rs        # Memory matching
│   ├── simple_expression.rs  # OS version matching
│   ├── csv.rs           # CSV matching
│   └── boolean.rs       # Boolean matching
├── blobs/               # Blob system
│   ├── mod.rs           # Factory
│   ├── base.rs          # Traits
│   ├── release_v9.rs    # ReleaseBlobV9
│   └── desupport.rs     # DesupportBlob
└── update/              # Update endpoint
    ├── mod.rs           # Handler
    ├── query.rs         # Query parsing
    ├── evaluate.rs      # Rule evaluation
    └── response.rs      # Response construction
```

## Contributing

When adding new features:

1. Add tests first (TDD)
2. Keep modules focused and small
3. Document public APIs
4. Follow Rust conventions (rustfmt, clippy)
5. Update this document with architectural decisions

## Performance Expectations

Based on Rust + Axum + SQLx stack:

- **Latency**: <10ms p99 (with warm cache)
- **Throughput**: >10,000 req/s per core
- **Memory**: <100MB per instance
- **Cold start**: <1s

These are theoretical; actual benchmarks needed.
