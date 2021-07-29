#!/bin/sh

docker run --name mcc-bot-db -e POSTGRES_PASSWORD=pawn -d -p 5432:5432 postgres
