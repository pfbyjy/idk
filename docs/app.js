// Page index with summaries for search
const pages = [
    { name: "awk", summary: "Pattern scanning and text processing", keywords: ["columns", "fields", "text", "csv"] },
    { name: "cat", summary: "Concatenate and print files", keywords: ["print", "display", "file", "contents"] },
    { name: "cd", summary: "Change the current working directory", keywords: ["change", "directory", "navigate"] },
    { name: "chmod", summary: "Change file mode (permissions)", keywords: ["permissions", "mode", "access"] },
    { name: "chown", summary: "Change file owner and group", keywords: ["owner", "permissions", "group"] },
    { name: "cp", summary: "Copy files and directories", keywords: ["copy", "duplicate", "files"] },
    { name: "curl", summary: "Transfer data to/from a server", keywords: ["http", "https", "request", "api"] },
    { name: "cut", summary: "Remove sections from each line", keywords: ["columns", "fields", "extract"] },
    { name: "df", summary: "Report file system disk space usage", keywords: ["disk", "space", "usage"] },
    { name: "dig", summary: "DNS lookup utility", keywords: ["dns", "lookup", "domain"] },
    { name: "docker", summary: "Container platform", keywords: ["container", "image", "build"] },
    { name: "du", summary: "Estimate file and directory space usage", keywords: ["disk", "usage", "size"] },
    { name: "find", summary: "Search for files in a directory hierarchy", keywords: ["search", "files", "locate"] },
    { name: "free", summary: "Display amount of free and used memory", keywords: ["memory", "ram", "usage"] },
    { name: "gh", summary: "GitHub CLI - work with GitHub from the command line", keywords: ["github", "pull request", "issue", "pr"] },
    { name: "git", summary: "Distributed version control system", keywords: ["version", "control", "commit"] },
    { name: "grep", summary: "Search for patterns in files", keywords: ["search", "pattern", "regex"] },
    { name: "head", summary: "Output the first part of files", keywords: ["first", "lines", "beginning"] },
    { name: "jq", summary: "Command-line JSON processor", keywords: ["json", "parse", "filter"] },
    { name: "kill", summary: "Send signals to processes", keywords: ["terminate", "stop", "signal"] },
    { name: "less", summary: "View file contents with pagination", keywords: ["view", "read", "paginate"] },
    { name: "ln", summary: "Create links between files", keywords: ["link", "symlink", "symbolic"] },
    { name: "ls", summary: "List directory contents", keywords: ["list", "files", "directory"] },
    { name: "make", summary: "Build automation tool", keywords: ["build", "compile", "makefile"] },
    { name: "mkdir", summary: "Create directories", keywords: ["make", "create", "directory"] },
    { name: "mv", summary: "Move or rename files and directories", keywords: ["move", "rename", "relocate"] },
    { name: "nc", summary: "Netcat - networking Swiss Army knife", keywords: ["network", "tcp", "port"] },
    { name: "npm", summary: "Node Package Manager", keywords: ["node", "javascript", "package"] },
    { name: "ping", summary: "Send ICMP ECHO_REQUEST to network hosts", keywords: ["network", "test", "connectivity"] },
    { name: "pip", summary: "Python package installer", keywords: ["python", "package", "install"] },
    { name: "ps", summary: "Report process status", keywords: ["process", "running", "list"] },
    { name: "rm", summary: "Remove files and directories", keywords: ["remove", "delete", "files"] },
    { name: "rsync", summary: "Fast, versatile file synchronization", keywords: ["sync", "copy", "transfer"] },
    { name: "scp", summary: "Secure copy - transfer files over SSH", keywords: ["copy", "transfer", "remote"] },
    { name: "sed", summary: "Stream editor for filtering and transforming text", keywords: ["replace", "substitute", "text"] },
    { name: "sort", summary: "Sort lines of text files", keywords: ["sort", "order", "alphabetical"] },
    { name: "ssh", summary: "Secure Shell - connect to remote machines", keywords: ["remote", "connect", "shell"] },
    { name: "sudo", summary: "Execute a command as another user", keywords: ["root", "admin", "superuser"] },
    { name: "tail", summary: "Output the last part of files", keywords: ["last", "lines", "logs"] },
    { name: "tar", summary: "Archive files", keywords: ["archive", "compress", "extract"] },
    { name: "top", summary: "Display real-time system processes", keywords: ["process", "monitor", "cpu"] },
    { name: "touch", summary: "Create empty files or update timestamps", keywords: ["create", "file", "empty"] },
    { name: "tr", summary: "Translate or delete characters", keywords: ["translate", "replace", "characters"] },
    { name: "uniq", summary: "Report or filter out repeated lines", keywords: ["unique", "duplicate", "repeated"] },
    { name: "wc", summary: "Word, line, character, and byte count", keywords: ["count", "lines", "words"] },
    { name: "wget", summary: "Download files from the web", keywords: ["download", "http", "web"] },
    { name: "xargs", summary: "Build and execute commands from stdin", keywords: ["pipe", "arguments", "batch"] }
];

