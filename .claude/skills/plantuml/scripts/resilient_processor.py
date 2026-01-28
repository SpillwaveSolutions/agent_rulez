#!/usr/bin/env python3
"""
Resilient PlantUML processor implementing the 4-step workflow.

Workflow:
1. Identify diagram type and load reference
2. Create file with structured naming
3. Convert with error handling and retry
4. Validate and integrate into markdown

Usage:
    python resilient_processor.py article.md --format png
    python resilient_processor.py diagram.puml --format svg
    python resilient_processor.py article.md --validate-only
"""

import sys
import os
import re
import json
import argparse
import subprocess
import shutil
from pathlib import Path
from datetime import datetime
from dataclasses import dataclass, field
from typing import List, Tuple, Optional, Dict
from glob import glob

# Get the script directory for relative imports
SCRIPT_DIR = Path(__file__).parent
SKILL_ROOT = SCRIPT_DIR.parent
REFERENCES_PATH = SKILL_ROOT / 'references'
TROUBLESHOOTING_PATH = REFERENCES_PATH / 'troubleshooting'


@dataclass
class ErrorResolution:
    """Result of error handling process."""
    original_error: str
    guide_loaded: Optional[str] = None
    error_number: Optional[int] = None
    suggested_fix: Optional[str] = None
    fixed_content: Optional[str] = None
    search_queries: List[str] = field(default_factory=list)
    resolved: bool = False


@dataclass
class ProcessingResult:
    """Result of processing a single diagram."""
    diagram_type: Optional[str] = None
    reference_loaded: Optional[str] = None
    puml_path: Optional[Path] = None
    image_path: Optional[Path] = None
    markdown_link: Optional[str] = None
    conversion_success: bool = False
    validation_error: Optional[str] = None
    errors: List[ErrorResolution] = field(default_factory=list)
    external_search_needed: bool = False
    search_queries: List[str] = field(default_factory=list)


