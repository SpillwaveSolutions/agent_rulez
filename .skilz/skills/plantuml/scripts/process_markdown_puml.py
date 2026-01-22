#!/usr/bin/env python3
"""
Enhanced PlantUML Markdown Processor

This script processes markdown files to extract and convert PlantUML diagrams to images.
It supports TWO methods of including PlantUML diagrams:

1. Embedded code blocks: ```puml ... ```
2. Linked .puml files: ![diagram](path/to/diagram.puml)

Both will be converted to image links: ![diagram](images/diagram.png)

This allows IDEs that support PlantUML to display diagrams during development,
while generating image-based markdown for publication (e.g., Confluence).

Usage:
    python process_markdown_puml.py article.md [--format png|svg] [--output-dir images/] [--validate]

Examples:
    # Process embedded and linked diagrams, convert to PNG
    python process_markdown_puml.py article.md

    # Convert to SVG format
    python process_markdown_puml.py article.md --format svg

    # Custom output directory
    python process_markdown_puml.py article.md --output-dir diagrams/

    # Validate PlantUML syntax without conversion
    python process_markdown_puml.py article.md --validate
"""

import argparse
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import List, Tuple, Optional


def find_plantuml_jar() -> Optional[str]:
    """Find plantuml.jar in common locations."""
    env_path = os.environ.get('PLANTUML_JAR')
    if env_path and os.path.isfile(env_path):
        return env_path

    search_paths = [
        './plantuml.jar',
        os.path.expanduser('~/plantuml.jar'),
        '/usr/local/bin/plantuml.jar',
        '/usr/share/plantuml/plantuml.jar',
    ]

    for path in search_paths:
        if os.path.isfile(path):
            return path

    return None


def validate_puml_syntax(puml_content: str, plantuml_jar: str) -> Tuple[bool, str]:
    """
    Validate PlantUML syntax without generating output.

    Returns:
        Tuple of (is_valid, error_message)
    """
    with tempfile.NamedTemporaryFile(mode='w', suffix='.puml', delete=False) as tmp:
        tmp.write(puml_content)
        tmp_path = tmp.name

    try:
        result = subprocess.run(
            ['java', '-jar', plantuml_jar, '-syntax', tmp_path],
            capture_output=True,
            text=True,
            timeout=10
        )

        os.unlink(tmp_path)

        # PlantUML returns 0 for valid syntax
        if result.returncode == 0:
            return True, "Syntax OK"
        else:
            return False, result.stderr or result.stdout

    except subprocess.TimeoutExpired:
        os.unlink(tmp_path)
        return False, "Validation timeout"
    except Exception as e:
        if os.path.exists(tmp_path):
            os.unlink(tmp_path)
        return False, f"Validation error: {str(e)}"


def convert_puml_to_image(puml_content: str, output_path: str, image_format: str, plantuml_jar: str) -> bool:
    """
    Convert PlantUML content to image file.

    Args:
        puml_content: PlantUML diagram source
        output_path: Path for output image (without extension)
        image_format: 'png' or 'svg'
        plantuml_jar: Path to plantuml.jar

    Returns:
        True if conversion successful
    """
    # Create temporary .puml file
    with tempfile.NamedTemporaryFile(mode='w', suffix='.puml', delete=False) as tmp:
        tmp.write(puml_content)
        tmp_path = tmp.name

    try:
        # Build command
        cmd = ['java', '-jar', plantuml_jar]
        if image_format == 'svg':
            cmd.append('-tsvg')
        else:
            cmd.append('-tpng')

        cmd.extend(['-o', os.path.dirname(output_path), tmp_path])

        # Run PlantUML
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=30
        )

        # PlantUML generates output with the temp filename
        tmp_output = tmp_path.replace('.puml', f'.{image_format}')
        expected_output = f"{output_path}.{image_format}"

        # Move to desired location
        if os.path.exists(tmp_output):
            os.rename(tmp_output, expected_output)
            os.unlink(tmp_path)
            return True
        else:
            print(f"❌ Conversion failed: {result.stderr}", file=sys.stderr)
            os.unlink(tmp_path)
            return False

    except subprocess.TimeoutExpired:
        print(f"❌ Conversion timeout for {output_path}", file=sys.stderr)
        if os.path.exists(tmp_path):
            os.unlink(tmp_path)
        return False
    except Exception as e:
        print(f"❌ Conversion error: {e}", file=sys.stderr)
        if os.path.exists(tmp_path):
            os.unlink(tmp_path)
        return False


def extract_embedded_puml_blocks(content: str) -> List[Tuple[str, int, int]]:
    """
    Extract embedded ```puml code blocks from markdown.

    Returns:
        List of tuples: (puml_content, start_pos, end_pos)
    """
    pattern = r'```puml\s*\n(.*?)```'
    blocks = []

    for match in re.finditer(pattern, content, re.DOTALL):
        puml_content = match.group(1).strip()
        blocks.append((puml_content, match.start(), match.end()))

    return blocks


def extract_linked_puml_files(content: str, markdown_dir: Path) -> List[Tuple[str, str, int, int]]:
    """
    Extract linked .puml files from markdown: ![alt](path/to/file.puml)

    Returns:
        List of tuples: (puml_content, original_link, start_pos, end_pos)
    """
    # Match: ![anything](path.puml) or ![anything](path.puml "title")
    pattern = r'!\[([^\]]*)\]\(([^)\s]+\.puml)(?:\s+"[^"]*")?\)'
    links = []

    for match in re.finditer(pattern, content):
        puml_path = match.group(2)
        # Resolve relative to markdown file location
        full_path = (markdown_dir / puml_path).resolve()

        if full_path.exists():
            with open(full_path, 'r', encoding='utf-8') as f:
                puml_content = f.read()
            links.append((puml_content, match.group(0), match.start(), match.end()))
        else:
            print(f"⚠️  Warning: Linked .puml file not found: {puml_path}", file=sys.stderr)

    return links


