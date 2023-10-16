## Inspiration

The goal was to empower individuals to assist each other in their learning journeys

## What it does

Examination is a web application that enables users to create, share, and take quizzes. Users can also explore and take quizzes created by others and track their quiz progress.

## How we built it

Front-end: Svelte, TypeScript, Tailwind, DaisyUI
Back-end: Rust
Database: PostgreSQL

## Challenges we ran into

**Integrating Rust with Svelte**: Combining Rust on the backend with Svelte on the front-end posed some integration challenges. However, with careful planning and collaboration among our team members, we overcame these challenges and achieved a seamless connection between the two components.

**Complex Quiz Structures**: Supporting various question types and creating a user-friendly quiz builder was a complex task. We invested time in designing an interface that allows users to create quizzes of varying complexity with ease.

## What we learned

One of the most significant aspects of this journey was our introduction to the Rust programming language. None of us had prior experience with Rust, but we recognized its potential for creating a robust and secure backend. We dedicated time and effort to learn Rust, and the experience has been incredibly rewarding. It showcased the importance of adaptability and continuous learning in the world of software development.

## Developing

Start a development server:

```bash
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open
```

## Building

To create a production version:

```bash
npm run build
```
