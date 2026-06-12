#!/usr/bin/env node

const fs = require("fs");
const path = require("path");
const {
    cleanFrontmatterValue,
    parseFrontmatter,
    stripFrontmatter,
} = require("./docs-frontmatter.cjs");

const rootDir = path.resolve(__dirname, "..");
const docsDir = path.join(rootDir, "docs");
const summaryPath = path.join(docsDir, "SUMMARY.md");
const outPath = process.argv[2]
    ? path.resolve(process.argv[2])
    : path.join(docsDir, "ai-index.jsonl");
const baseUrl = process.env.DOCS_BASE_URL || "https://dev.ackinacki.com";

function readText(filePath) {
    return fs.readFileSync(filePath, "utf8");
}

function parseSummary() {
    const summary = readText(summaryPath);
    const lines = summary.split(/\r?\n/);
    const entries = [];
    let section = "Root";

    for (const line of lines) {
        const heading = line.match(/^##\s+(.+)$/);
        if (heading) {
            section = heading[1].trim();
            continue;
        }

        const link = line.match(/\[([^\]]+)\]\(([^)]+)\)/);
        if (!link) {
            continue;
        }

        const target = link[2].split(/\s+/)[0].replace(/^<|>$/g, "");
        if (/^[a-z]+:\/\//i.test(target) || target.startsWith("#")) {
            continue;
        }
        if (!target.endsWith(".md")) {
            continue;
        }

        entries.push({
            label: link[1].trim(),
            target,
            section,
        });
    }

    return entries;
}

function pageUrl(relPath) {
    const normalized = relPath.replace(/\\/g, "/");
    if (normalized === "README.md") {
        return `${baseUrl}/readme.md`;
    }
    if (normalized.endsWith("/README.md")) {
        return `${baseUrl}/${normalized.slice(0, -"/README.md".length)}.md`;
    }
    return `${baseUrl}/${normalized}`;
}