def detect_diagram_type(puml_content: str) -> str:
    """Detect diagram type from @start tag."""
    match = re.search(r'@start(\w+)', puml_content)
    if match:
        return match.group(1).lower()
    return 'uml'


def process_markdown(
    markdown_path: Path,
    output_dir: Path,
    image_format: str,
    plantuml_jar: str,
    validate_only: bool = False
) -> Tuple[str, int, int]:
    """
    Process markdown file, converting all PlantUML diagrams to images.

    Returns:
        Tuple of (new_markdown_content, diagrams_processed, validation_errors)
    """
    with open(markdown_path, 'r', encoding='utf-8') as f:
        content = f.read()

    markdown_dir = markdown_path.parent
    output_dir.mkdir(parents=True, exist_ok=True)

    # Collect all diagrams (embedded and linked)
    all_diagrams = []

    # Extract embedded code blocks
    embedded = extract_embedded_puml_blocks(content)
    for puml_content, start, end in embedded:
        all_diagrams.append({
            'type': 'embedded',
            'content': puml_content,
            'start': start,
            'end': end,
            'original': content[start:end]
        })

    # Extract linked .puml files
    linked = extract_linked_puml_files(content, markdown_dir)
    for puml_content, original_link, start, end in linked:
        all_diagrams.append({
            'type': 'linked',
            'content': puml_content,
            'start': start,
            'end': end,
            'original': original_link
        })

    # Sort by position (process from end to start to maintain positions)
    all_diagrams.sort(key=lambda x: x['start'], reverse=True)

    if not all_diagrams:
        print("ℹ️  No PlantUML diagrams found (embedded or linked)")
        return content, 0, 0

    print(f"📊 Found {len(all_diagrams)} PlantUML diagram(s)")

    validation_errors = 0
    diagrams_processed = 0

    # Process each diagram
    for idx, diagram in enumerate(all_diagrams, 1):
        puml_content = diagram['content']
        diagram_type = detect_diagram_type(puml_content)

        # Validate syntax
        is_valid, error_msg = validate_puml_syntax(puml_content, plantuml_jar)

        if not is_valid:
            print(f"❌ Diagram {idx} - Syntax error: {error_msg}", file=sys.stderr)
            validation_errors += 1
            continue
        else:
            print(f"✅ Diagram {idx} - Syntax valid ({diagram_type})")

        if validate_only:
            continue

        # Generate output filename
        output_name = f"diagram_{idx}_{diagram_type}"
        output_path = output_dir / output_name

        # Convert to image
        success = convert_puml_to_image(
            puml_content,
            str(output_path),
            image_format,
            plantuml_jar
        )

        if success:
            # Replace in content (working backwards)
            relative_image_path = f"{output_dir.name}/{output_name}.{image_format}"
            image_link = f"![{output_name}]({relative_image_path})"

            content = content[:diagram['start']] + image_link + content[diagram['end']:]
            diagrams_processed += 1
            print(f"✅ Converted diagram {idx} → {relative_image_path}")
        else:
            print(f"❌ Failed to convert diagram {idx}", file=sys.stderr)

    return content, diagrams_processed, validation_errors


def main():
    parser = argparse.ArgumentParser(
        description='Process markdown files with PlantUML diagrams (embedded and linked)',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__
    )
    parser.add_argument('markdown_file', type=str, help='Markdown file to process')
    parser.add_argument(
        '--format',
        choices=['png', 'svg'],
        default='png',
        help='Output image format (default: png)'
    )
    parser.add_argument(
        '--output-dir',
        type=str,
        default='images',
        help='Output directory for images (default: images/)'
    )
    parser.add_argument(
        '--validate',
        action='store_true',
        help='Validate PlantUML syntax without converting'
    )

    args = parser.parse_args()

    # Check inputs
    markdown_path = Path(args.markdown_file)
    if not markdown_path.exists():
        print(f"❌ Error: Markdown file not found: {args.markdown_file}", file=sys.stderr)
        sys.exit(1)

    # Find plantuml.jar
    plantuml_jar = find_plantuml_jar()
    if not plantuml_jar:
        print("❌ Error: plantuml.jar not found", file=sys.stderr)
        print("   Download from: https://plantuml.com/download", file=sys.stderr)
        print("   Place in ~/plantuml.jar or set PLANTUML_JAR env variable", file=sys.stderr)
        sys.exit(1)

    print(f"📄 Processing: {markdown_path}")
    print(f"🔧 PlantUML: {plantuml_jar}")

    if args.validate:
        print("🔍 Validation mode (no conversion)")

    output_dir = markdown_path.parent / args.output_dir

    # Process markdown
    new_content, processed, errors = process_markdown(
        markdown_path,
        output_dir,
        args.format,
        plantuml_jar,
        args.validate
    )

    # Save result
    if not args.validate and processed > 0:
        output_path = markdown_path.with_stem(f"{markdown_path.stem}_with_images")
        with open(output_path, 'w', encoding='utf-8') as f:
            f.write(new_content)

        print(f"\n✅ Success!")
        print(f"   Processed: {processed} diagram(s)")
        print(f"   Output: {output_path}")
        print(f"   Images: {output_dir}/")
    elif args.validate:
        print(f"\n🔍 Validation complete:")
        print(f"   Total diagrams: {processed + errors}")
        print(f"   Valid: {processed}")
        print(f"   Errors: {errors}")

        if errors > 0:
            sys.exit(1)
    else:
        print("\n⚠️  No diagrams were converted")
        sys.exit(1)


if __name__ == '__main__':
    main()
