#!/usr/bin/env node

const fs = require("fs");
const https = require("https");
const path = require("path");
const { parseFrontmatter } = require("./docs-frontmatter.cjs");

const rootDir = path.resolve(__dirname, "..");
const docsDir = path.join(rootDir, "docs");
const summaryPath = path.join(docsDir, "SUMMARY.md");
const indexPath = path.join(docsDir, "ai-index.jsonl");
const baseUrl = process.env.DOCS_BASE_URL || "https://dev.ackinacki.com";
const checkLive = process.argv.includes("--live");
const liveUrls = [
    `${baseUrl}/llms.txt`,
    `${baseUrl}/llms-full.txt`,
    `${baseUrl}/sitemap.xml`,
    `${baseUrl}/sitemap-pages.xml`,
];
const requiredMetadataPages = new Set([
    "docs/README.md",
    "docs/how-to-deploy-a-multisig-wallet.md",
    "docs/migration-to-tvm-sdk-v3.md",
    "docs/acki-nacki-sdk/untitled.md",
    "docs/graphql/graphql-quick-start.md",
    "docs/graphql/blockchain-api.md",
    "docs/graphql/info-api.md",
    "docs/graphql/graphql-schema-for-ai-agents.md",
    "docs/abi/abi.md",
    "docs/cryptography/mnemonics-and-keys.md",
    "docs/vm-instructions/acki-nacki-vm-instructions.md",
    "docs/for-ai-agents.md",
]);
const requiredMetadataFields = ["status", "product", "audience", "last_verified"];

function readText(filePath) {
    return fs.readFileSync(filePath, "utf8");
}

function collectSummaryLinks() {
    const links = [];
    for (const line of readText(summaryPath).split(/\r?\n/)) {
        const match = line.match(/\[([^\]]+)\]\(([^)]+)\)/);
        if (!match) {
            continue;
        }
        const target = match[2].split(/\s+/)[0].replace(/^<|>$/g, "");
        if (/^[a-z]+:\/\//i.test(target) || target.startsWith("#")) {
            continue;
        }
        if (target.endsWith(".md")) {
            links.push(target);
        }
    }
    return links;
}

function collectHiddenPages() {
    const hidden = new Set();
    const stack = [docsDir];
    while (stack.length > 0) {
        const dir = stack.pop();
        for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
            const fullPath = path.join(dir, entry.name);
            if (entry.isDirectory()) {
                stack.push(fullPath);
                continue;
            }
            if (!entry.name.endsWith(".md")) {
                continue;
            }
            const frontmatter = parseFrontmatter(readText(fullPath));
            if (frontmatter.hidden === "true") {
                hidden.add(`docs/${path.relative(docsDir, fullPath).replace(/\\/g, "/")}`);
            }
        }
    }
    return hidden;
}

function collectMarkdownPages() {
    const pages = [];
    const stack = [docsDir];
    while (stack.length > 0) {
        const dir = stack.pop();
        for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
            const fullPath = path.join(dir, entry.name);
            if (entry.isDirectory()) {
                stack.push(fullPath);
                continue;
            }
            if (entry.name.endsWith(".md")) {
                pages.push(`docs/${path.relative(docsDir, fullPath).replace(/\\/g, "/")}`);
            }
        }
    }
    return pages.sort();
}