class DiagramTypeIdentifier:
    """Identifies PlantUML diagram type and loads appropriate reference."""

    # Map @start tags to diagram types
    START_TAG_MAPPING = {
        '@startuml': 'uml',
        '@startmindmap': 'mindmap',
        '@startgantt': 'gantt',
        '@startsalt': 'salt',
        '@startjson': 'json',
        '@startyaml': 'yaml',
        '@startditaa': 'ditaa',
        '@startwbs': 'wbs',
        '@startnwdiag': 'network',
    }

    # Keywords to identify UML subtypes
    UML_KEYWORDS = {
        'sequence': ['participant', 'actor', '->', '-->',  'autonumber', 'activate'],
        'class': ['class ', 'interface ', 'abstract ', 'extends', 'implements', '<|--'],
        'activity': ['start', 'stop', ':.*:', 'if ', 'while ', 'fork', 'partition'],
        'state': ['state ', '[*]', '-->', 'state "'],
        'component': ['component ', 'package ', '[', ']', 'interface '],
        'deployment': ['node ', 'database ', 'cloud ', 'artifact '],
        'usecase': ['usecase ', 'actor ', '(', ')'],
        'object': ['object ', 'map '],
        'timing': ['clock', 'concise', 'robust', '@'],
    }

    # Intent keywords for natural language
    INTENT_KEYWORDS = {
        'sequence': ['sequence', 'interaction', 'api call', 'message', 'request', 'response'],
        'class': ['class', 'oop', 'inheritance', 'interface', 'object-oriented'],
        'er': ['entity', 'database', 'schema', 'table', 'relationship', 'erd'],
        'activity': ['workflow', 'process', 'flowchart', 'decision', 'activity'],
        'state': ['state', 'transition', 'state machine', 'fsm'],
        'component': ['component', 'module', 'architecture', 'system'],
        'deployment': ['deployment', 'server', 'infrastructure', 'cloud', 'docker'],
        'gantt': ['gantt', 'timeline', 'schedule', 'project plan'],
        'mindmap': ['mindmap', 'brainstorm', 'ideas', 'mind map'],
        'usecase': ['use case', 'actor', 'feature', 'user story'],
        'network': ['network', 'topology', 'lan', 'wan'],
        'wbs': ['wbs', 'work breakdown', 'hierarchy'],
        'json': ['json', 'data structure'],
        'yaml': ['yaml', 'config'],
        'salt': ['wireframe', 'ui', 'mockup', 'interface'],
    }

    def __init__(self, references_path: Path = REFERENCES_PATH):
        self.references_path = references_path

    def identify_from_content(self, puml_content: str) -> str:
        """Identify diagram type from PlantUML source code."""
        content_lower = puml_content.lower()

        # Check @start tag first
        for tag, diagram_type in self.START_TAG_MAPPING.items():
            if tag in content_lower:
                if diagram_type == 'uml':
                    return self._identify_uml_subtype(puml_content)
                return diagram_type

        # Default to sequence if @startuml with no clear type
        return 'sequence'

    def _identify_uml_subtype(self, content: str) -> str:
        """Identify specific UML diagram type from content."""
        content_lower = content.lower()

        scores = {dtype: 0 for dtype in self.UML_KEYWORDS}
        for dtype, keywords in self.UML_KEYWORDS.items():
            for keyword in keywords:
                if keyword in content_lower:
                    scores[dtype] += 1

        # Return type with highest score, default to sequence
        best_type = max(scores, key=scores.get)
        return best_type if scores[best_type] > 0 else 'sequence'

    def identify_from_intent(self, user_intent: str) -> str:
        """Identify diagram type from natural language description."""
        intent_lower = user_intent.lower()

        for dtype, keywords in self.INTENT_KEYWORDS.items():
            for keyword in keywords:
                if keyword in intent_lower:
                    return dtype

        return 'sequence'  # Default

    def get_reference_path(self, diagram_type: str) -> Path:
        """Return path to appropriate reference file."""
        # Map diagram types to reference files
        ref_map = {
            'sequence': 'sequence_diagrams.md',
            'class': 'class_diagrams.md',
            'activity': 'activity_diagrams.md',
            'state': 'state_diagrams.md',
            'component': 'component_diagrams.md',
            'deployment': 'deployment_diagrams.md',
            'usecase': 'use_case_diagrams.md',
            'object': 'object_diagrams.md',
            'timing': 'timing_diagrams.md',
            'er': 'er_diagrams.md',
            'gantt': 'gantt_diagrams.md',
            'mindmap': 'mindmap_diagrams.md',
            'network': 'network_diagrams.md',
            'wbs': 'wbs_diagrams.md',
            'json': 'json_yaml_diagrams.md',
            'yaml': 'json_yaml_diagrams.md',
            'salt': 'wireframes_salt.md',
            'ditaa': 'ditaa_diagrams.md',
        }

        ref_file = ref_map.get(diagram_type, 'toc.md')
        return self.references_path / ref_file


class FileNamingConvention:
    """Implements structured naming convention for diagram files."""

    def __init__(self, base_dir: Path = None):
        self.base_dir = base_dir or Path('.')
        self.diagrams_dir = self.base_dir / 'diagrams'

    def ensure_directory(self) -> Path:
        """Create diagrams directory if it doesn't exist."""
        self.diagrams_dir.mkdir(parents=True, exist_ok=True)
        return self.diagrams_dir

    def generate_filename(
        self,
        markdown_file: str,
        diagram_num: int,
        diagram_type: str,
        title: Optional[str] = None
    ) -> str:
        """Generate structured filename following convention."""
        # Sanitize markdown filename
        md_name = self._sanitize(markdown_file.replace('.md', ''))

        # Zero-pad diagram number
        num_padded = f"{diagram_num:03d}"

        # Sanitize title (first 10 chars)
        title_segment = self._sanitize(title or 'diagram')[:10]

        return f"{md_name}_{num_padded}_{diagram_type}_{title_segment}"

    def _sanitize(self, text: str) -> str:
        """Remove special chars, lowercase, replace spaces with underscores."""
        sanitized = re.sub(r'[^a-zA-Z0-9\s_-]', '', text)
        return sanitized.lower().replace(' ', '_').replace('-', '_')

    def get_full_path(self, filename: str, extension: str = 'puml') -> Path:
        """Return full path to diagram file."""
        return self.diagrams_dir / f"{filename}.{extension}"


