export default {
  base: "/aws_lambda_rust_runtime/",
  lang: "en-US",
  title: "AWS Lambda with Rust Tutorial",
  description:
    "A step‑by‑step guide to building and deploying AWS Lambda functions using Rust.",
  lastUpdated: true,

  themeConfig: {
    // Top navigation bar
    nav: [
      { text: "Introduction", link: "/tutorial/00_intro" },
      { text: "Tutorial", link: "/tutorial/01_initial_setup" },
      {
        text: "GitHub",
        link: "https://github.com/BootstrapLaboratory/aws_lambda_rust_runtime",
      },
    ],

    // Sidebar navigation
    sidebar: [
      {
        text: "Getting started",
        collapsible: true,
        items: [{ text: "Introduction", link: "/tutorial/00_intro" }],
      },
      {
        text: "Tutorial",
        collapsible: true,
        items: [
          { text: "1. Initial Setup", link: "/tutorial/01_initial_setup" },
          {
            text: "2. Handling REST API Requests",
            link: "/tutorial/02_handle_rest_requests",
          },
          { text: "3. Deploy to AWS Lambda", link: "/tutorial/03_deploy_to_aws_lambda" },
        ],
      },
    ],

    // "Edit this page" link configuration
    editLink: {
      pattern:
        "https://github.com/BootstrapLaboratory/aws_lambda_rust_runtime/edit/main/doc/:path",
      text: "Edit this page on GitHub",
    },

    // Social media links (e.g. GitHub icon)
    socialLinks: [
      {
        icon: "github",
        link: "https://github.com/BootstrapLaboratory/aws_lambda_rust_runtime",
      },
    ],

    // Footer information
    footer: {
      message: "Released under the MIT License.",
      copyright: "Copyright © 2025 Artem Korolev",
    },

    // Local search provider (built‑in)
    search: {
      provider: "local",
    },
  },
};
