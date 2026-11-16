# Pull Request Checklist (code review record, IEC 62304 §5.5.2)
- [ ] Linked CR/PR id in title
- [ ] requirements.json updated; tests named after SRS ids
- [ ] No `unsafe`, `unwrap`, `expect`, bare dose `f64`, GQL outside store::query
- [ ] New errors implement `Reportable` with documented alert code (alert-codes.md)
- [ ] Risk analysis reviewed (new hazards? controls affected?)
- [ ] SOUP change → soup-list.md + deny.toml reviewed
- [ ] Docs / IFU updated
