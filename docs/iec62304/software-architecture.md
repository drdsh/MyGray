# MG-SAD-001 Software Architecture Description (IEC 62304 §5.3)

## 1 Context
```
 ┌──────────┐ DICOM C-STORE (TLS opt.) ┌─────────────────────────── MyGray ───────────────────────────┐
 │Modalities│ ───────────────────────► │ SI-04 DICOM node ─► SI-03 RDSR parser                        │
 │CT/XA/NM… │ ◄── DIMSE status ──────  │      │ journal/spool (filesystem)                            │
 └──────────┘                          │      ▼                                                       │
                                       │ SI-05 Store (Grafeo, embedded) ◄──── SI-06 Alert/rules        │
 ┌──────────┐  HTTPS (SSR HTML)        │      ▲                                   │                   │
 │ Browser  │ ◄──────────────────────► │ SI-07 Web UI ─────────────────────────────┘                   │
 └──────────┘                          │ SI-08 server (composition root, supervisor) · SI-02 config    │
 ┌──────────┐ SMTP / webhook / syslog  │ SI-01 core (types shared by all)                              │
 │Hospital  │ ◄─────────────────────── │                                                              │
 │IT / SIEM │                          └──────────────────────────────────────────────────────────────┘
```

## 2 Software items
| ID | Crate | Responsibility | Class | Allowed internal deps |
|----|-------|----------------|:----:|------------------------|
| SI-01 | `mygray-core` | domain model, units, error taxonomy, audit types | B | – |
| SI-02 | `mygray-config` | config loading + validation | B | SI-01 |
| SI-03 | `mygray-rdsr` | pure RDSR parsing | B | SI-01 |
| SI-04 | `mygray-dicom-node` | SCP, durable journal, quarantine, ingest pipeline | B | SI-01, SI-02, SI-03 |
| SI-05 | `mygray-store` | Grafeo persistence, repositories, migrations, backup | B | SI-01, SI-02 |
| SI-06 | `mygray-alert` | alert bus, error capture, rule engine, sinks | B | SI-01, SI-02 |
| SI-07 | `mygray-web` | SSR web UI, auth, health | B | SI-01, SI-02, SI-05, SI-06 |
| SI-08 | `mygray-server` | composition root, adapters, supervision | B | all |

Rules (checked by `cargo xtask arch-check` on `cargo metadata`):
* No cycles; SI-03 never does I/O; SI-04 and SI-06 never depend on SI-05 —
  they use **ports** (`IngestSink`, `AlertPort`, `DoseContext`) implemented by
  adapters in SI-08 (hexagonal architecture → each item unit-testable with mocks).
* Only SI-05 talks to Grafeo; only SI-04 talks DICOM network; only SI-07 talks HTTP.

## 3 Runtime view
Tokio multi-threaded runtime. Tasks: SCP acceptor + one task per association,
ingest pipeline workers (bounded mpsc), rule engine worker, alert dispatcher,
escalation timer, heartbeat supervisor, web server. All tasks registered with
the supervisor (SI-08) which restarts on failure and reports to SI-06.

## 4 Data flow & never-lose-data design
See `detailed-design/SI-04-dicom-node.md` and ADR-0002. The raw object is the
primary record until it is persisted *and* the retention period passed; the
graph is a derived, re-buildable index (`mygray reindex --from-spool`).

## 5 Safety classification & segregation (§4.3, §5.3.5)
All items are class B. Segregation is by crate boundary + Rust ownership/type
system + `unsafe` forbidden. No item is down-classified because SI-07
(display) and SI-05 (storage) can both contribute to HAZ-003/004. A future
down-classification (e.g. statistics module → A) requires a documented
segregation argument here.

## 6 SOUP
See `soup-list.md`. Functional/performance requirements for SOUP (§5.3.3)
and required system HW/SW (§5.3.4) documented there.

## 7 Interfaces
| Interface | Spec |
|-----------|------|
| DICOM | `../dicom/conformance-statement.md` |
| HTTP UI | `../../crates/mygray-web/src/server.rs` route table |
| Health/metrics | SRS-OPS-001 |
| Notifications | SMTP, HTTPS webhook (JSON schema in detailed design SI-06), syslog RFC 5424 |
| Configuration | `apps/mygray-server/config/mygray.example.toml` |
| Ports between items | rustdoc of `IngestSink`, `AlertPort`, `DoseContext`, `Sink` |
