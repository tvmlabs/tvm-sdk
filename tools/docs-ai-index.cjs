#!/usr/bin/env node

const fs = require("fs");
const path = require("path");

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

function parseFrontmatter(markdown) {
    if (!markdown.startsWith("---\n")) {
        return {};
    }
    const end = markdown.indexOf("\n---", 4);
    if (end === -1) {
        return {};
    }
    const body = markdown.slice(4, end).split(/\r?\n/);
    const data = {};
    let currentKey = null;
    for (const line of body) {
        const match = line.match(/^([A-Za-z0-9_-]+):\s*(.*)$/);
        if (match) {
            currentKey = match[1];
            data[currentKey] = match[2].trim();
            continue;
        }
        if (currentKey && /^\s+/.test(line)) {
            data[currentKey] = `${data[currentKey]} ${line.trim()}`.trim();
        }
    }
    return data;
}

function cleanFrontmatterValue(value) {
    if (!value) {
        return "";
    }
    return value
        .replace(/^>-\s*/, "")
        .replace(/^["']|["']$/g, "")
        .replace(/\s+/g, " ")
        .trim();
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
        .replace(/\s*<a\s+[^>]+><\/a>\s*/g, "")
        .replace(/[*_`]/g, "")
        .replace(/[^\x20-\x7E]/g, "")
        .replace(/\s+/g, " ")
        .trim();
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
    const headings = [];
    const re = /^#{2,3}\s+(.+)$/gm;
    let match;
    while ((match = re.exec(markdown)) !== null) {
        const text = cleanHeading(match[1]);
        if (text && headings.length < 20) {
            headings.push(text);
        }
    }
    return headings;
}

function stripFrontmatter(markdown) {
    if (!markdown.startsWith("---\n")) {
        return markdown;
    }
    const end = markdown.indexOf("\n---", 4);
    if (end === -1) {
        return markdown;
    }
    return markdown.slice(end + 4);
}

function contentPreview(markdown) {
    return markdown
        .replace(/```[\s\S]*?```/g, " ")
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
    const body = stripFrontmatter(markdown);
    const headingRe = /^(#{2,3})\s+(.+)$/gm;
    const headings = [];
    let match;
    while ((match = headingRe.exec(body)) !== null) {
        headings.push({
            depth: match[1].length,
            raw: match[2],
            title: cleanHeading(match[2]),
            start: match.index,
            contentStart: headingRe.lastIndex,
        });
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

        const baseAnchor = slugifyHeading(heading.raw) || `section-${i + 1}`;
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
