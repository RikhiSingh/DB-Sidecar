import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import Script from "next/script";
import { ThemeProvider } from "next-themes";
import "./globals.css";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export const metadata: Metadata = {
  metadataBase: undefined,
  title: {
    default: "Feytra - PostgreSQL Schema Intelligence Sidecar",
    template: "%s | Feytra",
  },
  description:
    "Rust-powered sidecar that canonicalizes PostgreSQL schema, detects changes via hashes/Merkle roots, builds dependency graphs and embeddings, and exposes a clean REST API for AI Q&A and advisor tooling.",
  keywords: [
    "Feytra",
    "PostgreSQL",
    "database schema",
    "schema intelligence",
    "canonical DDL",
    "Merkle hashing",
    "dependency graph",
    "embeddings",
    "REST API",
    "Axum",
    "Rust",
    "AI",
  ],
  authors: [{ name: "Feytra Team" }],
  creator: "Feytra Team",
  publisher: "Feytra",
  applicationName: "Feytra",
  openGraph: {
    title: "Feytra - PostgreSQL Schema Intelligence Sidecar",
    description:
      "Schema canonicalization, change detection, dependency graph, embeddings, and an OpenAPI-first REST layer.",
    type: "website",
    siteName: "Feytra",
    images: [
      {
        url: "/favicon.ico",
        width: 64,
        height: 64,
        alt: "Feytra logo",
      },
    ],
  },
  twitter: {
    card: "summary",
    title: "Feytra - PostgreSQL Schema Intelligence Sidecar",
    description:
      "Rust-powered schema intelligence and AI Q&A layer for PostgreSQL.",
    images: ["/logo.webp"],
  },
  robots: {
    index: true,
    follow: true,
  },
  icons: {
    icon: "/favicon.ico",
    shortcut: "/logo.webp",
    apple: "/logo.webp",
  },
  themeColor: [{ media: "(prefers-color-scheme: dark)", color: "#2563eb" }],
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body
        className={`${geistSans.variable} ${geistMono.variable} antialiased`}
      >
        <ThemeProvider
          attribute="class"
          defaultTheme="dark"
          forcedTheme="dark"
          disableTransitionOnChange
        >
          <Script
            id="ld-json-organization"
            type="application/ld+json"
            strategy="afterInteractive"
          >
            {JSON.stringify({
              "@context": "https://schema.org",
              "@type": "Organization",
              name: "Feytra",
              url: "",
              logo: "/favicon.ico",
              sameAs: [],
              description:
                "Rust-powered sidecar that provides schema intelligence for PostgreSQL: canonical DDL, Merkle hashing, dependency graphs, embeddings, and an OpenAPI-first REST API.",
            })}
          </Script>
          {children}
        </ThemeProvider>
      </body>
    </html>
  );
}