function titleFromMarkdown(markdown, fallback) {
    const match = markdown.match(/^#\s+(.+)$/m);
    return cleanHeading(match ? match[1] : fallback);
}

function cleanHeading(text) {
    return text
        .replace(/\s*<a\s+[^>]*><\/a>\s*/gi, "")
        .replace(/<[^>]+>/g, "")
        .replace(/[*_`]/g, "")
        .replace(/[\u0000-\u001F\u007F-\u009F\u200B-\u200D\uFEFF]/g, "")
        .replace(/\s+/g, " ")
        .trim();
}

function explicitAnchorFromHeading(text) {
    const match = text.match(/<a\s+[^>]*\bid=["']([^"']+)["'][^>]*><\/a>/i);
    if (!match) {
        return "";
    }
    const anchor = match[1].trim();
    if (!anchor || /^docs-internal-/i.test(anchor)) {
        return "";
    }
    return anchor;
}

function slugifyHeading(text) {
    return cleanHeading(text)
        .toLowerCase()
        .replace(/&/g, "and")
        .replace(/[^a-z0-9\s-]/g, "")
        .trim()
        .replace(/\s+/g, "-")
        .replace(/-+/g, "-");
}

function headingsFromMarkdown(markdown) {
    return markdownHeadings(markdown)
        .map((heading) => cleanHeading(heading.raw))
        .filter(Boolean);
}

function contentPreview(markdown) {
    return markdown
        .replace(/```[^\n\r]*(?:\r?\n)([\s\S]*?)```/g, " $1 ")
        .replace(/~~~[^\n\r]*(?:\r?\n)([\s\S]*?)~~~/g, " $1 ")
        .replace(/\{%\s*(hint|tabs|tab|stepper|step|endhint|endtabs|endtab|endstepper|endstep)[^%]*%\}/g, " ")
        .replace(/<[^>]+>/g, " ")
        .replace(/!\[[^\]]*\]\([^)]+\)/g, " ")
        .replace(/\[([^\]]+)\]\([^)]+\)/g, "$1")
        .replace(/[#>*_`|{}]/g, " ")
        .replace(/\s+/g, " ")
        .trim()
        .slice(0, 500);
}

function sectionRecords(markdown, pageRecord) {
    const { body, headings } = markdownHeadings(markdown, true);
    const explicitAnchorCounts = new Map();
    for (const heading of headings) {
        const explicitAnchor = explicitAnchorFromHeading(heading.raw);
        if (explicitAnchor) {
            explicitAnchorCounts.set(explicitAnchor, (explicitAnchorCounts.get(explicitAnchor) || 0) + 1);
        }
    }

    const records = [];
    const usedAnchors = new Map();
    const pathStack = [];

    for (let i = 0; i < headings.length; i += 1) {
        const heading = headings[i];
        const next = headings[i + 1];
        const content = body.slice(heading.contentStart, next ? next.start : body.length);
        if (!heading.title) {
            continue;
        }

        while (pathStack.length > 0 && pathStack[pathStack.length - 1].depth >= heading.depth) {
            pathStack.pop();
        }
        pathStack.push({ depth: heading.depth, title: heading.title });

        const explicitAnchor = explicitAnchorFromHeading(heading.raw);
        const baseAnchor = explicitAnchor && explicitAnchorCounts.get(explicitAnchor) === 1
            ? explicitAnchor
            : slugifyHeading(heading.raw) || `section-${i + 1}`;
        const previousCount = usedAnchors.get(baseAnchor) || 0;
        usedAnchors.set(baseAnchor, previousCount + 1);
        const anchor = previousCount === 0 ? baseAnchor : `${baseAnchor}-${previousCount}`;

        records.push({
            type: "section",
            visibility: "public",
            source_path: pageRecord.source_path,
            parent_url: pageRecord.url,
            url: `${pageRecord.url}#${anchor}`,
            title: heading.title,
            page_title: pageRecord.title,
            section: pageRecord.section,
            section_path: pathStack.map((item) => item.title),
            anchor,
            depth: heading.depth,
            status: pageRecord.status,
            product: pageRecord.product,
            audience: pageRecord.audience,
            task: pageRecord.task,
            last_verified: pageRecord.last_verified,
            content_preview: contentPreview(content),
        });
    }

    return records;
}

function markdownHeadings(markdown, includeOffsets = false) {
    const body = stripFrontmatter(markdown);
    const lines = body.split(/\n/);
    const headings = [];
    let offset = 0;
    let inFence = false;
    let fenceMarker = "";

    for (const rawLine of lines) {
        const line = rawLine.replace(/\r$/, "");
        const fence = line.match(/^(```+|~~~+)/);
        if (fence) {
            const marker = fence[1][0];
            if (!inFence) {
                inFence = true;
                fenceMarker = marker;
            } else if (marker === fenceMarker) {
                inFence = false;
                fenceMarker = "";
            }
            offset += rawLine.length + 1;
            continue;
        }

        if (!inFence) {
            const match = line.match(/^(#{2,3})\s+(.+)$/);
            if (match) {
                const heading = {
                    depth: match[1].length,
                    raw: match[2],
                    title: cleanHeading(match[2]),
                };
                if (includeOffsets) {
                    heading.start = offset;
                    heading.contentStart = offset + rawLine.length + 1;
                }
                headings.push(heading);
            }
        }

        offset += rawLine.length + 1;
    }

    return includeOffsets ? { body, headings } : headings;
}

function buildIndex() {
    const entries = parseSummary();
    const seen = new Set();
    const records = [];

    for (const entry of entries) {
        const sourcePath = path.join(docsDir, entry.target);
        const relPath = path.relative(docsDir, sourcePath).replace(/\\/g, "/");
        if (seen.has(relPath)) {
            continue;
        }
        seen.add(relPath);

        if (!fs.existsSync(sourcePath)) {
            throw new Error(`SUMMARY.md references missing page: ${entry.target}`);
        }

        const markdown = readText(sourcePath);
        const frontmatter = parseFrontmatter(markdown);
        if (frontmatter.hidden === "true") {
            continue;
        }

        const pageRecord = {
            type: "page",
            visibility: "public",
            source_path: `docs/${relPath}`,
            url: pageUrl(relPath),
            title: titleFromMarkdown(markdown, entry.label),
            description: cleanFrontmatterValue(frontmatter.description),
            section: entry.section,
            status: cleanFrontmatterValue(frontmatter.status),
            product: cleanFrontmatterValue(frontmatter.product),
            audience: cleanFrontmatterValue(frontmatter.audience),
            task: cleanFrontmatterValue(frontmatter.task),
            last_verified: cleanFrontmatterValue(frontmatter.last_verified),
            headings: headingsFromMarkdown(markdown),
        };

        records.push(pageRecord);
        records.push(...sectionRecords(markdown, pageRecord));
    }

    return records;
}

const records = buildIndex();
fs.writeFileSync(outPath, `${records.map((record) => JSON.stringify(record)).join("\n")}\n`);
const pageCount = records.filter((record) => record.type === "page").length;
const sectionCount = records.filter((record) => record.type === "section").length;
console.log(`Wrote ${pageCount} page records and ${sectionCount} section records to ${path.relative(rootDir, outPath)}`);