class ErrorHandler:
    """Handles PlantUML conversion errors with troubleshooting guide integration."""

    # Error patterns mapped to troubleshooting guides
    ERROR_PATTERNS = {
        r'cannot find java': ('installation_setup_guide.md', 1),
        r'unable to access jarfile': ('installation_setup_guide.md', 4),
        r'headlessexception': ('installation_setup_guide.md', 6),
        r'no dot executable': ('installation_setup_guide.md', 2),
        r'graphviz has crashed': ('installation_setup_guide.md', 3),
        r'no @startuml.*found': ('general_syntax_guide.md', 1),
        r'syntax error': ('general_syntax_guide.md', None),
        r'duplicate participant': ('sequence_diagrams_guide.md', 3),
        r'empty alt group': ('sequence_diagrams_guide.md', 11),
        r'cannot include file': ('preprocessor_includes_guide.md', 1),
        r'file already included': ('preprocessor_includes_guide.md', 5),
        r'stack overflow': ('preprocessor_includes_guide.md', 8),
        r'failed to generate image': ('image_generation_guide.md', 1),
        r'nullpointerexception': ('general_syntax_guide.md', 15),
    }

    def __init__(self, troubleshooting_path: Path = TROUBLESHOOTING_PATH):
        self.troubleshooting_path = troubleshooting_path
        self.max_retries = 3

    def handle_error(
        self,
        error_output: str,
        puml_content: str,
        diagram_type: Optional[str] = None
    ) -> ErrorResolution:
        """Main error handling entry point."""
        resolution = ErrorResolution(original_error=error_output)

        # Classify error
        guide, error_num = self._classify_error(error_output)

        if guide:
            resolution.guide_loaded = guide
            resolution.error_number = error_num

            # Load guide content (for logging)
            guide_content = self._load_guide(guide)
            if guide_content and error_num:
                resolution.suggested_fix = f"See {guide} error #{error_num}"

        # Generate search queries for external fallback
        resolution.search_queries = self._generate_search_queries(error_output)

        return resolution

    def _classify_error(self, error_output: str) -> Tuple[Optional[str], Optional[int]]:
        """Match error to troubleshooting guide."""
        error_lower = error_output.lower()

        for pattern, (guide, error_num) in self.ERROR_PATTERNS.items():
            if re.search(pattern, error_lower):
                return (guide, error_num)

        return (None, None)

    def _load_guide(self, guide_name: str) -> Optional[str]:
        """Load troubleshooting guide content."""
        guide_path = self.troubleshooting_path / guide_name
        if guide_path.exists():
            return guide_path.read_text()
        return None

    def _generate_search_queries(self, error_output: str) -> List[str]:
        """Generate search queries for external tools."""
        # Extract first 50 chars of error for query
        error_snippet = error_output[:50].replace('\n', ' ').strip()

        return [
            f"PlantUML error: {error_snippet}",
            f"How to fix PlantUML {error_snippet}",
            f"PlantUML {error_snippet} solution stackoverflow"
        ]


class ValidationEngine:
    """Validates diagram generation and manages markdown integration."""

    def __init__(self, diagrams_dir: Path):
        self.diagrams_dir = diagrams_dir

    def verify_image_exists(self, filename: str, format: str = 'png') -> bool:
        """Check if generated image file exists."""
        image_path = self.diagrams_dir / f"{filename}.{format}"
        return image_path.exists() and image_path.stat().st_size > 0

    def generate_markdown_link(
        self,
        filename: str,
        format: str = 'png',
        title: Optional[str] = None
    ) -> str:
        """Generate markdown image link."""
        alt_text = title or filename.replace('_', ' ').title()
        return f"![{alt_text}](diagrams/{filename}.{format})"


