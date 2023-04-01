# Build stage
FROM swift:latest as builder
COPY app.swift .
RUN swiftc app.swift -o app
# Run stage
FROM swift:slim AS runner
COPY --from=builder /app .
CMD ["./app"]
