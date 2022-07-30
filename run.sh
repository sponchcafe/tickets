#! /usr/bin/sh

(cd frontend/tickets; npm run build)

(cd backend/tickets; cargo r) &
(cd frontend/tickets; npm run dev)