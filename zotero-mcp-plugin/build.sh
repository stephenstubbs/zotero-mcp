#!/usr/bin/env bash

# Build the MCP Zotero API plugin XPI

set -e

PLUGIN_NAME="mcp-zotero-api"
VERSION="1.0.0"

cd "$(dirname "$0")"

# Remove old XPI if exists
rm -f "${PLUGIN_NAME}.xpi"

# Create XPI (which is just a ZIP file)
zip -r "${PLUGIN_NAME}.xpi" \
    manifest.json \
    bootstrap.js \
    icon.svg

echo "Built ${PLUGIN_NAME}.xpi (v${VERSION})"
echo ""
echo "To install:"
echo "  1. Open Zotero"
echo "  2. Go to Tools → Add-ons"
echo "  3. Click the gear icon → Install Add-on From File..."
echo "  4. Select ${PLUGIN_NAME}.xpi"
echo "  5. Restart Zotero"
