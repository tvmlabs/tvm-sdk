function frontmatterMatch(markdown) {
    if (!markdown.startsWith("---\n") && !markdown.startsWith("---\r\n")) {
        return null;
    }

    const start = markdown.startsWith("---\r\n") ? 5 : 4;
    const closing = /\r?\n---[ \t]*(?:\r?\n|$)/g;
    closing.lastIndex = start;
    const match = closing.exec(markdown);
    if (!match) {
        return null;
    }

    return {
        bodyStart: start,
        bodyEnd: match.index,
        end: match.index + match[0].length,
    };
}

function parseFrontmatter(markdown) {
    const match = frontmatterMatch(markdown);
    if (!match) {
        return {};
    }

    const data = {};
    let currentKey = null;
    for (const line of markdown.slice(match.bodyStart, match.bodyEnd).split(/\r?\n/)) {
        const field = line.match(/^([A-Za-z0-9_-]+):\s*(.*)$/);
        if (field) {
            currentKey = field[1];
            data[currentKey] = field[2].trim();
            continue;
        }
        if (currentKey && (/^\s+/.test(line) || line.trim() === "")) {
            data[currentKey] = `${data[currentKey]} ${line.trim()}`.trim();
        }
    }

    return data;
}

function cleanFrontmatterValue(value) {
    if (!value) {
        return "";
    }
    return String(value)
        .replace(/^(?:>[-+]?|\|[-+]?)\s*/, "")
        .replace(/^["']|["']$/g, "")
        .replace(/\s+/g, " ")
        .trim();
}

function stripFrontmatter(markdown) {
    const match = frontmatterMatch(markdown);
    return match ? markdown.slice(match.end) : markdown;
}

module.exports = {
    cleanFrontmatterValue,
    parseFrontmatter,
    stripFrontmatter,
};
