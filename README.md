# Pua

A headfirst plunge into your logs.

Pua is a tool for interactively exploring structured log files using a SQL based interface.

This is inspired by Meta's Scuba / Honeycomb's concept of wide events + interactive exploration for
observability.

## TODO:

- [ ] Integrate with `chdb`
    - [ ] See how to derive the schema on the fly
        - [ ] Accounting for ALL the fields available (the union of all fields observed in the file)
    - [ ] Create a table with the derived schema
        - [ ] All fields may be nullable, or maybe just the ones that are not in every row
    - [ ] Provide a way to give a query
        - [ ] Can we restrict only to SELECTs? Is it worth?
- [ ] Check if we can build some reasonable TUI, maybe with `ratatui`?
