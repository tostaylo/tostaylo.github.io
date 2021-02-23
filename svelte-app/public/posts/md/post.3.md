The main concepts of the book stem explore how to create Reliable, Scalable, and Maintainable software.

Reliability - The ability to tolerate hardware and software faults or failures.

Scalability - The ability to adapt to system growth under specific load parameters.

Maintainability - The ability to be operable, simple, and evolvable.

## Common System Requirements

Today's software applications are more data intensive than compute intensive.

- Store Data
- Cache Data
- Search Indexes
- Stream Processing
- Batch Processing

## Reliability

How do you ensure data remains correct and complete when things go wrong internally?

### Hardware errors

- Hardware errors are usually mitigated through redundancy.

### Human errors

- Decouple places where people make mistakes
- Well designed abstractions prevent bad patterns (must not be too strict or practice will not be followed)
- Test thoroughly at all levels (unit ... integration)
- Monitor (telemetry) smartly to give advanced warning of faults of failures to come
- Quick recoveries from errors

### Reliability Testing

- Isolate services and make them fail. Determine what other unrelated services are impacted from failure.
- System alerts if component is not meeting it's guarantees (messages in > messages out === future fault or failure)

## Scalability

If the system grows in a particular way what are our options for dealing with the growth.

### Describing Load - Avg vs Peak vs Percentile vs Median

- Active Users
- Cache Hits
- Reads vs Writes
- Server Requests
- Simultaneous Active Users

### Latency vs Response Time vs Throughput

## Maintainabilty

- Operability: Ability to operate easily
- Simplicity: Ability for newcomers to understand easily
- Evolvability: Ability to make changes easily

### Simplicity

- Have good defaults
- Predictable
- Documented
- Abstractions
