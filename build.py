#!/usr/bin/env python3
"""
Build static HTML pages from markdown command pages.
"""

import os
import json
import re

PAGES_DIR = "pages"
DOCS_DIR = "docs"

def parse_markdown_to_html(content):
    """Simple markdown to HTML converter for our page format."""
    lines = content.split('\n')
    html_parts = []
    in_flags = False
    in_exit_codes = False
    in_errors = False

    for line in lines:
        # Headers
        if line.startswith('# '):
            html_parts.append(f'<h1>{line[2:]}</h1>')
        elif line.startswith('## '):
            section = line[3:]
            in_flags = 'flag' in section.lower()
            in_exit_codes = 'exit' in section.lower()
            in_errors = 'error' in section.lower()
            html_parts.append(f'<h2>{section}</h2>')

        # Blockquotes (summary, see also, keywords)
        elif line.startswith('> '):
            text = line[2:]
            if text.startswith('See also:'):
                refs = text[9:].strip()
                # Convert command names to links
                linked = re.sub(r'`?(\w[\w-]*)`?', r'<a href="\1.html">\1</a>', refs)
                html_parts.append(f'<p class="see-also">See also: {linked}</p>')
            elif text.startswith('Keywords:'):
                html_parts.append(f'<p class="keywords">{text}</p>')
            else:
                html_parts.append(f'<p class="summary">{text}</p>')

        # Example descriptions
        elif line.startswith('- ') and not (in_flags or in_exit_codes or in_errors):
            html_parts.append(f'<p class="example-desc">{line[2:]}</p>')

        # Code blocks
        elif line.startswith('`') and line.endswith('`') and len(line) > 2:
            code = line[1:-1]
            html_parts.append(f'<pre><code>{code}</code></pre>')

        # Flag/exit code/error list items
        elif line.startswith('- ') and (in_flags or in_exit_codes or in_errors):
            item = line[2:]
            # Parse `flag`: description format
            match = re.match(r'`([^`]+)`[:\s]+(.+)', item)
            if match:
                html_parts.append(f'<li><code>{match.group(1)}</code> {match.group(2)}</li>')
            else:
                html_parts.append(f'<li>{item}</li>')

        # Empty lines
        elif line.strip() == '':
            pass

        # Other text
        else:
            # Convert inline code
            line = re.sub(r'`([^`]+)`', r'<code>\1</code>', line)
            html_parts.append(f'<p>{line}</p>')

    return '\n'.join(html_parts)

def generate_page_html(name, content):
    """Generate full HTML page for a command."""
    body = parse_markdown_to_html(content)

    return f'''<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{name} - idk</title>
    <link rel="stylesheet" href="style.css">
</head>
<body>
    <div class="container">
        <nav>
            <a href="index.html" class="back-link">&larr; All commands</a>
            <span class="brand">idk</span>
        </nav>

        <main class="page-content">
            {body}
        </main>

        <footer>
            <p>
                <a href="https://github.com/anthropics/idk">GitHub</a> ·
                Install: <code>cargo install idk</code>
            </p>
        </footer>
    </div>
</body>
</html>
'''

def generate_index_html(pages):
    """Generate the index page with all commands."""
    cards = []
    for p in pages:
        cards.append(f'''
            <a href="{p['name']}.html" class="command-card">
                <h3>{p['name']}</h3>
                <p>{p['summary']}</p>
            </a>''')

    cards_html = '\n'.join(cards)

    return f'''<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>idk - man pages for agents</title>
    <link rel="stylesheet" href="style.css">
</head>
<body>
    <div class="container">
        <header>
            <h1>idk</h1>
            <p class="tagline">man pages for agents</p>
        </header>

        <section class="about">
            <p><strong>idk</strong> is a command reference designed for AI agents. Unlike traditional man pages, idk pages prioritize:</p>
            <ul>
                <li><strong>Examples first</strong> &mdash; real commands you can run immediately</li>
                <li><strong>Exit codes</strong> &mdash; know what 0, 1, 127 actually mean</li>
                <li><strong>Common errors</strong> &mdash; what goes wrong and how to fix it</li>
                <li><strong>Semantic search</strong> &mdash; find commands by what you want to do</li>
            </ul>

            <h3>Installation</h3>
            <pre><code>cargo install idk</code></pre>

            <h3>Agent Usage</h3>
            <pre><code># Look up a specific command
idk curl

# Search by what you want to do
idk "download a file"
idk "find large files"

# Get JSON output for parsing
idk curl --json

# List all available pages
idk --list</code></pre>

            <p>Add to your agent's system prompt:</p>
            <pre><code>When you need help with a CLI command, run: idk &lt;command&gt;
When you're not sure which command to use, run: idk "&lt;what you want to do&gt;"</code></pre>
        </section>

        <div class="search-container">
            <input type="text" id="search" placeholder="Search commands..." autofocus>
        </div>

        <main>
            <div id="results" class="command-grid">
                {cards_html}
            </div>
        </main>

        <footer>
            <p>
                <a href="https://github.com/anthropics/idk">GitHub</a> ·
                Install: <code>cargo install idk</code>
            </p>
        </footer>
    </div>

    <script>
        const cards = document.querySelectorAll('.command-card');
        const search = document.getElementById('search');

        search.addEventListener('input', (e) => {{
            const query = e.target.value.toLowerCase();
            cards.forEach(card => {{
                const name = card.querySelector('h3').textContent.toLowerCase();
                const desc = card.querySelector('p').textContent.toLowerCase();
                const match = name.includes(query) || desc.includes(query);
                card.style.display = match ? '' : 'none';
            }});
        }});
    </script>
</body>
</html>
'''

def extract_summary(content):
    """Extract summary from markdown content."""
    for line in content.split('\n'):
        if line.startswith('> ') and not line.startswith('> See also') and not line.startswith('> Keywords'):
            return line[2:].strip()
    return ""

def main():
    # Read all markdown pages
    pages_info = []

    for filename in sorted(os.listdir(PAGES_DIR)):
        if filename.endswith('.md'):
            name = filename[:-3]
            filepath = os.path.join(PAGES_DIR, filename)

            with open(filepath, 'r') as f:
                content = f.read()

            # Generate individual page
            html = generate_page_html(name, content)
            output_path = os.path.join(DOCS_DIR, f"{name}.html")

            with open(output_path, 'w') as f:
                f.write(html)

            print(f"Generated {name}.html")

            # Collect info for index
            pages_info.append({
                'name': name,
                'summary': extract_summary(content)
            })

    # Generate index
    index_html = generate_index_html(pages_info)
    with open(os.path.join(DOCS_DIR, 'index.html'), 'w') as f:
        f.write(index_html)

    print(f"\nGenerated index.html with {len(pages_info)} commands")

if __name__ == '__main__':
    main()