class ResilientProcessor:
    """Main orchestrator implementing the 4-step resilient workflow."""

    def __init__(
        self,
        base_dir: Path = None,
        max_retries: int = 3,
        format: str = 'png',
        verbose: bool = False
    ):
        self.base_dir = base_dir or Path('.')
        self.max_retries = max_retries
        self.format = format
        self.verbose = verbose

        # Initialize components
        self.type_identifier = DiagramTypeIdentifier()
        self.naming = FileNamingConvention(self.base_dir)
        self.error_handler = ErrorHandler()
        self.validator = ValidationEngine(self.naming.diagrams_dir)

        # Error log
        self.error_log = []

    def _log(self, message: str):
        """Log message if verbose mode."""
        if self.verbose:
            print(f"[{datetime.now().strftime('%H:%M:%S')}] {message}")

    def process_diagram(
        self,
        puml_content: str,
        markdown_file: str = None,
        diagram_num: int = 1,
        title: Optional[str] = None
    ) -> ProcessingResult:
        """Process a single PlantUML diagram through the 4-step workflow."""
        result = ProcessingResult()

        # Step 1: Identify diagram type
        diagram_type = self.type_identifier.identify_from_content(puml_content)
        result.diagram_type = diagram_type
        result.reference_loaded = str(self.type_identifier.get_reference_path(diagram_type))
        self._log(f"Step 1: Identified diagram type: {diagram_type}")

        # Step 2: Create file with structured naming
        self.naming.ensure_directory()
        filename = self.naming.generate_filename(
            markdown_file or 'standalone',
            diagram_num,
            diagram_type,
            title
        )
        puml_path = self.naming.get_full_path(filename, 'puml')
        result.puml_path = puml_path

        # Write .puml file
        puml_path.write_text(puml_content)
        self._log(f"Step 2: Created {puml_path}")

        # Step 3: Convert with error handling
        success = False
        current_content = puml_content

        for retry in range(self.max_retries):
            success, error = self._convert(puml_path)

            if success:
                self._log(f"Step 3: Conversion successful (attempt {retry + 1})")
                break
            else:
                self._log(f"Step 3: Conversion failed (attempt {retry + 1}): {error}")
                resolution = self.error_handler.handle_error(
                    error,
                    current_content,
                    diagram_type
                )
                result.errors.append(resolution)

                # Log error
                self._log_error(filename, error, resolution)

                if not resolution.resolved:
                    result.external_search_needed = True
                    result.search_queries = resolution.search_queries

        result.conversion_success = success

        # Step 4: Validate and create markdown link
        if success:
            if self.validator.verify_image_exists(filename, self.format):
                result.image_path = self.naming.get_full_path(filename, self.format)
                result.markdown_link = self.validator.generate_markdown_link(
                    filename,
                    self.format,
                    title
                )
                self._log(f"Step 4: Validated image, link: {result.markdown_link}")
            else:
                result.validation_error = "Image file not created despite successful conversion"
                self._log(f"Step 4: FAILED - {result.validation_error}")

        return result

    def process_markdown(self, markdown_path: Path) -> Tuple[List[ProcessingResult], str]:
        """Process all PlantUML diagrams in a markdown file."""
        content = markdown_path.read_text()
        markdown_name = markdown_path.stem

        # Update naming base directory to markdown file's directory
        self.naming = FileNamingConvention(markdown_path.parent)
        self.validator = ValidationEngine(self.naming.diagrams_dir)

        # Extract puml blocks
        blocks = self._extract_puml_blocks(content)
        results = []
        updated_content = content

        self._log(f"Found {len(blocks)} PlantUML block(s) in {markdown_path}")

        for i, (block_content, full_match) in enumerate(blocks, 1):
            # Extract title from first line if comment
            title = self._extract_title(block_content)

            result = self.process_diagram(
                block_content,
                markdown_file=markdown_name,
                diagram_num=i,
                title=title
            )
            results.append(result)

            # Replace block with image link if successful
            if result.conversion_success and result.markdown_link:
                updated_content = updated_content.replace(
                    full_match,
                    result.markdown_link
                )

        return results, updated_content

    def _convert(self, puml_path: Path) -> Tuple[bool, str]:
        """Execute PlantUML conversion."""
        # Import convert_puml function
        try:
            from convert_puml import convert_puml, find_plantuml_jar
        except ImportError:
            # Fallback: call convert_puml.py directly
            return self._convert_subprocess(puml_path)

        try:
            success = convert_puml(
                str(puml_path),
                self.format,
                str(self.naming.diagrams_dir)
            )
            return (success, '' if success else 'Conversion failed')
        except Exception as e:
            return (False, str(e))

    def _convert_subprocess(self, puml_path: Path) -> Tuple[bool, str]:
        """Fallback conversion using subprocess."""
        script_path = SCRIPT_DIR / 'convert_puml.py'
        cmd = [
            sys.executable, str(script_path),
            str(puml_path),
            '--format', self.format,
            '--output-dir', str(self.naming.diagrams_dir)
        ]

        result = subprocess.run(cmd, capture_output=True, text=True)

        if result.returncode == 0:
            return (True, '')
        else:
            return (False, result.stderr or result.stdout or 'Unknown error')

    def _extract_puml_blocks(self, content: str) -> List[Tuple[str, str]]:
        """Extract PlantUML code blocks from markdown."""
        pattern = r'```(?:puml|plantuml)\n(.*?)```'
        matches = re.findall(pattern, content, re.DOTALL)

        # Return both content and full match for replacement
        full_pattern = r'```(?:puml|plantuml)\n.*?```'
        full_matches = re.findall(full_pattern, content, re.DOTALL)

        return list(zip(matches, full_matches))

    def _extract_title(self, puml_content: str) -> Optional[str]:
        """Extract title from PlantUML content."""
        # Check for title directive
        title_match = re.search(r'title\s+(.+)', puml_content)
        if title_match:
            return title_match.group(1).strip()

        # Check for first comment
        comment_match = re.search(r"'\s*(.+)", puml_content)
        if comment_match:
            return comment_match.group(1).strip()

        return None

    def _log_error(self, filename: str, error: str, resolution: ErrorResolution):
        """Log error to error log."""
        entry = {
            'timestamp': datetime.now().isoformat(),
            'file': filename,
            'error': error[:200],
            'guide_consulted': resolution.guide_loaded,
            'resolved': resolution.resolved
        }
        self.error_log.append(entry)

    def save_error_log(self):
        """Save error log to JSON file."""
        if self.error_log:
            log_path = self.naming.diagrams_dir / 'error_log.json'
            with open(log_path, 'w') as f:
                json.dump({'entries': self.error_log}, f, indent=2)