const searchInput = document.getElementById('search');
const resultsDiv = document.getElementById('results');
const pageContent = document.getElementById('page-content');

// Base URL for fetching pages
const PAGES_BASE = 'pages';

function renderCommandGrid(commands) {
    if (commands.length === 0) {
        resultsDiv.innerHTML = '<div class="no-results">No commands found. Try different keywords.</div>';
        return;
    }

    resultsDiv.innerHTML = commands.map(cmd => `
        <a href="#${cmd.name}" class="command-card" onclick="loadPage('${cmd.name}'); return false;">
            <h3>${cmd.name}</h3>
            <p>${cmd.summary}</p>
        </a>
    `).join('');
}

function searchCommands(query) {
    if (!query.trim()) {
        return pages;
    }

    const terms = query.toLowerCase().split(/\s+/);

    return pages
        .map(cmd => {
            let score = 0;
            const name = cmd.name.toLowerCase();
            const summary = cmd.summary.toLowerCase();
            const keywords = cmd.keywords.join(' ').toLowerCase();

            for (const term of terms) {
                if (name === term) score += 100;
                else if (name.includes(term)) score += 50;
                if (keywords.includes(term)) score += 30;
                if (summary.includes(term)) score += 10;
            }

            return { ...cmd, score };
        })
        .filter(cmd => cmd.score > 0)
        .sort((a, b) => b.score - a.score);
}

async function loadPage(name) {
    try {
        const response = await fetch(`${PAGES_BASE}/${name}.md`);
        if (!response.ok) throw new Error('Page not found');

        const markdown = await response.text();
        const html = marked.parse(markdown);

        pageContent.innerHTML = `
            <a href="#" class="back-link" onclick="showGrid(); return false;">&larr; Back to all commands</a>
            ${html}
        `;

        resultsDiv.classList.add('hidden');
        pageContent.classList.remove('hidden');

        window.location.hash = name;
    } catch (error) {
        pageContent.innerHTML = `
            <a href="#" class="back-link" onclick="showGrid(); return false;">&larr; Back to all commands</a>
            <h1>Page not found</h1>
            <p>Could not load the page for "${name}".</p>
        `;
        resultsDiv.classList.add('hidden');
        pageContent.classList.remove('hidden');
    }
}

function showGrid() {
    pageContent.classList.add('hidden');
    resultsDiv.classList.remove('hidden');
    window.location.hash = '';
    searchInput.focus();
}

// Event listeners
searchInput.addEventListener('input', (e) => {
    const results = searchCommands(e.target.value);
    renderCommandGrid(results);
    showGrid();
});

// Handle back/forward navigation
window.addEventListener('hashchange', () => {
    const hash = window.location.hash.slice(1);
    if (hash) {
        loadPage(hash);
    } else {
        showGrid();
    }
});

// Initial render
function init() {
    renderCommandGrid(pages);

    // Check for hash on load
    const hash = window.location.hash.slice(1);
    if (hash) {
        loadPage(hash);
    }
}

init();
