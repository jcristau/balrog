# Balrog Rust PoC - Public Update Server

This is a proof-of-concept rewrite of Balrog's public update XML endpoint in Rust using Axum and SQLx.

## Features

- `/update/6/...` endpoint compatible with existing Python implementation
- 2-phase rule matching (SQL + in-memory filtering)
- ReleaseBlobV9 support (most common Firefox update format)
- DesupportBlob support
- Emergency shutoff checks
- Pinnable release lookups
- Dockerflow health checks (`/__heartbeat__`, `/__lbheartbeat__`)
- Read-only database access

## Building

### Local Development

```bash
cargo build
```

### Docker Build

```bash
docker build -t balrog-rust-poc .
```

## Running

### Prerequisites

- MySQL database (same as Python Balrog)
- Environment variables:
  - `DBURI`: MySQL connection string (required)
  - `PORT`: Server port (default: 9010)
  - `CACHE_CONTROL`: Cache-Control header value (default: "public, max-age=90")

### Local Run

```bash
export DBURI="mysql://user:pass@localhost:3306/balrog"
cargo run
```

### Docker Run

```bash
docker run -p 9010:9010 \
  -e DBURI="mysql://user:pass@db:3306/balrog" \
  balrog-rust-poc
```

## Testing

### Smoke Test

```bash
curl "http://localhost:9010/update/6/Firefox/130.0/20240801000000/WINNT_x86_64-msvc-x64/en-US/release/Windows_NT%2010.0/ISET:SSE4_2,MEM:32768/default/default/update.xml"
```

Expected: XML response with update information or empty `<updates></updates>`

### Health Checks

```bash
# Load balancer heartbeat (always returns 200)
curl http://localhost:9010/__lbheartbeat__

# Application heartbeat (checks database connectivity)
curl http://localhost:9010/__heartbeat__
```

### Side-by-Side Testing

1. Start the existing Python Balrog server
2. Start the Rust PoC pointing to the same database
3. Send identical requests to both servers
4. Compare XML responses

```bash
# Python server (default port 8080)
curl "http://localhost:8080/update/6/..." > python.xml

# Rust server (port 9010)
curl "http://localhost:9010/update/6/..." > rust.xml

# Compare
diff python.xml rust.xml
```

## Project Structure

```
rust-poc/
  Cargo.toml           - Dependencies and project metadata
  Dockerfile           - Multi-stage Docker build
  src/
    main.rs            - Entry point, server setup
    config.rs          - Configuration from environment
    error.rs           - Error types and responses
    health.rs          - Dockerflow health endpoints
    db/
      mod.rs
      models.rs        - Database row structs
      rules.rs         - Phase 1 rule query
      releases.rs      - Release blob fetching
      emergency_shutoffs.rs
      pinnable_releases.rs
    rule_matching/
      mod.rs           - Phase 2 in-memory filtering
      channel.rs       - Channel glob matching
      version.rs       - Version parsing and comparison
      buildid.rs       - Build ID matching
      memory.rs        - Memory matching
      simple_expression.rs  - OS version CSV+AND matching
      csv.rs           - CSV and locale matching
      boolean.rs       - Boolean truth table matching
      comparison.rs    - Generic comparison operators
    blobs/
      mod.rs           - Blob factory
      base.rs          - Blob and XmlBlob traits
      release_v9.rs    - ReleaseBlobV9 implementation
      desupport.rs     - DesupportBlob implementation
    update/
      mod.rs           - Axum route handler
      query.rs         - UpdateQuery parsing
      evaluate.rs      - Rule evaluation orchestration
      response.rs      - XML response construction
```

## Out of Scope

The following features are intentionally NOT implemented in this PoC:

- Admin API (read-only PoC)
- Caching layers
- StatsD metrics
- Content signatures / Autograph
- SuperBlob, GMP, SystemAddons blob types
- Update URL versions 1-5 (only v6 supported)
- Write operations to database

## Performance Notes

- Connection pooling with SQLx (5 max connections)
- Async/await throughout with Tokio runtime
- Minimal allocations in hot paths
- Compiled release builds are highly optimized

## Testing Against Production

To compare against production data:

1. Export a production database snapshot
2. Load into a test MySQL instance
3. Start both Python and Rust servers
4. Run update requests and compare outputs
5. Look for differences in:
   - Rule matching
   - XML structure
   - URL generation
   - Header values

## Known Limitations

- Only PostModernMozillaVersion (Firefox 5+) supported
- Only tested with ReleaseBlobV9 schema
- Background rate uses non-deterministic RNG (may differ between runs)
- Some legacy URL formats not implemented
- Platform alias resolution simplified

## Next Steps

If this PoC proves successful:

1. Add remaining blob types (SuperBlob, GMP, SystemAddons)
2. Implement older update URL versions (1-5)
3. Add comprehensive integration tests
4. Performance benchmarking vs Python
5. Metrics and observability
6. Admin API implementation
7. Full production deployment plan
