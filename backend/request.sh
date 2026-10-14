#!/bin/bash

RESPONSE=$(curl -s -X POST "http://localhost:80/auth/realms/playground_realm/protocol/openid-connect/token" \
    -H "Content-Type: application/x-www-form-urlencoded" \
    -d "grant_type=password" \
    -d "client_id=frontend" \
    -d "username=ivan" \
    -d "password=ivan_password" \
    -d "scope=openid" \
    -d "redirect_uri=http://localhost:3000/callback"
)

echo "$RESPONSE"