#! /usr/bin/sh

(cd backend/tickets; cargo r) &
(cd frontend/tickets; npm run dev)