def main():
    """CLI entry point."""
    parser = argparse.ArgumentParser(
        description='Resilient PlantUML processor with 4-step workflow'
    )
    parser.add_argument('input', help='Input file (.puml or .md)')
    parser.add_argument('--format', choices=['png', 'svg'], default='png',
                        help='Output format (default: png)')
    parser.add_argument('--output-dir', default=None,
                        help='Output directory (default: ./diagrams/)')
    parser.add_argument('--max-retries', type=int, default=3,
                        help='Maximum retry attempts (default: 3)')
    parser.add_argument('--validate-only', action='store_true',
                        help='Only validate syntax without converting')
    parser.add_argument('--verbose', '-v', action='store_true',
                        help='Verbose output')

    args = parser.parse_args()

    input_path = Path(args.input)

    if not input_path.exists():
        print(f"ERROR: File not found: {input_path}")
        sys.exit(1)

    # Determine base directory
    base_dir = Path(args.output_dir) if args.output_dir else input_path.parent

    processor = ResilientProcessor(
        base_dir=base_dir,
        max_retries=args.max_retries,
        format=args.format,
        verbose=args.verbose
    )

    if input_path.suffix == '.puml':
        # Process single diagram
        content = input_path.read_text()
        result = processor.process_diagram(
            content,
            markdown_file=input_path.stem,
            diagram_num=1
        )

        print(f"\n{'='*50}")
        print(f"Diagram Type: {result.diagram_type}")
        print(f"Source: {result.puml_path}")
        print(f"Success: {result.conversion_success}")

        if result.conversion_success:
            print(f"Image: {result.image_path}")
            print(f"Markdown: {result.markdown_link}")
        else:
            print(f"Errors: {len(result.errors)}")
            if result.external_search_needed:
                print(f"Search queries: {result.search_queries}")

    elif input_path.suffix == '.md':
        # Process markdown file
        results, updated_content = processor.process_markdown(input_path)

        print(f"\n{'='*50}")
        print(f"Processed {len(results)} diagram(s)")

        success_count = sum(1 for r in results if r.conversion_success)
        print(f"Successful: {success_count}/{len(results)}")

        for i, result in enumerate(results, 1):
            status = "OK" if result.conversion_success else "FAILED"
            print(f"  {i}. [{status}] {result.diagram_type}: {result.puml_path}")

        if success_count > 0:
            # Save updated markdown
            output_path = input_path.with_name(f"{input_path.stem}_with_images.md")
            output_path.write_text(updated_content)
            print(f"\nUpdated markdown saved to: {output_path}")

    else:
        print(f"ERROR: Unsupported file type: {input_path.suffix}")
        sys.exit(1)

    # Save error log if any errors occurred
    processor.save_error_log()

    # Exit with appropriate code
    if input_path.suffix == '.puml':
        sys.exit(0 if result.conversion_success else 1)
    else:
        sys.exit(0 if success_count == len(results) else 1)


if __name__ == '__main__':
    main()
