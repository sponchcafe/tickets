# Case Study Issue Tracking System
This is a toy project to build a issue tracking system like JIRA.
It is a full stack project has three goals:

1. Study: Improve and extend my software development skills
2. Demo: Test and evaluate across the tech stack
3. Resume: Have a show and tell for interviews and potential employers

## Overview: Issue Tracking
While issue tracking systems can become quite involved (see JIRA), the basic
functionality is rather simple: The issue tracker manages a database of tickets.
Tickets are documents that describe a task. They go through states like `initial`
and `in progress` and typically associated with one or more users of the system. 
At its core, the issue tracker is therefore a CRUD application for tickets.

## Technologies
Rough idea: The backend database service will be written in Rust.
The frontend will be a Svelte app using TypeScript.

## Goals
- Testability
- Modularity
- Readability

## Non-goals
- Scalability
- Production-readiness