"use client";

import Link from "next/link";
import Image from "next/image";
import { useEffect, useState, type ReactElement } from "react";
import { motion, type Variants, type Easing } from "framer-motion";

import { Button } from "@/components/ui/button";
import {
  Card,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Separator } from "@/components/ui/separator";
import {
  Database,
  GitBranch,
  ShieldCheck,
  LineChart,
  Search,
  Zap,
  Github,
  BookOpen,
  ExternalLink,
  Cpu,
  Rocket,
  Timer,
  Server,
  Layers,
} from "lucide-react";
import TrueFocus from "@/components/react-bits/true-focus";
import DarkVeil from "@/components/react-bits/dark-veil";

type Feature = {
  title: string;
  description: string;
  icon: React.ReactNode;
};

const container: Variants = {
  hidden: { opacity: 0, y: 16 },
  show: {
    opacity: 1,
    y: 0,
    transition: {
      staggerChildren: 0.08,
      duration: 0.5,
      ease: "easeOut" as Easing,
    },
  },
};

const item: Variants = {
  hidden: { opacity: 0, y: 12 },
  show: {
    opacity: 1,
    y: 0,
    transition: { duration: 0.5, ease: "easeOut" as Easing },
  },
};

export default function Home(): ReactElement {
  const [mounted, setMounted] = useState<boolean>(false);
  useEffect(() => setMounted(true), []);

  const features: readonly Feature[] = [
    {
      title: "Canonical DDL",
      description:
        "Stable, whitespace-normalized DDL with deterministic ordering for schema diffs and hashing.",
      icon: <Database className="h-5 w-5 text-blue-600" />,
    },
    {
      title: "Merkle Hashing",
      description:
        "Per-object SHA-256 with versioned Merkle roots for fast change detection.",
      icon: <GitBranch className="h-5 w-5 text-violet-600" />,
    },
    {
      title: "Dependency Graph",
      description:
        "Accurate relationships across tables, FKs, views and indexes for impact analysis.",
      icon: <LineChart className="h-5 w-5 text-emerald-600" />,
    },
    {
      title: "Privacy-First Stats",
      description:
        "Read-only access with configurable masking; lightweight pg_stats and samples.",
      icon: <ShieldCheck className="h-5 w-5 text-rose-600" />,
    },
    {
      title: "Vector Embeddings",
      description:
        "Pluggable providers and HNSW/Qdrant backends to power semantic Q&A.",
      icon: <Search className="h-5 w-5 text-cyan-600" />,
    },
    {
      title: "OpenAPI-first",
      description:
        "Axum REST with generated docs for /bootstrap, /refresh, /schema, /ask, /advice and more.",
      icon: <Zap className="h-5 w-5 text-amber-600" />,
    },
  ] as const;

  return (
    <div className="min-h-dvh bg-gradient-to-br from-slate-950 via-slate-900 to-slate-900">
      <a
        href="#content"
        className="sr-only focus:not-sr-only focus:fixed focus:top-3 focus:left-3 focus:z-50 focus:rounded-md focus:bg-slate-900 focus:text-white focus:px-3 focus:py-2"
      >
        Skip to content
      </a>

      {/* Navbar */}
      <header className="sticky top-0 z-40 bg-slate-900/50 backdrop-blur-xl border-b border-slate-800/60">
        <div className="container mx-auto px-4">
          <div className="h-14 md:h-16 flex items-center justify-between">
            <div className="flex items-center gap-2">
              <Link
                href="/"
                aria-label="Feytra Home"
                className="flex items-center"
              >
                <Image
                  src="/logo.webp"
                  alt="Feytra logo"
                  width={32}
                  height={32}
                  priority
                />
              </Link>
              <span className="text-base md:text-lg font-semibold tracking-tight">
                Feytra
              </span>
              <Badge variant="secondary" className="ml-2 hidden sm:inline-flex">
                Coming soon
              </Badge>
            </div>
            <nav className="hidden md:flex items-center gap-1">
              <Link
                href="#features"
                className="px-3 py-2 text-sm text-slate-300 hover:text-white"
              >
                Features
              </Link>
              <Link
                href="#how"
                className="px-3 py-2 text-sm text-slate-300 hover:text-white"
              >
                How it works
              </Link>
              <Link
                href="#api"
                className="px-3 py-2 text-sm text-slate-300 hover:text-white"
              >
                API
              </Link>
            </nav>
            <div className="flex items-center gap-2">
              <Link
                href="https://github.com/RikhiSingh"
                target="_blank"
                rel="noreferrer"
                className="hidden sm:inline-flex"
              >
                <Button variant="ghost" size="sm" className="gap-2">
                  <Github className="h-4 w-4" /> GitHub
                </Button>
              </Link>
              <Link href="#" className="hidden sm:inline-flex">
                <Button
                  variant="default"
                  size="sm"
                  className="gradient-feytra text-white shadow"
                >
                  Join waitlist
                </Button>
              </Link>
            </div>
          </div>
        </div>
      </header>

      {/* Hero */}
      <section className="relative overflow-hidden">
        {/* DarkVeil Background */}
        <div className="absolute inset-0 pointer-events-none opacity-40">
          <DarkVeil />
        </div>
        {/* Subtle gradient overlays for better readability */}
        <div className="absolute inset-0 pointer-events-none">
          <div className="absolute inset-0 bg-gradient-to-b from-slate-950/20 via-transparent to-slate-950/40" />
          <div className="absolute -top-24 -left-24 w-72 h-72 rounded-full bg-blue-500/5 blur-3xl" />
          <div className="absolute -bottom-24 -right-24 w-96 h-96 rounded-full bg-violet-500/5 blur-3xl" />
        </div>
        <div
          className="container mx-auto px-4 py-16 md:py-24 relative z-10 backdrop-blur-sm"
          id="content"
        >
          <motion.div
            variants={container}
            initial="hidden"
            animate="show"
            className="mx-auto max-w-3xl text-center"
          >
            <motion.div variants={item}>
              <Badge variant="secondary" className="mb-4">
                PostgreSQL Schema Intelligence
              </Badge>
            </motion.div>
            <motion.h1
              variants={item}
              className="text-3xl md:text-5xl font-extrabold tracking-tight leading-tight drop-shadow-lg"
            >
              Make your database{" "}
              <TrueFocus
                sentence="Understand Itself"
                manualMode={false}
                blurAmount={3}
                borderColor="var(--feytra-primary)"
                glowColor="var(--feytra-primary)"
                animationDuration={0.8}
                pauseBetweenAnimations={0.5}
                className="inline-flex"
              />
            </motion.h1>
            <motion.p
              variants={item}
              className="mt-5 text-lg md:text-xl text-slate-300 drop-shadow-md"
            >
              Feytra is a Rust sidecar that snapshots catalogs, canonicalizes
              DDL, builds a dependency graph and embeddings, and exposes an
              OpenAPI-first REST API for AI Q&A and advisor tooling.
            </motion.p>
            <motion.div
              variants={item}
              className="mt-8 flex flex-col sm:flex-row items-center justify-center gap-3"
            >
              <Link href="#api">
                <Button
                  size="lg"
                  className="gradient-feytra text-white shadow-lg"
                >
                  Explore API
                </Button>
              </Link>
              <Link
                href="https://github.com/RikhiSingh"
                target="_blank"
                rel="noreferrer"
              >
                <Button size="lg" variant="outline" className="gap-2">
                  <Github className="h-4 w-4" /> View repository
                </Button>
              </Link>
            </motion.div>

            <motion.div
              variants={item}
              className="mt-10 grid grid-cols-3 gap-8 text-left max-w-xl mx-auto"
            >
              <div>
                <div className="text-2xl md:text-3xl font-semibold">0%</div>
                <div className="text-sm text-slate-400">In-line proxying</div>
              </div>
              <div>
                <div className="text-2xl md:text-3xl font-semibold">100%</div>
                <div className="text-sm text-slate-400">Schema coverage</div>
              </div>
              <div>
                <div className="text-2xl md:text-3xl font-semibold">
                  OpenAPI
                </div>
                <div className="text-sm text-slate-400">First-class docs</div>
              </div>
            </motion.div>
          </motion.div>
        </div>
      </section>

      {/* Technology & Performance */}
      <section className="relative">
        <div className="container mx-auto px-4 py-16 md:py-24">
          <div className="mx-auto max-w-5xl text-center mb-10">
            <h2 className="text-2xl md:text-4xl font-bold tracking-tight">
              Built for speed, safety, and scale
            </h2>
            <p className="mt-3 text-slate-300">
              Rust, HNSW, and Qdrant underpin a system designed to be fast by
              default and safe by design.
            </p>
          </div>
          <div className="grid gap-4 sm:gap-6 md:grid-cols-2 lg:grid-cols-3">
            <Card className="border border-slate-200/70 dark:border-slate-800/60">
              <CardHeader>
                <div className="flex items-center gap-2">
                  <Cpu className="h-5 w-5 text-blue-500" />
                  <CardTitle className="text-base md:text-lg">
                    Rust core
                  </CardTitle>
                </div>
                <CardDescription className="mt-2">
                  Memory-safe, zero-cost abstractions with predictable
                  latencies. Built with Axum and async Rust.
                </CardDescription>
              </CardHeader>
            </Card>
            <Card className="border border-slate-200/70 dark:border-slate-800/60">
              <CardHeader>
                <div className="flex items-center gap-2">
                  <Layers className="h-5 w-5 text-violet-500" />
                  <CardTitle className="text-base md:text-lg">
                    HNSW vectors
                  </CardTitle>
                </div>
                <CardDescription className="mt-2">
                  In-process HNSW index by default for lightning-fast semantic
                  lookup of schema/context.
                </CardDescription>
              </CardHeader>
            </Card>
            <Card className="border border-slate-200/70 dark:border-slate-800/60">
              <CardHeader>
                <div className="flex items-center gap-2">
                  <Server className="h-5 w-5 text-cyan-500" />
                  <CardTitle className="text-base md:text-lg">
                    Qdrant ready
                  </CardTitle>
                </div>
                <CardDescription className="mt-2">
                  Optional external vector backend via feature flag. Scale out
                  without changing your code.
                </CardDescription>
              </CardHeader>
            </Card>
            <Card className="border border-slate-200/70 dark:border-slate-800/60">
              <CardHeader>
                <div className="flex items-center gap-2">
                  <Timer className="h-5 w-5 text-amber-500" />
                  <CardTitle className="text-base md:text-lg">
                    Out-of-band
                  </CardTitle>
                </div>
                <CardDescription className="mt-2">
                  Sidecar architecture keeps your critical path clean. Snapshot,
                  analyze, and serve without proxying queries.
                </CardDescription>
              </CardHeader>
            </Card>
            <Card className="border border-slate-200/70 dark:border-slate-800/60">
              <CardHeader>
                <div className="flex items-center gap-2">
                  <Rocket className="h-5 w-5 text-emerald-500" />
                  <CardTitle className="text-base md:text-lg">
                    Low overhead
                  </CardTitle>
                </div>
                <CardDescription className="mt-2">
                  Zero proxy latency by design. Read-only access with
                  lightweight stats and masked samples.
                </CardDescription>
              </CardHeader>
            </Card>
            <Card className="border border-slate-200/70 dark:border-slate-800/60">
              <CardHeader>
                <div className="flex items-center gap-2">
                  <ShieldCheck className="h-5 w-5 text-rose-500" />
                  <CardTitle className="text-base md:text-lg">
                    Enterprise ready
                  </CardTitle>
                </div>
                <CardDescription className="mt-2">
                  Deterministic canonicalization, Merkle roots, and dependency
                  graphs for auditable change tracking.
                </CardDescription>
              </CardHeader>
            </Card>
          </div>
        </div>
      </section>

      {/* How it works */}
      <section id="how" className="relative">
        <div className="container mx-auto px-4 py-16 md:py-24">
          <div className="mx-auto max-w-5xl">
            <div className="text-center mb-12">
              <h2 className="text-2xl md:text-4xl font-bold tracking-tight">
                How Feytra works
              </h2>
              <p className="mt-3 text-slate-600 dark:text-slate-300">
                Sidecar, not proxy. Snapshot → canonicalize → hash → graph →
                embed → serve.
              </p>
            </div>
            <div className="relative pl-6 sm:pl-10">
              <div className="absolute left-1 top-0 bottom-0 w-px bg-gradient-to-b from-blue-200 via-violet-200 to-cyan-200 dark:from-blue-900/40 dark:via-violet-900/40 dark:to-cyan-900/40" />
              <ul className="space-y-10">
                {[
                  "Bootstrap catalogs",
                  "Canonicalize DDL + Hash",
                  "Build dependency graph",
                  "Collect stats + samples",
                  "Create embeddings",
                  "Serve REST + OpenAPI",
                ].map((step, i) => (
                  <li key={step} className="relative">
                    <div className="absolute -left-2 top-1.5 w-3.5 h-3.5 rounded-full gradient-feytra ring-2 ring-slate-900" />
                    <div className="rounded-xl border bg-slate-900/60 backdrop-blur p-4 sm:p-5">
                      <div className="flex items-start gap-3">
                        <Badge variant="secondary" className="shrink-0">
                          {String(i + 1).padStart(2, "0")}
                        </Badge>
                        <div>
                          <div className="font-semibold">{step}</div>
                          <p className="text-sm text-slate-600 dark:text-slate-400 mt-1">
                            {i === 0 &&
                              "Connect with read-only creds; store metadata in SQLite for portability."}
                            {i === 1 &&
                              "Lowercase unquoted identifiers, normalize whitespace and sort order-insensitive parts."}
                            {i === 2 &&
                              "Use pg_depend to map tables, FKs, views, and indexes accurately."}
                            {i === 3 &&
                              "Collect estimated row counts and safe masked samples for context."}
                            {i === 4 &&
                              "Use pluggable providers (local stub by default) and HNSW/Qdrant backends."}
                            {i === 5 &&
                              "Expose /v1 endpoints; browse Swagger at /docs."}
                          </p>
                        </div>
                      </div>
                    </div>
                  </li>
                ))}
              </ul>
            </div>
          </div>
        </div>
      </section>

      {/* Features grid */}
      <section id="features" className="relative">
        <div className="container mx-auto px-4 py-16 md:py-24">
          <div className="text-center mb-12">
            <h2 className="text-2xl md:text-4xl font-bold tracking-tight">
              Designed for clarity and control
            </h2>
            <p className="mt-3 text-slate-600 dark:text-slate-300">
              Everything you need to understand, compare and query your schema
              safely.
            </p>
          </div>
          <motion.div
            variants={container}
            initial="hidden"
            whileInView="show"
            viewport={{ once: true, amount: 0.2 }}
            className="grid gap-4 sm:gap-6 md:grid-cols-2 lg:grid-cols-3"
          >
            {features.map((f) => (
              <motion.div key={f.title} variants={item}>
                <Card className="border border-slate-200/70 dark:border-slate-800/60 shadow-sm hover:shadow-md transition-shadow">
                  <CardHeader>
                    <div className="flex items-center gap-2 text-slate-800 dark:text-slate-200">
                      <div className="rounded-md bg-slate-50 dark:bg-slate-800/60 p-2">
                        {f.icon}
                      </div>
                      <CardTitle className="text-base md:text-lg">
                        {f.title}
                      </CardTitle>
                    </div>
                    <CardDescription className="mt-2 text-slate-600 dark:text-slate-400">
                      {f.description}
                    </CardDescription>
                  </CardHeader>
                </Card>
              </motion.div>
            ))}
          </motion.div>
        </div>
      </section>

      {/* API showcase */}
      <section id="api" className="relative">
        <div className="container mx-auto px-4 pb-16 md:pb-24">
          <div className="rounded-2xl border bg-slate-900/60 backdrop-blur p-6 md:p-8">
            <div className="flex items-center justify-between">
              <div>
                <h3 className="text-lg md:text-xl font-semibold">
                  OpenAPI-first REST
                </h3>
                <p className="text-sm text-slate-600 dark:text-slate-400 mt-1">
                  Simple, explicit endpoints with JSON responses.
                </p>
              </div>
              <Link
                href="#"
                className="hidden md:inline-flex items-center gap-1 text-sm text-slate-600 dark:text-slate-300 hover:underline"
              >
                <BookOpen className="h-4 w-4" /> Open Swagger{" "}
                <ExternalLink className="h-3.5 w-3.5" />
              </Link>
            </div>
            <Separator className="my-6" />
            <div className="grid md:grid-cols-2 gap-6 font-mono text-sm">
              <div className="space-y-2">
                <div className="rounded-md border p-3 bg-slate-50 dark:bg-slate-800/60">
                  <span className="text-emerald-600">GET</span> /health
                </div>
                <div className="rounded-md border p-3 bg-slate-50 dark:bg-slate-800/60">
                  <span className="text-sky-600">POST</span> /v1/bootstrap
                </div>
                <div className="rounded-md border p-3 bg-slate-50 dark:bg-slate-800/60">
                  <span className="text-sky-600">POST</span> /v1/refresh
                </div>
              </div>
              <div className="space-y-2">
                <div className="rounded-md border p-3 bg-slate-50 dark:bg-slate-800/60">
                  <span className="text-emerald-600">GET</span> /v1/schema/
                  {"{version}"}
                </div>
                <div className="rounded-md border p-3 bg-slate-50 dark:bg-slate-800/60">
                  <span className="text-emerald-600">GET</span> /v1/objects/
                  {"{id}"}
                </div>
                <div className="rounded-md border p-3 bg-slate-50 dark:bg-slate-800/60">
                  <span className="text-sky-600">POST</span> /v1/ask, /v1/advice
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* CTA */}
      <section className="relative">
        <div className="container mx-auto px-4 py-16 md:py-24">
          <div className="relative overflow-hidden rounded-2xl gradient-feytra text-white p-8 md:p-10">
            <div className="absolute inset-0 opacity-30 pointer-events-none">
              <div className="absolute -top-20 right-10 w-40 h-40 rounded-full bg-white/20 blur-2xl" />
            </div>
            <div className="flex flex-col md:flex-row items-center justify-between gap-6">
              <div>
                <h3 className="text-2xl md:text-3xl font-bold">
                  Ready to transform your schema workflows?
                </h3>
                <p className="mt-2 text-white/90">
                  Join the waitlist to get early access and updates as we build.
                </p>
              </div>
              <div className="flex items-center gap-3">
                <Link href="#">
                  <Button
                    size="lg"
                    variant="secondary"
                    className="text-blue-700"
                  >
                    Join waitlist
                  </Button>
                </Link>
                <Link
                  href="https://github.com/RikhiSingh"
                  target="_blank"
                  rel="noreferrer"
                >
                  <Button
                    size="lg"
                    variant="outline"
                    className="border-white text-white hover:bg-white hover:text-blue-600"
                  >
                    GitHub
                  </Button>
                </Link>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="border-t bg-slate-950">
        <div className="container mx-auto px-4 py-10">
          <div className="flex flex-col md:flex-row items-start md:items-center justify-between gap-6">
            <div className="flex items-center gap-2">
              <Image
                src="/logo.webp"
                alt="Feytra logo"
                width={28}
                height={28}
                className="rounded"
              />
              <span className="font-semibold">Feytra</span>
            </div>
            <div className="flex items-center gap-4 text-sm">
              <Link
                href="#features"
                className="text-slate-300 hover:text-white"
              >
                Features
              </Link>
              <Link href="#how" className="text-slate-300 hover:text-white">
                How it works
              </Link>
              <Link href="#api" className="text-slate-300 hover:text-white">
                API
              </Link>
              <Link
                href="https://reactbits.dev/"
                target="_blank"
                className="text-slate-300 hover:text-white"
              >
                React Bits
              </Link>
              <Link
                href="https://ui.aceternity.com/components"
                target="_blank"
                className="text-slate-300 hover:text-white"
              >
                Aceternity UI
              </Link>
            </div>
          </div>
          <Separator className="my-6" />
          <div className="text-center text-sm text-slate-400">
            <p>&copy; {new Date().getFullYear()} Feytra. Apache 2.0 License.</p>
          </div>
        </div>
      </footer>
    </div>
  );
}