function unlabeledFenceLines(markdown) {
    const lines = markdown.split(/\r?\n/);
    const unlabeled = [];
    let inFence = false;
    let fenceMarker = "";

    for (let i = 0; i < lines.length; i += 1) {
        const line = lines[i];
        const match = line.match(/^(```+|~~~+)(.*)$/);
        if (!match) {
            continue;
        }

        const marker = match[1];
        const rest = match[2].trim();
        if (!inFence) {
            inFence = true;
            fenceMarker = marker[0];
            if (!rest) {
                unlabeled.push(i + 1);
            }
            continue;
        }

        if (marker[0] === fenceMarker) {
            inFence = false;
            fenceMarker = "";
        }
    }

    return unlabeled;
}

function head(url) {
    return new Promise((resolve, reject) => {
        const request = https.request(url, { method: "HEAD", timeout: 20000 }, (response) => {
            response.resume();
            resolve({
                statusCode: response.statusCode,
                contentType: response.headers["content-type"] || "",
            });
        });
        request.on("timeout", () => {
            request.destroy(new Error(`Timed out: ${url}`));
        });
        request.on("error", reject);
        request.end();
    });
}

async function main() {
    const errors = [];
    const warnings = [];
    const hiddenPages = collectHiddenPages();
    const unlabeledFenceWarnings = [];

    for (const target of collectSummaryLinks()) {
        const fullPath = path.join(docsDir, target);
        if (!fs.existsSync(fullPath)) {
            errors.push(`SUMMARY.md references missing page: ${target}`);
            continue;
        }
        if (hiddenPages.has(`docs/${target}`)) {
            warnings.push(`SUMMARY.md references hidden page: ${target}`);
        }
    }

    for (const sourcePath of collectMarkdownPages()) {
        if (hiddenPages.has(sourcePath)) {
            continue;
        }
        const lines = unlabeledFenceLines(readText(path.join(rootDir, sourcePath)));
        for (const line of lines) {
            unlabeledFenceWarnings.push(`${sourcePath}:${line}`);
        }
    }
    if (unlabeledFenceWarnings.length > 0) {
        const preview = unlabeledFenceWarnings.slice(0, 30).join(", ");
        const suffix = unlabeledFenceWarnings.length > 30 ? ", ..." : "";
        warnings.push(`Unlabeled code fences found (${unlabeledFenceWarnings.length}): ${preview}${suffix}`);
    }

    if (checkLive) {
        for (const url of liveUrls) {
            try {
                const response = await head(url);
                if (response.statusCode !== 200) {
                    errors.push(`Live endpoint returned ${response.statusCode}: ${url}`);
                }
                if (url.endsWith(".txt") && !response.contentType.includes("text/")) {
                    warnings.push(`Live endpoint has unexpected content-type ${response.contentType}: ${url}`);
                }
                if (url.endsWith(".xml") && !response.contentType.includes("xml")) {
                    warnings.push(`Live endpoint has unexpected content-type ${response.contentType}: ${url}`);
                }
            } catch (error) {
                errors.push(`Live endpoint check failed for ${url}: ${error.message}`);
            }
        }
    }

    if (!fs.existsSync(indexPath)) {
        errors.push("docs/ai-index.jsonl is missing. Run: npm --prefix tools run ai:index");
    } else {
        const lines = readText(indexPath).split(/\r?\n/).filter(Boolean);
        const seen = new Set();
        const pageRecords = new Set();
        const records = [];
        for (let i = 0; i < lines.length; i += 1) {
            let record;
            try {
                record = JSON.parse(lines[i]);
            } catch (error) {
                errors.push(`docs/ai-index.jsonl line ${i + 1} is not valid JSON`);
                continue;
            }
            records.push({ line: i + 1, record });

            if (record.visibility !== "public") {
                errors.push(`Index line ${i + 1} has non-public visibility`);
            }
            if (record.type !== "page" && record.type !== "section") {
                errors.push(`Index line ${i + 1} has invalid type: ${record.type}`);
            }
            if (!record.source_path || !record.source_path.startsWith("docs/")) {
                errors.push(`Index line ${i + 1} has invalid source_path`);
                continue;
            }
            if (hiddenPages.has(record.source_path)) {
                errors.push(`Hidden page included in AI index: ${record.source_path}`);
            }

            const key = record.type === "section"
                ? `${record.type}:${record.source_path}:${record.anchor || ""}`
                : `${record.type}:${record.source_path}`;
            if (seen.has(key)) {
                errors.push(`Duplicate AI index record: ${key}`);
            }
            seen.add(key);

            const localPath = path.join(rootDir, record.source_path);
            if (!fs.existsSync(localPath)) {
                errors.push(`AI index references missing source_path: ${record.source_path}`);
            }
            if (!record.title) {
                errors.push(`AI index record has empty title: ${record.source_path}`);
            }
            if (!record.url || !record.url.startsWith(`${baseUrl}/`)) {
                errors.push(`AI index record has invalid url: ${record.source_path}`);
            }
            if (record.type === "page") {
                pageRecords.add(record.source_path);
                if (requiredMetadataPages.has(record.source_path)) {
                    for (const field of requiredMetadataFields) {
                        if (!record[field]) {
                            errors.push(`Required metadata field ${field} is missing in ${record.source_path}`);
                        }
                    }
                    if (record.last_verified && !/^\d{4}-\d{2}-\d{2}$/.test(record.last_verified)) {
                        errors.push(`Required metadata field last_verified must use YYYY-MM-DD in ${record.source_path}`);
                    }
                }
            }
            if (record.type === "section") {
                if (!record.page_title) {
                    errors.push(`Section record has empty page_title: ${record.source_path}`);
                }
                if (!record.anchor) {
                    errors.push(`Section record has empty anchor: ${record.source_path}`);
                }
                if (!record.url.includes("#")) {
                    errors.push(`Section record URL has no anchor: ${record.source_path}`);
                }
                if (!Array.isArray(record.section_path) || record.section_path.length === 0) {
                    errors.push(`Section record has invalid section_path: ${record.source_path}`);
                }
                if (record.depth !== 2 && record.depth !== 3) {
                    errors.push(`Section record has invalid depth: ${record.source_path}`);
                }
            }
        }

        for (const { line, record } of records) {
            if (record.type === "section" && !pageRecords.has(record.source_path)) {
                errors.push(`Section line ${line} has no parent page record: ${record.source_path}`);
            }
        }
    }

    for (const warning of warnings) {
        console.warn(`warning: ${warning}`);
    }

    if (errors.length > 0) {
        for (const error of errors) {
            console.error(`error: ${error}`);
        }
        process.exit(1);
    }

    console.log(`Docs AI check passed. Hidden pages excluded from AI index: ${hiddenPages.size}`);
}

main().catch((error) => {
    console.error(`error: ${error.message}`);
    process.exit(1);
});
