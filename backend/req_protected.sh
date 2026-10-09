#!/bin/bash

RESPONSE=$(curl -s -X POST "http://localhost/auth/realms/playground_realm/protocol/openid-connect/token" \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "grant_type=password" \
  -d "client_id=frontend" \
  -d "username=ivan" \
  -d "password=ivan_password" \
  -d "scope=openid")

# Если в ответе НЕТ access_token, пишем ERROR и выходим
if ! echo "$RESPONSE" | grep -q "access_token"; then
    echo "ERROR"
    echo "$RESPONSE"
    exit 1
fi

# Извлекаем токен
ACCESS_TOKEN=$(echo "$RESPONSE" | python3 -c "import sys, json; print(json.load(sys.stdin)['access_token'])")

# Делаем запрос к бэкенду с правильным заголовком -H
RESPONSE=$(curl -s -X GET "http://localhost:80/api/protected_backend_health" \
  -H "Authorization: Bearer $ACCESS_TOKEN")
  
echo "$RESPONSE"
